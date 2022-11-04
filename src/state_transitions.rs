use crate::level::*;
use crate::platformer::{spawn_level, PauseMessage};
use bevy::prelude::*;
use image::{DynamicImage, RgbaImage};
use std::iter;

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
    let portal_image_handle: Handle<Image> = asset_server.load("portal.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(18.0, 18.0), 20, 9);
    let tile_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(tile_texture_atlas);

    // TODO also hack, what if cwd is not project root?
    let level_image: DynamicImage = image::io::Reader::open("assets/level3.png")
        .expect("failed to open file assets/level3.png")
        .decode()
        .expect("decoding level3.png failed");

    let level_image: &RgbaImage = level_image
        .as_rgba8()
        .expect("level3.png could not be converted to rgba8");

    let level = Level::parse_image(level_image).unwrap_or_else(|e| match e {
        LevelParseError::WrongNumberPlayers(_) => {
            panic!("Wrong number of players while parsing level image: {:?}", e)
        }
    });

    // let level = generate_menu_level();

    spawn_level(
        &mut commands,
        tile_texture_atlas_handle,
        portal_image_handle,
        &level,
    );
}

// fn convert_image(images: Res<Assets<Image>>) {
//   if let Some(image) = images.get(your_handle) {
//     ImageBuffer::from_raw(w, h, image.data)
//   }
// }
