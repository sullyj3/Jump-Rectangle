use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    // input::keyboard::KeyboardInput,
    // input::gamepad::*,
};

pub const TIME_STEP: f32 = 1. / 60.0;
pub const PHYSICS_TIME_STEP: f32 = 1.0 / 120.0;

#[derive(Component)]
pub struct Guy {
    pub h_speed: f32,
}

#[derive(Bundle)]
struct GuyBundle {
    guy: Guy,
    #[bundle]
    sprite: SpriteBundle,
    physics: PhysicsObject,
}

impl Default for GuyBundle {
    fn default() -> Self {
        const GUY_SIZE: Vec3 = Vec3::new(20.0, 50.0, 0.0);
        GuyBundle {
            guy: Guy { h_speed: 300. },
            sprite: SpriteBundle {
                transform: Transform {
                    scale: GUY_SIZE,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            physics: PhysicsObject::default() 
        }
    }
}

impl GuyBundle {
    fn with_translation(translation: Vec3) -> Self {
        let mut guy = GuyBundle::default();
        guy.sprite.transform.translation = translation;
        guy
    }
}

#[derive(Component, Default)]
pub struct PhysicsObject {
    pub velocity: Vec2,
    pub old_position: Vec3,
    pub on_ground: Option<f32>, // the y coordinate if guy is on ground, else None
}

pub struct Level(Vec<Transform>);

pub fn make_level_1() -> Level {
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
pub struct Wall;

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

#[derive(Component, PartialEq, Eq)]
pub struct PauseMessage;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(Camera2dBundle::default());
    // Pause message
    commands
        .spawn_bundle(
            TextBundle::from_section(
                "PAUSED",
                TextStyle {
                    font: asset_server
                        .load("fonts/AL Ubuntu Mono Nerd Font Complete.ttf"),
                    font_size: 100.0,
                    color: Color::BLACK,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(default()),
        )
        .insert(Visibility { is_visible: false })
        .insert(PauseMessage);
}

pub fn spawn_level(commands: &mut Commands) {
    info!("spawning level");

    // guy
    commands.spawn_bundle(GuyBundle::with_translation(Vec3::new(-300.0, -250.0, 0.0)));

    let level1 = make_level_1();
    add_level_walls(commands, &level1);
}

pub fn physics_system(
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
pub enum AppState {
    MainMenu,
    InGame,
    Paused,
}

pub fn guy_collision_system(
    mut guy_query: Query<
        (&mut PhysicsObject, &mut Transform),
        (With<Guy>, Without<Wall>),
    >,
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
                guy_transform.translation.x = wall_transform.translation.x
                    + (wall_size.x / 2.)
                    + (guy_size.x / 2.);
            }
            Some(Collision::Right) => {
                guy_physics.velocity.x = guy_physics.velocity.x.max(0.0);
                guy_transform.translation.x = wall_transform.translation.x
                    - (wall_size.x / 2.)
                    - (guy_size.x / 2.);
            }
            Some(Collision::Top) => {
                guy_physics.velocity.y = guy_physics.velocity.y.min(0.0);
                guy_transform.translation.y = wall_transform.translation.y
                    - (wall_size.y / 2.)
                    - (guy_size.y / 2.);
            }
            Some(Collision::Bottom) => {
                guy_physics.velocity.y = guy_physics.velocity.y.max(0.0);
                guy_transform.translation.y = wall_transform.translation.y
                    + (wall_size.y / 2.)
                    + (guy_size.y / 2.);
                guy_physics.on_ground = Some(guy_transform.translation.y);
            }
            Some(Collision::Inside) => {
                // Not sure what to do here
            }
            None => (),
        }
    }
}

pub fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Guy>)>,
    player: Query<&Transform, (With<Guy>, Without<Camera>)>,
    state: Res<AppState>,
) {
    match *state {
        AppState::MainMenu => return,
        AppState::Paused => return,
        AppState::InGame => (),
    }
    let guy_pos: Vec3 = player.single().translation;

    for mut transform in camera.iter_mut() {
        transform.translation = guy_pos;
    }
}
