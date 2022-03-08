mod platformer;
mod input;

use bevy::{
    prelude::*,
};
use platformer::{
    AppState,
    setup,
    physics_system,
    guy_collision_system,
    move_camera, AverageDt, TimeToProcess, PrintAvgDtCountdown, print_avg_dt,
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
        .insert_resource(AverageDt(None))
        .insert_resource(TimeToProcess(0.))
        .insert_resource(PrintAvgDtCountdown(Timer::from_seconds(1., true)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_system(input_system.label("input"))
        )
        .add_system_set(
            SystemSet::new()
                .with_system(physics_system.label("physics").after("input"))
                .with_system(guy_collision_system.label("guy_collision").after("physics"))
                .with_system(print_avg_dt)
        )
        .add_system(gamepad_connections)
        .add_system(move_camera.after("guy_collision"))
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

