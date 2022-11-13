#![allow(clippy::type_complexity)]
use crate::guy::Guy;
use crate::level::*;
use crate::platformer::{spawn_level, AppState, PauseMessage};
use bevy::ecs::query::ReadOnlyWorldQuery;
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

pub fn despawn_where<F: ReadOnlyWorldQuery>(
    to_despawn: Query<Entity, F>,
    mut commands: Commands,
) {
    to_despawn.for_each(|e| {
        commands.entity(e).despawn();
    });
}

pub fn despawn_level_contents(
    to_despawn: Query<Entity, Or<(With<Guy>, With<Wall>, With<Portal>)>>,
    commands: Commands,
) {
    despawn_where(to_despawn, commands)
}

////////////////////
/// Level Loading //
////////////////////

// This whole setup is very confusing, sorry. IMO this is the fault of the
// current bevy asset API.
// The way it's structured means we have to orchestrate things via Resources
// (ie mutable global variables), turning into a tricky ball of state.
// Maybe there are ways I could improve this, although it's not clear to me how
// to do that without ditching the AssetServer and loading the image myself, as
// I was doing before.
//
// 1. User begins in AppState::MainMenu, and presses Start, transitioning to
//    `AppState::Loading`
// 2. enter_loading checks whether we want a normal level, or the overworld level.
//    - Normal levels will be loaded as png files via the AssetServer. We
//      convert them into `RgbaImage`, then generate a `Level` from the pixels.
//    - The overworld will be dynamically generated without an image file, based on
//      how many regular levels there are. (TODO I just realized this check
//      depends on a relative path to the assets directory)
//  3. If we want to load a regular level
//     a. we insert a LoadingLevelImageHandle
//        resource, and wait in Loading using `wait_level_load` to check whether
//        the Image is available yet. Once it is, we generate a Level from it,
//        insert a LoadedLevel, and transition to AppState::InGame.
//     b. this will trigger exit_loading, which looks at the LoadedLevel, and
//        spawns entities based on it.
//
//     Otherwise, if we want the overworld level, we generate it immediately in
//     enter_loading, insert the LoadedLevel, and directly transition to AppState::InGame,
//     thereby triggering exit_loading, which will spawn the entities.
//     This should mean wait_level_load is never called.
//
// This approach feels insanely brittle
#[derive(Resource)]
pub struct LoadedLevel(Level);

#[derive(Resource)]
pub struct LoadingLevelImageHandle(Handle<Image>);

pub fn enter_loading(
    mut commands: Commands,
    to_load: Res<LoadingLevel>,
    asset_server: Res<AssetServer>,
) {
    info!("enter_loading");
    match &*to_load {
        LoadingLevel::Path(level_path) => {
            let level_image: Handle<Image> = asset_server.load(level_path.as_path());
            commands.insert_resource(LoadingLevelImageHandle(level_image));
        }
        LoadingLevel::Overworld => {
            let level = Level::generate_overworld_level();
            commands.insert_resource(LoadedLevel(level));
            commands.insert_resource(NextState(AppState::InGame));
        }
    }
}

// This should only run if we're loading a regular level, which implies waiting
// on the AssetServer. If we want the overworld level, we should have skipped through
// AppState::Loading to AppState::InGame
pub fn wait_level_load(
    level_image_handle: Res<LoadingLevelImageHandle>,
    images: Res<Assets<Image>>,
    mut commands: Commands,
) {
    info!("wait_level_load");
    let LoadingLevelImageHandle(handle) = &*level_image_handle;
    let Some(img) = images.get(&handle) else {
        debug!("waiting for level image to become available");
        return
    };

    let level = Level::from_bevy_image(img).unwrap_or_else(|e| match e {
        LevelParseError::WrongNumberPlayers(_) => {
            panic!("Wrong number of players while parsing level image: {:?}", e)
        }
    });
    commands.insert_resource(LoadedLevel(level));
    commands.insert_resource(NextState(AppState::InGame));
}

pub fn exit_loading(
    loaded_level: Res<LoadedLevel>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    info!("exit_loading");
    let LoadedLevel(level) = &*loaded_level;
    let tile_texture_handle = asset_server.load("tiles_packed.png");
    let portal_image_handle: Handle<Image> = asset_server.load("portal.png");
    let tile_texture_atlas =
        TextureAtlas::from_grid(tile_texture_handle, Vec2::new(18.0, 18.0), 20, 9, None, None);
    let tile_texture_atlas_handle: Handle<TextureAtlas> =
        texture_atlases.add(tile_texture_atlas);

    spawn_level(
        &mut commands,
        tile_texture_atlas_handle,
        portal_image_handle,
        &level,
    );

    commands.remove_resource::<LoadingLevelImageHandle>();
    commands.remove_resource::<LoadedLevel>();
    commands.insert_resource(NextState(AppState::InGame));
    debug!("loading complete, starting game");
}
