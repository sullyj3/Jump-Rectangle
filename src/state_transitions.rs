use crate::platformer::{spawn_level, PauseMessage};
use bevy::prelude::*;
use bevy_polyline::prelude::*;

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
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
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

    spawn_level(
        &mut commands,
        character_texture_atlas_handle,
        tile_texture_atlas_handle,
        polyline_materials,
        polylines,
    );
}
