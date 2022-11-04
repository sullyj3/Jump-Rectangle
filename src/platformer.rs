#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        Rect,
    },
    // input::keyboard::KeyboardInput,
    // input::gamepad::*,
};
use bevy_prototype_debug_lines::*;
use iyes_loopless::state::NextState;
// use rand::prelude::*;

use crate::{
    guy::*,
    physics_object::{Gravity, PhysicsObject},
    portal::{Portal, PortalBundle},
};
use crate::{
    level::{Level, LevelContents, WALL_TILE_SIZE},
    state_transitions::LoadingLevel,
};

pub const TIME_STEP: f32 = 1. / 60.0;
pub const PHYSICS_TIME_STEP: f32 = 1.0 / 120.0;

// pub fn make_level_1() -> Level {
//     let wall_thickness = 10.0;
//     let bounds = Vec2::new(900.0, 600.0);

//     Level(vec![
//         // left
//         Transform {
//             translation: Vec3::new(-bounds.x / 2.0, 0.0, 0.0),
//             scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
//             ..Default::default()
//         },
//         // right
//         Transform {
//             translation: Vec3::new(bounds.x / 2.0, 0.0, 0.0),
//             scale: Vec3::new(wall_thickness, bounds.y + wall_thickness, 1.0),
//             ..Default::default()
//         },
//         // bottom
//         Transform {
//             translation: Vec3::new(0.0, -bounds.y / 2.0, 0.0),
//             scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         // top
//         Transform {
//             translation: Vec3::new(0.0, bounds.y / 2.0, 0.0),
//             scale: Vec3::new(bounds.x + wall_thickness, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         // platforms
//         // Transform {
//         //     translation: Vec3::new(-280.0, -220.0, 0.0),
//         //     scale: Vec3::new(50.0, wall_thickness, 1.0),
//         //     ..Default::default()
//         // },
//         // Transform {
//         //     translation: Vec3::new(-160.0, -200.0, 0.0),
//         //     scale: Vec3::new(50.0, wall_thickness, 1.0),
//         //     ..Default::default()
//         // },
//         // Transform {
//         //     translation: Vec3::new(-100.0, -180.0, 0.0),
//         //     scale: Vec3::new(90.0, wall_thickness, 1.0),
//         //     ..Default::default()
//         // },
//         // Transform {
//         //     translation: Vec3::new(60., -200.0, 0.0),
//         //     scale: Vec3::new(50.0, wall_thickness, 1.0),
//         //     ..Default::default()
//         // },
//         // Transform {
//         //     translation: Vec3::new(160., -220.0, 0.0),
//         //     scale: Vec3::new(50.0, wall_thickness, 1.0),
//         //     ..Default::default()
//         // },
//     ])
// }

#[derive(Component)]
pub struct Wall;

// for now we assume they're fixed size, specified by some constant
// revisit this once dynamically checking size becomes more ergonomic for
// both sprites and spritesheets/texture atlases
#[derive(Component, Clone, Copy)]
pub enum Aabb {
    StaticAabb { scale: &'static Vec2 },
    TransformScaleAabb,
}

impl Aabb {
    // pub fn new_static(scale: &'static Vec2) -> Self {
    //     Self::StaticAabb { scale }
    // }

    // Get the scale of the bounding box. We need the transform in case the Aabb is
    // TransformScaleAabb
    pub fn get_scale(&self, transform: &Transform) -> Vec2 {
        match self {
            Aabb::StaticAabb { scale: &scale } => scale,
            Aabb::TransformScaleAabb => transform.scale.truncate(),
        }
    }

    pub fn get_rect(&self, transform: &Transform) -> Rect {
        let scale = self.get_scale(transform);
        let translation = transform.translation.truncate();
        let top_left: Vec2 = translation - scale / 2.;
        let bottom_right: Vec2 = translation + scale / 2.;

        Rect {
            min: top_left,
            max: bottom_right,
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self::TransformScaleAabb
    }
}

// fn add_level_walls(commands: &mut Commands, Level(level): &Level) {
//     let wall_color = Color::rgb(0.8, 0.8, 0.8);
//     for transform in level {
//         commands
//             .spawn_bundle(SpriteBundle {
//                 transform: *transform,
//                 sprite: Sprite {
//                     color: wall_color,
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             })
//             .insert(Wall)
//             .insert(DrawAabb)
//             .insert(Aabb::default());
//     }
// }

#[derive(Component, PartialEq, Eq)]
pub struct PauseMessage;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add the game's entities to our world

    // cameras
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            // scale: 0.35,
            scale: 1.0,
            ..Default::default()
        },
        ..Default::default()
    });
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
        .insert(PauseMessage)
        .insert(Visibility { is_visible: false });
}

#[derive(Component)]
pub struct DrawAabb;

pub fn draw_aabbs(
    mut lines: ResMut<DebugLines>,
    q: Query<(&Transform, &Aabb), With<DrawAabb>>,
) {
    for (transform, aabb) in q.iter() {
        let rect = aabb.get_rect(transform);
        draw_rect_colored(&mut lines, rect, 0.0, Color::GREEN);
    }
}

fn draw_rect_colored(
    lines: &mut DebugLines,
    Rect { min, max }: Rect,
    duration: f32,
    color: Color,
) {
    let top_left = min.extend(0.0);
    let top_right = Vec3::new(max.x, min.y, 0.0);
    let bottom_left = Vec3::new(min.x, max.y, 0.0);
    let bottom_right = max.extend(0.0);

    lines.line_colored(top_left, top_right, duration, color);
    lines.line_colored(top_right, bottom_right, duration, color);
    lines.line_colored(bottom_right, bottom_left, duration, color);
    lines.line_colored(bottom_left, top_left, duration, color);
}

///        ┌──────┐
///        │      │
///        │ 0001 │
///        │      │
/// ┌──────┼──────┼──────┐
/// │      │      │      │
/// │ 0010 │      │ 0100 │
/// │      │      │      │
/// └──────┼──────┼──────┘
///        │      │
///        │ 1000 │
///        │      │
///        └──────┘
///
/// In order to decide which tile to render, tiles need to be classified
/// based on whether they have neighbours on each of the four sides.
/// This can be represented with 4 bits
fn classify_autotile(position: &IVec2, level: &Level) -> usize {
    let above = *position - IVec2::Y;
    let left = *position - IVec2::X;
    let right = *position + IVec2::X;
    let below = *position + IVec2::Y;

    let mut ret = 0b0000;
    if let Some(LevelContents::Tile) = level.0.get(&above) {
        ret |= 0b0001;
    }
    if let Some(LevelContents::Tile) = level.0.get(&left) {
        ret |= 0b0010;
    }
    if let Some(LevelContents::Tile) = level.0.get(&right) {
        ret |= 0b0100;
    }
    if let Some(LevelContents::Tile) = level.0.get(&below) {
        ret |= 0b1000;
    }

    ret
}

fn index2d_to_1d(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

// this is totally contingent and specific to the spritesheet I'm using
fn autotile_code_to_spritesheet_index(n: usize) -> usize {
    match n {
        0 => index2d_to_1d(0, 0, 20),
        1 => index2d_to_1d(0, 7, 20),
        2 => index2d_to_1d(3, 0, 20),
        3 => index2d_to_1d(3, 7, 20),
        4 => index2d_to_1d(1, 0, 20),
        5 => index2d_to_1d(1, 7, 20),
        6 => index2d_to_1d(2, 0, 20),
        7 => index2d_to_1d(2, 7, 20),
        8 => index2d_to_1d(0, 1, 20),
        9 => index2d_to_1d(0, 6, 20),
        10 => index2d_to_1d(3, 1, 20),
        11 => index2d_to_1d(3, 6, 20),
        12 => index2d_to_1d(1, 1, 20),
        13 => index2d_to_1d(1, 6, 20),
        14 => index2d_to_1d(2, 1, 20),
        15 => index2d_to_1d(2, 6, 20),
        _ => {
            assert!(n <= 0b1111);
            unreachable!();
        }
    }
}

pub fn spawn_level(
    commands: &mut Commands,
    tile_texture_atlas_handle: Handle<TextureAtlas>,
    portal_image_handle: Handle<Image>,
    level: &Level,
) {
    debug!("spawning level");

    for (position @ IVec2 { x, y }, level_contents_type) in &level.0 {
        // -y because image coordinates treat down as positive y direction
        const TILE_WIDTH: i32 = 18;
        let translation =
            Vec3::new((x * TILE_WIDTH) as f32, -1.0 * (y * TILE_WIDTH) as f32, 0.0);
        match level_contents_type {
            LevelContents::Player => {
                debug!("spawning a player at {:?}", translation);
                commands
                    .spawn_bundle(GuyBundle::with_translation(translation))
                    .insert(DrawAabb);
            }
            LevelContents::Tile => {
                // TODO extract this to a new custom bundle

                // let mut rng = rand::thread_rng();

                // let mut grass_dirt_indices: [usize; 8] = [0; 8];
                // for (i, tile_idx) in (0..4).chain(20..24).enumerate() {
                //     grass_dirt_indices[i] = tile_idx;
                // }

                // // const N_TILES: usize = 20 * 9;
                // // chosen by fair dice roll, guaranteed random
                // let random_index: usize =
                //     *grass_dirt_indices.choose(&mut rng).unwrap();
                debug!("spawning a tile at {:?}", translation);

                let tile_index = autotile_code_to_spritesheet_index(
                    classify_autotile(position, level),
                );
                commands
                    .spawn_bundle(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: tile_index,
                            ..Default::default()
                        },
                        transform: Transform {
                            translation,
                            ..default()
                        },
                        texture_atlas: tile_texture_atlas_handle.clone(),
                        ..default()
                    })
                    .insert(Wall)
                    .insert(DrawAabb)
                    .insert(Aabb::StaticAabb {
                        scale: &WALL_TILE_SIZE,
                    });
            }
            LevelContents::Portal(level_path) => {
                debug!("spawning a portal at {:?}", translation);
                commands.spawn_bundle(PortalBundle::new(
                    portal_image_handle.clone(),
                    translation,
                    level_path.to_path_buf(),
                ));
            }
        }
    }

    // let level1 = make_level_1();
    // add_level_walls(commands, &level1);
}

pub fn physics_system(
    mut query: Query<(Entity, &mut PhysicsObject, &mut Transform, Option<&Gravity>)>,
) {
    for (_entity, mut physics, mut transform, gravity) in query.iter_mut() {
        if gravity.is_some() {
            physics.velocity.y -= 23.0;
        }

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
    Loading,
    Paused,
}

pub fn guy_collision_system(
    time: Res<Time>,
    mut guy_query: Query<
        (&mut PhysicsObject, &mut Transform, &Aabb, &mut JumpState),
        (With<Guy>, Without<Wall>),
    >,
    wall_query: Query<(&Transform, &Aabb), (With<Wall>, Without<Guy>)>,
    portal_query: Query<(&Portal, &Transform, &Aabb), Without<Guy>>,
    mut commands: Commands,
) {
    let (mut guy_physics, mut guy_transform, &guy_aabb, mut jump_state) =
        guy_query.single_mut();

    let guy_size = guy_aabb.get_scale(&guy_transform);

    // PORTAL COLLISIONS
    for (portal, portal_transform, portal_aabb) in portal_query.iter() {
        let portal_size = portal_aabb.get_scale(portal_transform);
        let collision = collide(
            portal_transform.translation,
            portal_size,
            guy_transform.translation,
            guy_size,
        );

        if collision.is_some() {
            commands.insert_resource(NextState(AppState::Loading));
            commands.insert_resource(LoadingLevel::Path(portal.0.clone()));
        }
    }

    // WALL COLLISIONS
    // assume we're in the air until proven otherwise
    jump_state.on_ground = None;
    jump_state.coyote_timer.tick(time.delta());

    for (wall_transform, wall_aabb) in wall_query.iter() {
        let wall_size = wall_aabb.get_scale(wall_transform);

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
                jump_state.set_on_ground(guy_transform.translation.y);
                guy_transform.scale = GUY_SIZE.extend(0.);
            }
            Some(Collision::Inside) => {
                // Not sure if there's anything more reasonable we can do here
                guy_physics.velocity = Vec2::ZERO;
                guy_transform.translation = guy_physics.old_position;
            }
            None => (),
        }
    }
}

pub fn move_camera(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Guy>)>,
    player: Query<&Transform, (With<Guy>, Without<Camera>)>,
) {
    let guy_pos = if let Ok(player) = player.get_single() {
        player.translation
    } else {
        return;
    };

    for mut transform in camera.iter_mut() {
        let camera_pos: Vec3 = transform.translation;
        // i don't even know what the units are
        let epsilon: f32 = 1.0;
        transform.translation = if camera_pos.distance(guy_pos) < epsilon {
            guy_pos
        } else {
            camera_pos.lerp(guy_pos, 0.1)
        }
    }
}

pub fn update_jump_state(
    time: Res<Time>,
    mut query: Query<
        (&mut PhysicsObject, &mut Transform, &mut JumpState),
        With<Guy>,
    >,
) {
    for (mut physics, mut transform, mut jump_state) in query.iter_mut() {
        jump_state.coyote_timer.tick(time.delta());

        // update PreJump and possibly enact triggered prejump on contact with ground
        let on_ground = jump_state.on_ground.is_some();
        let timer = &mut jump_state.pre_jump_timer.timer;

        timer.tick(time.delta());

        if on_ground && !timer.finished() {
            jump_state.perform_jump(&mut physics, &mut transform);
        }
    }
}
