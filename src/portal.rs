use std::path::PathBuf;

use bevy::prelude::*;

use crate::platformer::{Aabb, DrawAabb};

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
        texture_handle: Handle<Image>,
        at: Vec3,
        level_path: PathBuf,
    ) -> Self {
        PortalBundle {
            portal: Portal(level_path),
            sprite: SpriteBundle {
                texture: texture_handle,
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
