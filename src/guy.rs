use crate::physics_object::PhysicsObject;
use bevy::prelude::*;


#[derive(Component)]
pub struct PreJumpTimer {
    pub timer: Timer,
}

impl PreJumpTimer {
    const PRE_JUMP_TOLERANCE: f32 = 0.07;

    pub fn pre_jump(&mut self) {
        self.timer.reset();
    }
}

impl Default for PreJumpTimer {
    fn default() -> Self {
        PreJumpTimer {
            timer: Timer::from_seconds(Self::PRE_JUMP_TOLERANCE, false),
        }
    }
}

#[derive(Component)]
pub struct JumpState {
    pub on_ground: Option<f32>,

    // When the user presses the jump key just before hitting the ground, we allow them
    // queue up a jump, which will be triggered when they make contact with the ground
    pub pre_jump_timer: PreJumpTimer,

    // TODO When the use jumps just after walking off a ledge, we allow them to jump anyway
    // pub coyote_timer: Timer,
}


impl Default for JumpState {
    fn default() -> Self {
        Self {
            on_ground: None,
            pre_jump_timer: Default::default(),
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
    jump_state.on_ground = None;
}
