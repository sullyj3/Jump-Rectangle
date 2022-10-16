use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::platformer::{spawn_level, AppState, Guy, PhysicsObject, PauseMessage};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Action {
    Move,
    Jump,
    Start,
    Debug,
}

pub fn make_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();
    // keyboard
    input_map.insert_multiple([
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
    ]);
    input_map.insert(VirtualDPad::dpad(), Action::Move);
    input_map.insert(DualAxis::left_stick(), Action::Move);
    input_map
}

pub fn input_system(
    action_state: Res<ActionState<Action>>,
    mut query: Query<(&Guy, &mut PhysicsObject)>,
    mut pause_message_vis: Query<&mut Visibility, With<PauseMessage>>,
    state: Res<AppState>,
    mut commands: Commands,
) {
    if action_state.just_pressed(Action::Start) {
        let mut pm_visibility = pause_message_vis.single_mut();
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            }
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
                pm_visibility.is_visible = true;
            }
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
                pm_visibility.is_visible = false;
            }
        };
        return;
    }

    // TODO: remove after iyes loopless allows me to use proper states
    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    let (guy, mut physics) = query.single_mut();

    // Movement
    let direction_x = action_state
        .clamped_axis_pair(Action::Move)
        .map_or(0., |axis_data| axis_data.x());

    physics.velocity.x = direction_x * guy.h_speed;

    if action_state.just_pressed(Action::Debug) {
        // debug things here
        let ap = action_state.clamped_axis_pair(Action::Move);
        println!("Move axis pair: {:?}", ap);
    }

    if action_state.just_pressed(Action::Jump) {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}
