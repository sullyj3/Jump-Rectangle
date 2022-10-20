use crate::physics_object::PhysicsObject;
use bevy::prelude::*;

#[derive(Component)]
pub enum JumpState {
    OnGround { y: f32 },
    Airborne {
        // When the user presses the jump key just before hitting the ground, we allow them
        // queue up a jump, which will be triggered when they make contact with the ground
        // pre_jump_timer: Timer,

        // TODO When the use jumps just after walking off a ledge, we allow them to jump anyway
        // coyote_timer: Timer,
    }
}

impl Default for JumpState {
    fn default() -> Self {
        Self::Airborne {
            // pre_jump_timer: Default::default(),
        }
    }
}

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
    jump_state: JumpState,
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
            physics: PhysicsObject::default(),
            jump_state: JumpState::default(),
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


pub fn jump(
    physics: &mut PhysicsObject,
    guy_transform: &mut Transform,
    jump_state: &mut JumpState,
) {
    physics.velocity.y = 750.0;
    guy_transform.scale = GUY_JUMPING_SIZE;
    *jump_state = JumpState::Airborne {};
}
