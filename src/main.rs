use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision}
    // input::gamepad::*,
};

const TIME_STEP: f32 = 1.0 / 60.0;
const PHYSICS_TIME_STEP: f32 = 1.0 / 120.0;
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

#[derive(Component)]
struct Guy {
    h_speed: f32,
}


#[derive(Component)]
struct PhysicsObject {
    velocity: Vec2,
    old_position: Vec3,
    on_ground: Option<f32>, // the y coordinate if guy is on ground, else None
}

struct Level(Vec<Transform>);

fn make_level_1() -> Level {
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    Level(vec![
        // left
        Transform {
            translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
            scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
            ..Default::default()
        },
        // right
        Transform {
            translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
            scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
            ..Default::default()
        },
        // bottom
        Transform {
            translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
            scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
            ..Default::default()
        },
        // top
        Transform {
            translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
            scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
            ..Default::default()
        },
        // platforms
        Transform {
            translation: Vec3::new(-280.0, -220.0, 0.0),
            scale: Vec3::new(50.0, wall_thickness, 1.0),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(-200.0, -200.0, 0.0),
            scale: Vec3::new(50.0, wall_thickness, 1.0),
            ..Default::default()
        },
        Transform {
            translation: Vec3::new(-120.0, -180.0, 0.0),
            scale: Vec3::new(50.0, wall_thickness, 1.0),
            ..Default::default()
        },
    ])
}

#[derive(Component)]
struct Wall;

struct MyGamepad(Gamepad);

fn gamepad_connections(
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

fn add_level_walls(commands: &mut Commands, Level(level): &Level) {
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    for transform in level {
        commands
            .spawn_bundle(SpriteBundle {
                transform: transform.clone(),
                sprite: Sprite {
                    color: wall_color,
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Wall);
    }
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

}


fn spawn_level(commands: &mut Commands) {
    info!("spawning level");

    let guy_size: Vec3 = Vec3::new(20.0, 50.0, 0.0);

    // guy
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-300.0, -250.0, 0.0),
                scale: guy_size,
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Guy { h_speed: 300.})
        .insert(PhysicsObject {
            velocity: Vec2::ZERO,
            on_ground: None,
            old_position: Vec3::ZERO,
        });

    let level1 = make_level_1();
    add_level_walls(commands, &level1);
}

fn input_system(
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(&Guy, &mut PhysicsObject)>,
    state: Res<AppState>,
    mut commands: Commands,
) {
    let gamepad = match my_gamepad {
        Some(gp) => gp.0,
        None => return,
    };

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
    let jump = GamepadButton(gamepad, GamepadButtonType::East);
    if buttons.just_pressed(jump)  {
        if let Some(_) = physics.on_ground {
            physics.velocity.y = 750.0;
            physics.on_ground = None;
        }
    }
}

fn physics_system( 
    mut query: Query<(Entity, &mut PhysicsObject, &mut Transform)>,
    state: Res<AppState>,
    ) {
    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    for (_entity, mut physics, mut transform) in query.iter_mut() {
        // apply gravity
        physics.velocity.y -= 23.0;

        // move
        let delta = physics.velocity * PHYSICS_TIME_STEP;
        let translation: &mut Vec3 = &mut transform.translation;
        physics.old_position = *translation;
        *translation += delta.extend(0.0);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn guy_collision_system(
    mut guy_query: Query<(&mut PhysicsObject, &mut Transform), (With<Guy>, Without<Wall>)>,
    wall_query: Query<&Transform, (With<Wall>, Without<Guy>)>,
    state: Res<AppState>,
) {
    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }

    let (mut guy_physics, mut guy_transform) = guy_query.single_mut();

    let guy_size = guy_transform.scale.truncate();
    guy_physics.on_ground = None;

    for wall_transform in wall_query.iter() {
        let wall_size = wall_transform.scale.truncate();
        let collision = collide(
            wall_transform.translation,
            wall_size,
            guy_transform.translation,
            guy_size,
        );
        match collision {
            Some(Collision::Left) => {
                guy_physics.velocity.x = guy_physics.velocity.x.min(0.0);
                guy_transform.translation.x =
                    wall_transform.translation.x + (wall_size.x / 2.) + (guy_size.x / 2.);
            },
            Some(Collision::Right) => {
                guy_physics.velocity.x = guy_physics.velocity.x.max(0.0);
                guy_transform.translation.x =
                    wall_transform.translation.x - (wall_size.x / 2.) - (guy_size.x / 2.);
            },
            Some(Collision::Top) => {
                guy_physics.velocity.y = guy_physics.velocity.y.min(0.0);
                guy_transform.translation.y = 
                    wall_transform.translation.y - (wall_size.y / 2.) - (guy_size.y / 2.);
            },
            Some(Collision::Bottom) => {
                guy_physics.velocity.y = guy_physics.velocity.y.max(0.0);
                guy_transform.translation.y = 
                    wall_transform.translation.y + (wall_size.y / 2.) + (guy_size.y / 2.);
                guy_physics.on_ground = Some(guy_transform.translation.y);
            },
            None => (),
        }
    }
}
