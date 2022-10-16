use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::platformer::{spawn_level, AppState, Guy, PhysicsObject};

pub struct MyGamepad(Gamepad);

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Jump,
    Start,
}

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent {
        gamepad,
        event_type,
    } in gamepad_evr.iter()
    {
        match (event_type, my_gamepad.as_deref()) {
            (GamepadEventType::Connected, None) => {
                commands.insert_resource(MyGamepad(*gamepad));
            }
            (GamepadEventType::Disconnected, Some(MyGamepad(old_id)))
                if old_id == gamepad =>
            {
                commands.remove_resource::<MyGamepad>();
            }
            // other events are irrelevant
            _ => {}
        }
    }
}

// todo: try input_map.insert_multiple([ <pairs> ])
pub fn make_input_map() -> InputMap<Action> {
    let mut input_map = InputMap::default();
    // keyboard
    input_map.insert(KeyCode::Space, Action::Jump);
    input_map.insert(KeyCode::Return, Action::Start);

    input_map.insert(GamepadButtonType::South, Action::Jump);
    input_map.insert(GamepadButtonType::Start, Action::Start);
    input_map
}

// todo separate functions for gamepad and keyboard is wack, need to refactor to translate both
// into some sort of common data type
fn gamepad_input(
    gamepad: Gamepad,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(&Guy, &mut PhysicsObject)>,
    state: Res<AppState>,
    mut commands: Commands,
) {
    let start = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::Start,
    };
    if buttons.just_pressed(start) {
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            }
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
            }
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
            }
        };
        return;
    }

    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    let (guy, mut physics) = query.single_mut();

    // Movement

    // let dpad_up = GamepadButton{ gamepad, button_type: GamepadButtonType::DPadUp };
    // let dpad_down = GamepadButton{ gamepad, button_type: GamepadButtonType::DPadDown };
    let dpad_left = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let dpad_right = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    let dpad_x: f32 = (-1. * buttons.pressed(dpad_left) as i32 as f32)
        + buttons.pressed(dpad_right) as i32 as f32;

    let lstick_x = axes
        .get(GamepadAxis {
            gamepad,
            axis_type: GamepadAxisType::LeftStickX,
        })
        .unwrap();

    let direction_x = if dpad_x == 0.0 { lstick_x } else { dpad_x };

    physics.velocity.x = direction_x * guy.h_speed;

    // Jumping
    let jump1 = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::East,
    };
    let jump2 = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::South,
    };
    if buttons.any_just_pressed([jump1, jump2]) {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}

pub fn input_system(
    action_state: Res<ActionState<Action>>,
    guy: Query<&Guy>,
    state: Res<AppState>,
    mut commands: Commands,
) {
    if action_state.just_pressed(Action::Start) {
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            }
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
            }
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
            }
        };
        return;
    }
}

pub fn old_input_system(
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    keyboard: Res<Input<KeyCode>>,
    query: Query<(&Guy, &mut PhysicsObject)>,
    state: Res<AppState>,
    commands: Commands,
) {
    match my_gamepad {
        Some(gp) => gamepad_input(gp.0, axes, buttons, query, state, commands),
        None => keyboard_input(keyboard, query, state, commands),
    }
}

fn keyboard_input(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&Guy, &mut PhysicsObject)>,
    state: Res<AppState>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Return) {
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            }
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
            }
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
            }
        };
        return;
    }

    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    let (guy, mut physics) = query.single_mut();

    // Movement
    // let dpad_x = axes
    //     .get(GamepadAxis(gamepad, GamepadAxisType::DPadX))
    //     .unwrap();
    // let lstick_x = axes
    //     .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
    //     .unwrap();

    // let direction_x = if dpad_x == 0.0 {
    //     lstick_x
    // } else {
    //     dpad_x
    // };

    // todo improve this logic, currently right just overrides left. should instead be last
    // pressed.
    let mut direction_x = 0.0;
    if keyboard.pressed(KeyCode::Left) {
        direction_x = -1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        direction_x = 1.0;
    }

    physics.velocity.x = direction_x * guy.h_speed;

    // Jumping
    if keyboard.just_pressed(KeyCode::Space) {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}
