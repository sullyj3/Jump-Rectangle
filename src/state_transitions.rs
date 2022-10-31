use crate::platformer::{spawn_level, PauseMessage};
use bevy::prelude::*;
use image::{DynamicImage, Rgba, RgbaImage};
use std::collections::HashMap;

pub fn enter_paused(
    mut pause_message_vis: Query<&mut Visibility, With<PauseMessage>>,
) {
    debug!("Game paused");
    let mut pm_visibility = pause_message_vis.single_mut();
    pm_visibility.is_visible = true;
}

pub fn exit_paused(
    mut pause_message_vis: Query<&mut Visibility, With<PauseMessage>>,
) {
    debug!("Game resumed");
    let mut pm_visibility = pause_message_vis.single_mut();
    pm_visibility.is_visible = false;
}

pub fn exit_menu(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    // images: Res<Assets<Image>>,
) {
    debug!("starting game");

    let tile_texture_handle = asset_server.load("tiles_packed.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(18.0, 18.0), 20, 9);
    let tile_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(tile_texture_atlas);

    // TODO also hack, what if cwd is not project root?
    let level_image: DynamicImage = image::io::Reader::open("assets/level2.png")
        .expect("failed to open file assets/level2.png")
        .decode()
        .expect("decoding level2.png failed");

    let level_image: &RgbaImage = level_image
        .as_rgba8()
        .expect("level2.png could not be converted to rgba8");

    let level = parse_level_image(level_image).unwrap_or_else(|e| match e {
        LevelParseError::WrongNumberPlayers(_) => {
            panic!("Wrong number of players while parsing level image: {:?}", e)
        }
    });

    spawn_level(&mut commands, tile_texture_atlas_handle, level);
}

pub enum LevelContents {
    Player,
    Tile,
}
pub struct Level(pub HashMap<UVec2, LevelContents>);

#[derive(Debug)]
enum LevelParseError {
    WrongNumberPlayers(i32),
}

pub const WALL_TILE_SIZE: Vec2 = Vec2::new(18., 18.);

fn parse_level_image(level_image: &RgbaImage) -> Result<Level, LevelParseError> {
    const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
    const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);

    let mut player_count = 0;

    let level: Level = Level(
        level_image
            .enumerate_pixels()
            .filter_map(|(x, y, pixel)| {
                match *pixel {
                    // Black represents a wall tile
                    BLACK => Some((UVec2::new(x, y), LevelContents::Tile)),

                    // red represents the player
                    RED => {
                        player_count += 1;
                        Some((UVec2::new(x, y), LevelContents::Player))
                    }
                    _ => None,
                }
            })
            .collect(),
    );

    if player_count != 1 {
        Err(LevelParseError::WrongNumberPlayers(player_count))
    } else {
        Ok(level)
    }
}

// fn convert_image(images: Res<Assets<Image>>) {
//   if let Some(image) = images.get(your_handle) {
//     ImageBuffer::from_raw(w, h, image.data)
//   }
// }
