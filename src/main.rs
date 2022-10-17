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
    let input_stage = SystemStage::parallel()
        .with_system(
            input_system.label("input")
        );

    let physics_stage = SystemStage::parallel()
        .with_system_set(
            SystemSet::new()
                .with_system(physics_system.label("physics").after("input"))
                .with_system(
                    guy_collision_system.label("guy_collision").after("physics"),
                ),
        );

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())

        .insert_resource(ActionState::<Action>::default())
        .insert_resource(make_input_map())
        .insert_resource(AppState::MainMenu)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))

        .add_stage_before(
            CoreStage::Update,
            "input",
            FixedTimestepStage::new(Duration::from_secs_f32(TIME_STEP), "input")
                .with_stage(input_stage)
        )
        .add_stage_before(
            CoreStage::Update,
            "physics",
            FixedTimestepStage::new(Duration::from_secs_f32(PHYSICS_TIME_STEP), "physics")
                .with_stage(physics_stage)
        )

        .add_startup_system(setup)
        .add_system(move_camera.after("guy_collision"))
        .add_system(bevy::window::close_on_esc)
        .run();
}
