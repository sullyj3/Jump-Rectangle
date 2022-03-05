# Jump-Rectangle

A platformer for the purpose of learning the Bevy game engine

## Current state
I'm currently unsatisfied with using both `FixedTimestep` and `Res<Time>`.
  
### `FixedTimestep`
`FixedTimestep` is something called a *run criteria* - a piece of metadata that is attached to a Bevy ECS *system* that determines when that system runs, or, in other words, whether it runs on a given iteration of the main loop. This is great in theory. 

The issue is that I also want to be able to use a bevy feature called `State`. Bevy states are also ways of determining whether a system should run. For example, if you are in a `Paused` state, a physics system should not run. The problem is that Bevy States are implemented in terms of run criteria. This means if you only want a system to run in a given state, but also want it to run at a `FixedTimestep`, the latter directive clobbers the former, and it just runs at the fixed timestep with no regard to that current state. This is annoying.

An alternative to `State` is to instead create a bevy resource that will hold the current state. I can't tell bevy to decide whether to run a system based on the resource, but I can have each system access the resource, and decide internally to return immediately if we're in the wrong state. 

This works, but it leads to extra boilerplate in the form of an additional `Res<AppState>` parameter to every system that I only want to run in certain states. Additionally, each such system needs an extra check at the top, something like

```rust
fn my system(
    // .. other params
    state: Res<AppState>,
) {
    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }
    // ...
}
```

This is gross and I hate it.

### `Res<Time>`

An alternative to using `FixedTimestep` to schedule systems is to use a bevy `Time` Resource. Each iteration, you ask the resource how much time has elapsed since the last iteration, and use that *delta* to determine how much time to simulate.
  
This is not quite a drop in replacement, since it changes the behavior - we've gone from *fixed time* to *delta time*. Delta time has advantages and disadvantages compared to fixed time, which have been well studied. Notably, delta time can lead to physics glitches - when the time delta gets too high due to slowdowns, the physics won't notice collisions of fast moving objects. For example, on one frame a bullet might be on the left of a wall, a large delta of time passes, and on the next frame it's on the right. At no point in time do they intersect, causing the bullet to clip through the wall.

This is an actual issue that I ran into in practise - as soon as I switched from `FixedTimestep` to `Res<Time>` my character started clipping through the floor when he fell from a medium height.
  
There are ways of getting around this issue. See [Fix your timestep](https://gafferongames.com/post/fix_your_timestep/) for details. To summarize, for a given delta time, you divide it up into smaller, fixed timesteps, and execute them in a loop until you've processed all the time that elapsed. That way you don't get any large 'timeskips', eliminating the clipping issue.
  
I attempted to implement this manually in bevy using Res<Time>, and immediately ran into an issue. If you want this strategy of dividing the delta into steps to work, each relevant system needs to be executed for every subdivision. That means that it's no longer possible to have a separate system for handling collisions - that needs to be moved into the physics system, inside the subdivisions loop. Otherwise the collisions system will never see the positions of the objects in their intermediate states!
  
This is clearly pretty unsustainable - ultimately every system that needs to know intermediate states needs to be folded in. One of the main benefits of ECS systems - composability - has been lost.
  
### Stageless scheduling and a States rework?
  
So currently I'm unhappy with all my options. However, delving into the depths of Bevy developer discussions on github and discourse, it looks as though the system scheduling system is going to get a rework at some point. Importantly, it will include a redesign of `State`s, making them no longer run criteria based, and therefore no longer conflicting with `FixedTimestep`. See the [relevant rfc](https://github.com/bevyengine/rfcs/pull/45)
  
So I guess I'll just wait until that drops.
  
### Custom hybrid run criteria? 
Mentioned by Alice on discord as something to look into. Not ideal imo, but may serve as a stopgap until the above RFC is accepted and implemented
