use bevy::prelude::*;
use crate::physics_object::PhysicsObject;

#[derive(Component)]
pub struct Guy {
    pub h_speed: f32,
}

#[derive(Bundle)]
pub struct GuyBundle {
    guy: Guy,
    #[bundle]
    sprite: SpriteBundle,
    physics: PhysicsObject,
}

pub const GUY_SIZE: Vec3 = Vec3::new(20.0, 50.0, 0.0);
pub const GUY_JUMPING_SIZE: Vec3 = Vec3::new(17.0, 55.0, 0.0);

impl Default for GuyBundle {
    fn default() -> Self {
        GuyBundle {
            guy: Guy { h_speed: 300. },
            sprite: SpriteBundle {
                transform: Transform {
                    scale: GUY_SIZE,
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            physics: PhysicsObject::default() 
        }
    }
}

impl GuyBundle {
    pub fn with_translation(translation: Vec3) -> Self {
        let mut guy = GuyBundle::default();
        guy.sprite.transform.translation = translation;
        guy
    }
}

