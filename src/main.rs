mod input;
mod platformer;

use bevy::{prelude::*, time::FixedTimestep};
use input::{gamepad_connections, input_system, Action, make_input_map};
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
        .insert_resource(AppState::MainMenu)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(input_system.label("input")),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP as f64))
                .with_system(physics_system.label("physics").after("input"))
                .with_system(
                    guy_collision_system.label("guy_collision").after("physics"),
                ),
        )
        .add_system(gamepad_connections)
        .add_system(move_camera.after("guy_collision"))
        .add_system(bevy::window::close_on_esc)
        .run();
}
