use std::{collections::HashMap, io::Cursor, path::PathBuf};

use crate::platformer::{Aabb, DrawAabb};
use bevy::{prelude::*, render::render_resource::TextureFormat};

use image::{DynamicImage, ImageBuffer, ImageFormat, Rgba, RgbaImage};

pub enum LevelContents {
    Player,
    Tile,
    Portal(PathBuf),
}

// Vec2 is the position in units of 18x18 tiles, not in world space
pub struct Level(pub HashMap<IVec2, LevelContents>);

#[derive(Debug)]
pub enum LevelParseError {
    WrongNumberPlayers(i32),
}

// Specifies a level to be either fetched or generated
pub enum LoadingLevel {
    Path(PathBuf),
    // needs to be its own variant because menu is dynamically generated
    // rather than being loaded from a file
    Overworld,
}

// Copied from
// https://github.com/bevyengine/bevy/blob/v0.8.1/crates/bevy_render/src/texture/image_texture_conversion.rs
// Temporary measure for 0.8, 0.9 will have a public function
fn texture_to_image(texture: &Image) -> Option<DynamicImage> {
    match texture.texture_descriptor.format {
        TextureFormat::R8Unorm => ImageBuffer::from_raw(
            texture.texture_descriptor.size.width,
            texture.texture_descriptor.size.height,
            texture.data.clone(),
        )
        .map(DynamicImage::ImageLuma8),
        TextureFormat::Rg8Unorm => ImageBuffer::from_raw(
            texture.texture_descriptor.size.width,
            texture.texture_descriptor.size.height,
            texture.data.clone(),
        )
        .map(DynamicImage::ImageLumaA8),
        TextureFormat::Rgba8UnormSrgb => ImageBuffer::from_raw(
            texture.texture_descriptor.size.width,
            texture.texture_descriptor.size.height,
            texture.data.clone(),
        )
        .map(DynamicImage::ImageRgba8),
        _ => None,
    }
}

impl Level {
    pub fn from_bevy_image(img: &Image) -> Result<Self, LevelParseError> {
        let dynamic_image = texture_to_image(img).unwrap();
        let rgba: &RgbaImage = dynamic_image
            .as_rgba8()
            .expect("level could not be converted to rgba8");

        Level::from_rgba(rgba)
    }

    // pub fn load(to_load: &LoadingLevel) -> Self {
    //     match to_load {
    //         LoadingLevel::Path(level_path) => {
    //             // TODO also hack, what if cwd is not project root?
    //             let level_image: DynamicImage = image::io::Reader::open(level_path)
    //                 .expect("failed to open file assets/level3.png")
    //                 .decode()
    //                 .expect("decoding level3.png failed");

    //             let level_image: &RgbaImage = level_image
    //                 .as_rgba8()
    //                 .expect("level3.png could not be converted to rgba8");

    //             Level::from_rgba(level_image).unwrap_or_else(|e| match e {
    //                 LevelParseError::WrongNumberPlayers(_) => {
    //                     panic!(
    //                         "Wrong number of players while parsing level image: {:?}",
    //                         e
    //                     )
    //                 }
    //             })
    //         }
    //         LoadingLevel::Overworld => Level::generate_overworld_level(),
    //     }
    // }

    pub fn from_rgba(level_image: &RgbaImage) -> Result<Level, LevelParseError> {
        const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
        const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);

        let mut player_count = 0;

        let level: Level = Level(
            level_image
                .enumerate_pixels()
                .filter_map(|(x, y, pixel)| {
                    match *pixel {
                        // Black represents a wall tile
                        BLACK => Some((
                            IVec2::new(x as i32, y as i32),
                            LevelContents::Tile,
                        )),

                        // red represents the player
                        RED => {
                            player_count += 1;
                            Some((
                                IVec2::new(x as i32, y as i32),
                                LevelContents::Player,
                            ))
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

    pub fn generate_overworld_level() -> Self {
        let levels: glob::Paths =
            glob::glob("assets/level*.png").expect("failed to read glob pattern");

        const N_TILES_PER_LEVEL: usize = 5;
        Level(
            levels
                .map(Result::unwrap)
                .map(|p| p.strip_prefix("assets").unwrap().to_path_buf())
                .enumerate()
                .flat_map(|(i, level)| {
                    let offset = i * N_TILES_PER_LEVEL;
                    (offset..offset + N_TILES_PER_LEVEL)
                        .map(|x| {
                            let vec = IVec2::new(x as i32, 0);
                            (vec, LevelContents::Tile)
                        })
                        .chain(std::iter::once((
                            IVec2::new(offset as i32 + 2, -1),
                            LevelContents::Portal(level),
                        )))
                })
                .chain(std::iter::once((IVec2::new(0, -1), LevelContents::Player)))
                .collect(),
        )
    }
}

#[derive(Component)]
pub struct Portal(pub PathBuf);

#[derive(Bundle)]
pub struct PortalBundle {
    portal: Portal,
    #[bundle]
    sprite: SpriteBundle,
    aabb: Aabb,
    draw_aabb: DrawAabb,
}

impl PortalBundle {
    const PORTAL_BUNDLE_SCALE: Vec2 = Vec2::new(15., 15.);

    pub fn new(
        texture_handle: &Handle<Image>,
        at: Vec3,
        level_path: PathBuf,
    ) -> Self {
        PortalBundle {
            portal: Portal(level_path),
            sprite: SpriteBundle {
                texture: texture_handle.clone(),
                transform: Transform {
                    translation: at,
                    ..Default::default()
                },
                ..Default::default()
            },
            aabb: Aabb::StaticAabb {
                scale: &Self::PORTAL_BUNDLE_SCALE,
            },
            draw_aabb: DrawAabb,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    wall: Wall,
    aabb: Aabb,
}

#[derive(Component)]
pub struct Wall;

impl TileBundle {
    pub const TILE_SIZE: Vec2 = Vec2::new(18., 18.);

    pub fn new(
        tile_index: usize,
        translation: Vec3,
        texture_atlas: &Handle<TextureAtlas>,
    ) -> Self {
        TileBundle {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: tile_index,
                    ..Default::default()
                },
                transform: Transform {
                    translation,
                    ..default()
                },
                texture_atlas: texture_atlas.clone(),
                ..default()
            },
            wall: Wall,
            aabb: Aabb::StaticAabb {
                scale: &Self::TILE_SIZE,
            },
        }
    }
}
