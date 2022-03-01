use bevy::{
    core::FixedTimestep,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    // input::gamepad::*,
};

const TIME_STEP: f32 = 1.0 / 120.0;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(input_system.label("input"))
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
    is_on_ground: bool,
    was_on_ground: bool,
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

fn add_walls(commands: &mut Commands) {
    // Add walls
    let wall_color = Color::rgb(0.8, 0.8, 0.8);
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);

    // left
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall);
    // right
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
                scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall);
    // bottom
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall);
    // top
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
                scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: wall_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Wall);
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // guy
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(20.0, 50.0, 0.0),
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
            is_on_ground: false,
            was_on_ground: false,
            old_position: Vec3::ZERO,
        });

    add_walls(&mut commands);
}

fn input_system(
    my_gamepad: Option<Res<MyGamepad>>,
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<(&Guy, &mut PhysicsObject)>,
) {
    let (guy, mut physics) = query.single_mut();

    let gamepad = match my_gamepad {
        Some(gp) => gp.0,
        None => return,
    };

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

    // jump
    let jump = GamepadButton(gamepad, GamepadButtonType::East);
    if buttons.just_pressed(jump) && physics.is_on_ground {
        physics.velocity.y = 750.0;
        physics.is_on_ground = false;
    }
}

fn physics_system( 
    mut query: Query<(Entity, &mut PhysicsObject, &mut Transform)>,
    ) {
    for (_entity, mut physics, mut transform) in query.iter_mut() {
        // apply gravity
        physics.velocity.y -= 23.0;

        let delta = physics.velocity * TIME_STEP;
        let translation: &mut Vec3 = &mut transform.translation;
        physics.old_position = *translation;
        *translation += Vec3::from((delta, 0.0));
    }
}

fn guy_collision_system(
    mut guy_query: Query<(&Guy, &mut PhysicsObject, &mut Transform), Without<Wall>>,
    wall_query: Query<(&Wall, &Transform), Without<Guy>>,
) {
    let (_, mut guy_physics, mut guy_transform) = guy_query.single_mut();
    let guy_size = guy_transform.scale.truncate();

    // check collision with walls
    for (_, wall_transform) in wall_query.iter() {
        let collision = collide(
            wall_transform.translation,
            wall_transform.scale.truncate(),
            guy_transform.translation,
            guy_size,
        );
        match collision {
            Some(Collision::Left) => {
                guy_physics.velocity.x = 0.0;
                guy_transform.translation.x = guy_physics.old_position.x;
            },
            Some(Collision::Right) => {
                guy_physics.velocity.x = 0.0;
                guy_transform.translation.x = guy_physics.old_position.x;
            },
            Some(Collision::Top) => {
                guy_physics.velocity.y = 0.0;
                guy_transform.translation.y = guy_physics.old_position.y;
            },
            Some(Collision::Bottom) => {
                guy_physics.velocity.y = 0.0;
                guy_transform.translation.y = guy_physics.old_position.y;
                guy_physics.is_on_ground = true;
            },
            None => (),
        }
    }
}
