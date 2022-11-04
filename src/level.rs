use std::collections::HashMap;

use bevy::prelude::*;

use image::{Rgba, RgbaImage};

pub enum LevelContents {
    Player,
    Tile,
    Portal,
}

// Vec2 is the position in units of 18x18 tiles, not in world space
pub struct Level(pub HashMap<IVec2, LevelContents>);

#[derive(Debug)]
pub enum LevelParseError {
    WrongNumberPlayers(i32),
}

pub const WALL_TILE_SIZE: Vec2 = Vec2::new(18., 18.);

// TODO make this a method
pub fn parse_level_image(level_image: &RgbaImage) -> Result<Level, LevelParseError> {
    const BLACK: Rgba<u8> = Rgba([0, 0, 0, 255]);
    const RED: Rgba<u8> = Rgba([255, 0, 0, 255]);

    let mut player_count = 0;

    let level: Level = Level(
        level_image
            .enumerate_pixels()
            .filter_map(|(x, y, pixel)| {
                match *pixel {
                    // Black represents a wall tile
                    BLACK => {
                        Some((IVec2::new(x as i32, y as i32), LevelContents::Tile))
                    }

                    // red represents the player
                    RED => {
                        player_count += 1;
                        Some((IVec2::new(x as i32, y as i32), LevelContents::Player))
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
