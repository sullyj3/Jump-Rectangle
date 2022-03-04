mod platformer;
mod input;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    // input::keyboard::KeyboardInput,
    // input::gamepad::*,
};
use platformer::{
    AppState,
    setup,
    physics_system,
    guy_collision_system,
    TIME_STEP,
    PHYSICS_TIME_STEP,
};
use input::{
    input_system,
    gamepad_connections,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AppState::MainMenu)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(input_system.label("input"))
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(PHYSICS_TIME_STEP as f64))
                .with_system(physics_system.label("physics").after("input"))
                .with_system(guy_collision_system.after("physics"))
        )
        .add_system(gamepad_connections)
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

