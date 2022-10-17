use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::guy::*;
use crate::physics_object::PhysicsObject;
use crate::platformer::AppState;

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
    mut query: Query<(&Guy, &mut PhysicsObject, &mut Transform)>,
    mut commands: Commands,
    state: Res<CurrentState<AppState>>,
) {
    if action_state.just_pressed(Action::Start) {
        match state.0 {
            AppState::MainMenu => {
                commands.insert_resource(NextState(AppState::InGame))
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

    // TODO: split into 2 systems, one for the character and
    // one for the whole game. this will allow us to conditionally run ingame input system only
    // during AppState::InGame, eliminating this check
    match state.0 {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    let (guy, mut physics, mut transform) = query.single_mut();

    // Movement
    let direction_x = action_state
        .clamped_axis_pair(Action::Move)
        .map_or(0., |axis_data| axis_data.x());

    physics.velocity.x = direction_x * guy.h_speed;

    if action_state.just_pressed(Action::Debug) {
        // debug things here
        let axis_pair = action_state.clamped_axis_pair(Action::Move);
        println!("Move axis pair: {:?}", axis_pair);
    }

    if action_state.just_pressed(Action::Jump) {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            transform.scale = GUY_JUMPING_SIZE;
            physics.on_ground = None;
        }
    }
}
