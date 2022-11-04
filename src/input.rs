#![allow(clippy::type_complexity)]

use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::guy::*;
use crate::physics_object::{Gravity, PhysicsObject};
use crate::platformer::AppState;
use crate::state_transitions::LoadingLevel;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Action {
    Move,
    Jump,
    Start,
    Select,
    Debug,
}

pub fn make_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();
    // keyboard
    input_map.insert_multiple([
        (KeyCode::Grave, Action::Debug),
        (KeyCode::Space, Action::Jump),
        (KeyCode::Return, Action::Start),
    ]);
    input_map.insert(VirtualDPad::arrow_keys(), Action::Move);

    // gamepad
    input_map.insert_multiple([
        // For debugging
        (GamepadButtonType::North, Action::Debug),
        (GamepadButtonType::South, Action::Jump),
        (GamepadButtonType::Start, Action::Start),
        (GamepadButtonType::Select, Action::Select),
    ]);
    input_map.insert(VirtualDPad::dpad(), Action::Move);
    input_map.insert(DualAxis::left_stick(), Action::Move);
    input_map
}

pub fn input_system(
    action_state: Res<ActionState<Action>>,
    mut query: Query<(
        Entity,
        &Guy,
        &mut PhysicsObject,
        &mut Transform,
        &mut JumpState,
        Option<&CanFly>,
    )>,
    mut commands: Commands,
    state: Res<CurrentState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    // Quit if start+select
    if action_state.pressed(Action::Select) && action_state.pressed(Action::Start) {
        exit.send(AppExit);
    }

    // Start button state transitions
    if action_state.just_pressed(Action::Start) {
        match state.0 {
            AppState::Loading => (),
            AppState::MainMenu => {
                commands.insert_resource(LoadingLevel("assets/level1.png".into()));
                commands.insert_resource(NextState(AppState::Loading))
            }
            AppState::InGame => {
                commands.insert_resource(NextState(AppState::Paused))
            }
            AppState::Paused => {
                commands.insert_resource(NextState(AppState::InGame))
            }
        };
        return;
    }

    // Exit if we're not in game, and therefore shouldn't handle in game input
    // TODO: split into 2 systems, one for the character and
    // one for the whole game. this will allow us to conditionally run ingame input system only
    // during AppState::InGame, eliminating this check
    match state.0 {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::Loading => return,
        AppState::InGame => (),
    }

    let (guy_entity, guy, mut physics, mut transform, mut jump_state, can_fly) =
        query.single_mut();

    // TODO it might also be good to have separate systems for eg movement and jumping. Is
    // this idiomatic bevy? need to research

    // Movement
    if can_fly.is_some() {
        let direction = action_state
            .clamped_axis_pair(Action::Move)
            .map_or(Vec2::ZERO, |axis_data| axis_data.xy());
        physics.velocity = direction * guy.h_speed;
    } else {
        let direction_x = action_state
            .clamped_axis_pair(Action::Move)
            .map_or(0., |axis_data| axis_data.x());
        physics.velocity.x = direction_x * guy.h_speed;
    }

    // debug things here
    if action_state.just_pressed(Action::Debug) {
        // toggle flying
        if can_fly.is_some() {
            commands.entity(guy_entity).remove::<CanFly>();
            commands.entity(guy_entity).insert(Gravity);
        } else {
            commands.entity(guy_entity).insert(CanFly);
            commands.entity(guy_entity).remove::<Gravity>();
        }
    }

    if action_state.just_pressed(Action::Jump) {
        jump_state.try_jump(&mut physics, &mut transform);
    }
}
