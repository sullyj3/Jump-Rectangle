use crate::guy::Guy;
use crate::level::*;
use crate::platformer::{spawn_level, AppState, PauseMessage};
use bevy::prelude::*;
use iyes_loopless::state::NextState;

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

pub fn enter_loading(
    to_despawn: Query<Entity, Or<(With<Guy>, With<Wall>, With<Portal>)>>,
    mut commands: Commands,
) {
    // despawn the level
    // TODO this seems brittle as hell
    //   do I have to remember to the marker struct of every single thing
    //   that can be in a level? There's got to be a better way
    //   need to investigate parent/child stuff
    for e in to_despawn.iter() {
        commands.entity(e).despawn();
    }

    // TODO load level assets using assetserver here, then wait for them
    // with a system in Loading state, instead of just loading everything in
    // the exit_loading state

    commands.insert_resource(NextState(AppState::InGame));
}

pub fn exit_loading(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    loading_level: Res<LoadingLevel>,
) {
    // TODO move all this stuff into level.rs probably
    let level: Level = Level::load(&*loading_level);

    let tile_texture_handle = asset_server.load("tiles_packed.png");
    let portal_image_handle: Handle<Image> = asset_server.load("portal.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(18.0, 18.0), 20, 9);
    let tile_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(tile_texture_atlas);

    spawn_level(
        &mut commands,
        tile_texture_atlas_handle,
        portal_image_handle,
        &level,
    );

    commands.remove_resource::<LoadingLevel>();
    commands.insert_resource(NextState(AppState::InGame));
    debug!("loading complete, starting game");
}

// // for level loading: to convert Image to ImageBuffer for pixel pixel access
// fn convert_image(images: Res<Assets<Image>>) {
//   if let Some(image) = images.get(your_handle) {
//     ImageBuffer::from_raw(w, h, image.data)
//   }
// }
