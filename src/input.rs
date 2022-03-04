use bevy::prelude::*;

use crate::platformer::{
    PhysicsObject,
    Guy, AppState, spawn_level,
};

pub struct MyGamepad(Gamepad);

pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
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
    let start = GamepadButton(gamepad, GamepadButtonType::Start);
    if buttons.just_pressed(start) {
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            },
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
            },
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
            },
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
    let dpad_x = axes
        .get(GamepadAxis(gamepad, GamepadAxisType::DPadX))
        .unwrap();
    let lstick_x = axes
        .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
        .unwrap();

    let direction_x = if dpad_x == 0.0 {
        lstick_x
    } else {
        dpad_x
    };
    physics.velocity.x = direction_x * guy.h_speed;

    // Jumping
    let jump1 = GamepadButton(gamepad, GamepadButtonType::East);
    let jump2 = GamepadButton(gamepad, GamepadButtonType::South);
    if buttons.any_pressed([jump1, jump2])  {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}

pub fn input_system(
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
    mut commands: Commands
) {
    if keyboard.just_pressed(KeyCode::Return) {
        match *state {
            AppState::MainMenu => {
                info!("starting game");
                spawn_level(&mut commands);
                commands.insert_resource(AppState::InGame);
            },
            AppState::InGame => {
                info!("Game paused");
                commands.insert_resource(AppState::Paused);
            },
            AppState::Paused => {
                info!("Game resumed");
                commands.insert_resource(AppState::InGame);
            },
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
    if keyboard.just_pressed(KeyCode::Space)  {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}
