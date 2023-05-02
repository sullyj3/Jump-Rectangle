#![allow(clippy::type_complexity)]

use bevy::app::AppExit;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::guy::*;
use crate::level::LoadingLevel;
use crate::physics_object::{Gravity, PhysicsObject};
use crate::platformer::AppState;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GlobalAction {
    Start,
    Select,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum GameAction {
    Move,
    Jump,
    Debug,
}

pub fn make_global_input_map() -> InputMap<GlobalAction> {
    let mut input_map = InputMap::default();
    // keyboard
    input_map.insert_multiple([(KeyCode::Return, GlobalAction::Start)]);

    // gamepad
    input_map.insert_multiple([
        (GamepadButtonType::Start, GlobalAction::Start),
        (GamepadButtonType::Select, GlobalAction::Select),
    ]);
    input_map
}

pub fn make_game_input_map() -> InputMap<GameAction> {
    let mut input_map = InputMap::default();
    // keyboard
    input_map.insert_multiple([
        (KeyCode::Grave, GameAction::Debug),
        (KeyCode::Space, GameAction::Jump),
    ]);
    input_map.insert(VirtualDPad::arrow_keys(), GameAction::Move);

    // gamepad
    input_map.insert_multiple([
        (GamepadButtonType::North, GameAction::Debug),
        (GamepadButtonType::South, GameAction::Jump),
    ]);
    input_map.insert(VirtualDPad::dpad(), GameAction::Move);
    input_map.insert(DualAxis::left_stick(), GameAction::Move);
    input_map
}

pub fn global_input_system(
    global_action_state: Res<ActionState<GlobalAction>>,
    mut commands: Commands,
    state: Res<CurrentState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    // Quit if start+select
    if global_action_state.pressed(GlobalAction::Select)
        && global_action_state.pressed(GlobalAction::Start)
    {
        exit.send(AppExit);
    }

    // Start button state transitions
    if global_action_state.just_pressed(GlobalAction::Start) {
        use AppState::*;
        match state.0 {
            Loading => (),
            MainMenu => {
                // I don't really love this approach of needing to insert a LoadingLevel
                // when I switch to the loading state
                // not sure if there's a better way to communicate between states.
                commands.insert_resource(LoadingLevel::Overworld);
                commands.insert_resource(NextState(AppState::Loading))
            }
            InGame => commands.insert_resource(NextState(AppState::Paused)),
            Paused => commands.insert_resource(NextState(AppState::InGame)),
        };
    }
}

pub fn game_input_system(
    mut query: Query<(
        Entity,
        &ActionState<GameAction>,
        &Guy,
        &mut PhysicsObject,
        &mut Transform,
        &mut JumpState,
        Option<&CanFly>,
    )>,
    mut commands: Commands,
) {
    let (
        guy_entity,
        action_state,
        guy,
        mut physics,
        mut transform,
        mut jump_state,
        can_fly,
    ) = query.single_mut();

    // TODO it might also be good to have separate systems for eg movement and jumping. Is
    // this idiomatic bevy? need to research

    // Movement
    if can_fly.is_some() {
        let direction = action_state
            .clamped_axis_pair(GameAction::Move)
            .map_or(Vec2::ZERO, |axis_data| axis_data.xy());
        physics.velocity = direction * guy.h_speed;
    } else {
        let direction_x = action_state
            .clamped_axis_pair(GameAction::Move)
            .map_or(0., |axis_data| axis_data.x());
        physics.velocity.x = direction_x * guy.h_speed;
    }

    // debug things here
    if action_state.just_pressed(GameAction::Debug) {
        // toggle flying
        if can_fly.is_some() {
            commands.entity(guy_entity).remove::<CanFly>();
            commands.entity(guy_entity).insert(Gravity);
        } else {
            commands.entity(guy_entity).insert(CanFly);
            commands.entity(guy_entity).remove::<Gravity>();
        }
    }

    if action_state.just_pressed(GameAction::Jump) {
        jump_state.try_jump(&mut physics, &mut transform);
    }
}
