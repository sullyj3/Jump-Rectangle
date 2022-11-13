mod guy;
mod input;
mod level;
mod physics_object;
mod platformer;
mod state_transitions;

use bevy::log::LogPlugin;
use bevy::utils::Duration;
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_prototype_debug_lines::*;
use input::{
    game_input_system, global_input_system, make_global_input_map, GameAction,
    GlobalAction,
};
use iyes_loopless::{fixedtimestep::FixedTimestepStageLabel, prelude::*};
use leafwing_input_manager::prelude::*;
use platformer::{
    draw_aabbs, guy_collision_system, move_camera, physics_system, setup,
    update_jump_state, AppState, PHYSICS_TIME_STEP, TIME_STEP,
};
use state_transitions::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 640.0,
                    height: 360.0,
                    // width: 960.0,
                    // height: 540.0,
                    ..default()
                },
                ..default()
            }
            )
            .set(LogPlugin {
                level: bevy::log::Level::DEBUG,
                ..default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(InputManagerPlugin::<GlobalAction>::default())
        .add_plugin(InputManagerPlugin::<GameAction>::default())
        .add_plugin(DebugLinesPlugin::default())
        .insert_resource(ActionState::<GlobalAction>::default())
        .insert_resource(make_global_input_map())
        .insert_resource(ClearColor(Color::rgb(0.7, 0.8, 0.9)))
        .add_fixed_timestep(Duration::from_secs_f32(TIME_STEP), "input_timestep")
        .add_fixed_timestep_system(
            "input_timestep",
            0,
            global_input_system.label("global_input"),
        )
        .add_fixed_timestep_system(
            "input_timestep",
            0,
            game_input_system
                .run_in_state(AppState::InGame)
                .label("game_input"),
        )
        .add_fixed_timestep_after_stage(
            FixedTimestepStageLabel("input_timestep"),
            Duration::from_secs_f32(PHYSICS_TIME_STEP),
            "physics_timestep",
        )
        .add_fixed_timestep_system(
            "physics_timestep",
            0,
            physics_system
                .run_in_state(AppState::InGame)
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
            "input_timestep",
            0,
            update_jump_state
                .run_in_state(AppState::InGame)
                // .after("guy_collision")
                .label("update_jump_state"),
        )
        .add_system(
            move_camera
                .run_in_state(AppState::InGame)
                .after("guy_collision"),
        )
        .add_startup_system(setup)
        .add_system(draw_aabbs)
        .add_system(bevy::window::close_on_esc)
        .add_loopless_state(AppState::MainMenu)
        .add_enter_system(AppState::Loading, enter_loading)
        .add_enter_system(AppState::Loading, despawn_level_contents)
        .add_system(wait_level_load.run_in_state(AppState::Loading))
        .add_exit_system(AppState::Loading, exit_loading)
        .add_enter_system(AppState::Paused, enter_paused)
        .add_exit_system(AppState::Paused, exit_paused)
        .run();
}
