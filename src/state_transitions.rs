use crate::platformer::{spawn_level, PauseMessage};
use bevy::prelude::*;
use image::{DynamicImage, RgbaImage};

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
    images: Res<Assets<Image>>,
) {
    debug!("starting game");

    let character_texture_handle = asset_server.load("characters_packed.png");
    let character_texture_atlas = TextureAtlas::from_grid(
        character_texture_handle,
        Vec2::new(24.0, 24.0),
        9,
        3,
    );
    let character_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(character_texture_atlas);

    let tile_texture_handle = asset_server.load("tiles_packed.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(18.0, 18.0), 20, 9);
    let tile_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(tile_texture_atlas);

    // TODO also hack, what if cwd is not project root?
    let level_image: DynamicImage = image::io::Reader::open("assets/level1.png")
        .expect("failed to open file assets/level1.png")
        .decode()
        .expect("decoding level1.png failed");

    // let level_image_handle: Handle<Image> = asset_server.load("level1.png");
    // // TODO Hack
    // // block until loaded
    // let level_image: &Image = loop {
    //     use bevy::asset::LoadState;
    //     match asset_server.get_load_state(level_image_handle.clone()) {
    //         LoadState::Loading => continue,
    //         LoadState::Loaded => break images.get(&level_image_handle).unwrap(),
    //         LoadState::Failed => {
    //             info!("loading level failed! Exiting.");
    //             std::process::exit(1);
    //         }
    //         ls @ LoadState::NotLoaded => {
    //             error!("loadstate is: {:?}", ls);
    //             unreachable!()
    //         },
    //         ls => {
    //             error!("loadstate is: {:?}", ls);
    //             unreachable!()
    //         }
    //     }
    // };

    spawn_level(
        &mut commands,
        character_texture_atlas_handle,
        tile_texture_atlas_handle,
        level_image,
    );
}

// fn convert_image(images: Res<Assets<Image>>) {
//   if let Some(image) = images.get(your_handle) {
//     ImageBuffer::from_raw(w, h, image.data)
//   }
// }
