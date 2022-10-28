#![allow(clippy::type_complexity)]

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    // input::keyboard::KeyboardInput,
    // input::gamepad::*,
};
use bevy_prototype_debug_lines::*;

use crate::{
    guy::*,
    physics_object::{Gravity, PhysicsObject},
};

pub const TIME_STEP: f32 = 1. / 60.0;
pub const PHYSICS_TIME_STEP: f32 = 1.0 / 120.0;

// pub struct Level(Vec<Transform>);

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
//         Transform {
//             translation: Vec3::new(-280.0, -220.0, 0.0),
//             scale: Vec3::new(50.0, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         Transform {
//             translation: Vec3::new(-160.0, -200.0, 0.0),
//             scale: Vec3::new(50.0, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         Transform {
//             translation: Vec3::new(-100.0, -180.0, 0.0),
//             scale: Vec3::new(90.0, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         Transform {
//             translation: Vec3::new(60., -200.0, 0.0),
//             scale: Vec3::new(50.0, wall_thickness, 1.0),
//             ..Default::default()
//         },
//         Transform {
//             translation: Vec3::new(160., -220.0, 0.0),
//             scale: Vec3::new(50.0, wall_thickness, 1.0),
//             ..Default::default()
//         },
//     ])
// }

#[derive(Component)]
pub struct Wall;

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
//             .insert(Wall);
//     }
// }

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
        .insert(PauseMessage)
        .insert(Visibility { is_visible: false });
}

#[derive(Component)]
pub struct DrawAABB;

// TODO update this once we have proper aabb components
pub fn draw_aabbs(
    mut lines: ResMut<DebugLines>,
    q: Query<&Transform, With<DrawAABB>>,
) {
    for transform in q.iter() {
        // TODO BUG translation is in fact the center, not the top left
        // consult the guy_collision_system collide() call
        let top_left: Vec3 = transform.translation;
        let top_right: Vec3 = top_left + transform.scale.x * Vec3::X;
        let bottom_left: Vec3 = top_left + transform.scale.y * Vec3::Y;
        let bottom_right: Vec3 = top_left + transform.scale;

        lines.line_colored(top_left, top_right, 0.0, Color::GREEN);
        lines.line_colored(top_right, bottom_right, 0.0, Color::GREEN);
        lines.line_colored(bottom_right, bottom_left, 0.0, Color::GREEN);
        lines.line_colored(bottom_left, top_left, 0.0, Color::GREEN);
    }
}

pub fn spawn_level(
    commands: &mut Commands,
    character_texture_atlas_handle: Handle<TextureAtlas>,
    tile_texture_atlas_handle: Handle<TextureAtlas>,
) {
    info!("spawning level");

    //texture
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: character_texture_atlas_handle,
        ..default()
    });

    let tile_width = 18;
    for i in 0..10 {
        let translation = Vec3::new(-280.0 + (i * tile_width) as f32, -220.0, 0.0);

        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation,
                    // TODO this is not correct
                    // we need the scale to remain at default for the sprite to be drawn at its
                    // native size
                    // however, we need scale for Guy, since it's just a color
                    // what this implies is that the AABB needs to be decoupled from the scale
                    // possibly we chould create an AABB component
                    scale: Vec3::new(tile_width as f32, tile_width as f32, 0.0),
                    ..default()
                },
                texture_atlas: tile_texture_atlas_handle.clone(),
                ..default()
            })
            .insert(Wall)
            .insert(DrawAABB);
    }

    // guy
    commands
        .spawn_bundle(GuyBundle::with_translation(Vec3::new(-260.0, -130.0, 0.0)))
        // todo Gravity should be in guy bundle
        // .insert(Gravity)
        .insert(CanFly)
        .insert(DrawAABB);

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
    Paused,
}

pub fn guy_collision_system(
    time: Res<Time>,
    mut guy_query: Query<
        (&mut PhysicsObject, &mut Transform, &mut JumpState),
        (With<Guy>, Without<Wall>),
    >,
    wall_query: Query<&Transform, (With<Wall>, Without<Guy>)>,
) {
    let (mut guy_physics, mut guy_transform, mut jump_state) =
        guy_query.single_mut();

    let guy_size = guy_transform.scale.truncate();
    jump_state.on_ground = None;

    jump_state.coyote_timer.tick(time.delta());

    for wall_transform in wall_query.iter() {
        // TODO BUG: using the scale as the size is not correct
        // eg we can have an 18x18 image with a scale of 1.0
        let wall_size = wall_transform.scale.truncate();
        let collision = collide(
            wall_transform.translation,
            wall_size,
            guy_transform.translation,
            guy_size,
        );
        if let Some(col) = &collision {
            debug!("collision detected: {:?}", col);
        }
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
                jump_state.on_ground = Some(guy_transform.translation.y);
                // reset the coyote timer, aka "time since guy was last on ground"
                jump_state.coyote_timer.set_on_ground();
                guy_transform.scale = GUY_SIZE;
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
) {
    let guy_pos: Vec3 = player.single().translation;

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
