mod input;
mod platformer;
mod guy;
mod physics_object;

use bevy::prelude::*;
use bevy::utils::Duration;
use iyes_loopless::prelude::*;
use input::{input_system, make_input_map, Action};
use leafwing_input_manager::prelude::*;
use platformer::{
    guy_collision_system, move_camera, physics_system, setup, AppState,
    PHYSICS_TIME_STEP, TIME_STEP,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())

        .insert_resource(ActionState::<Action>::default())
        .insert_resource(make_input_map())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))

        .add_fixed_timestep(
            Duration::from_secs_f32(TIME_STEP),
            "input_timestep",
        )
        // for now this needs to run in all states, to handle Start press
        // we should factor it into an ingame and out of game system
        // then we can add the ingame input handler as a component to guy
        .add_fixed_timestep_system("input_timestep", 0, input_system.label("input"))
        .add_fixed_timestep(
            Duration::from_secs_f32(PHYSICS_TIME_STEP),
            "physics_timestep",
        )
        .add_fixed_timestep_system(
            "physics_timestep",
            0,
            physics_system
                .run_in_state(AppState::InGame)
                .after("input")
                .label("physics")
        )

        .add_fixed_timestep_system(
            "physics_timestep",
            0,
            guy_collision_system
                .run_in_state(AppState::InGame)
                .label("guy_collision")
                .after("physics")
        )
        .add_system(
            move_camera
                .run_in_state(AppState::InGame)
                .after("guy_collision"))

        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_loopless_state(AppState::MainMenu)
        .run();
}
