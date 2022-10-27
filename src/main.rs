mod guy;
mod input;
mod physics_object;
mod platformer;
mod state_transitions;

use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::utils::Duration;
use input::{input_system, make_input_map, Action};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;
use platformer::{
    guy_collision_system, move_camera, physics_system, setup, update_jump_state,
    AppState, PHYSICS_TIME_STEP, TIME_STEP,
};
use state_transitions::*;

fn main() {
    App::new()
        .insert_resource(bevy::log::LogSettings {
            level: bevy::log::Level::DEBUG,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ActionState::<Action>::default())
        .insert_resource(make_input_map())
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_fixed_timestep(Duration::from_secs_f32(TIME_STEP), "input_timestep")
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
                .label("physics"),
        )
        .add_fixed_timestep_system(
            "physics_timestep",
            0,
            guy_collision_system
                .run_in_state(AppState::InGame)
                .label("guy_collision")
                .after("physics"),
        )
        .add_fixed_timestep_system(
            "physics_timestep",
            0,
            update_jump_state
                .run_in_state(AppState::InGame)
                .after("guy_collision")
                .label("update_jump_state"),
        )
        .add_system(
            move_camera
                .run_in_state(AppState::InGame)
                .after("guy_collision"),
        )
        .add_startup_system(setup)
        .add_system(bevy::window::close_on_esc)
        .add_loopless_state(AppState::MainMenu)
        .add_enter_system(AppState::Paused, enter_paused)
        .add_exit_system(AppState::Paused, exit_paused)
        .add_exit_system(AppState::MainMenu, exit_menu)
        .run();
}
