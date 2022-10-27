use crate::physics_object::PhysicsObject;
use bevy::prelude::*;
use bevy::utils::Duration;

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
pub struct CoyoteTimer {
    // None if we jumped to get into the air, rather than falling off a ledge
    pub timer: Option<Timer>,
}

impl CoyoteTimer {
    const COYOTE_TOLERANCE: f32 = 0.4;

    pub fn tick(&mut self, delta: Duration) {
        if let Some(timer) = &mut self.timer {
            timer.tick(delta);
        }
    }

    // if we jumped, set the timer to None to ensure we aren't allowing double
    // jumps
    pub fn jump(&mut self) {
        self.timer = None;
    }

    pub fn set_on_ground(&mut self) {
        *self = Self::default();
        self.timer.as_mut().map(|t| t.reset());
    }

    pub fn can_jump(&self) -> bool {
        if let Some(timer) = &self.timer {
            !timer.finished()
        } else {
            false
        }
    }
}

impl Default for CoyoteTimer {
    fn default() -> Self {
        Self {
            timer: Some(Timer::from_seconds(Self::COYOTE_TOLERANCE, false)),
        }
    }
}

#[derive(Component, Debug)]
pub struct JumpState {
    pub on_ground: Option<f32>,

    // When the user presses the jump key just before hitting the ground, we allow them
    // queue up a jump, which will be triggered when they make contact with the ground
    pub pre_jump_timer: PreJumpTimer,

    // When the payer jumps just after walking off a ledge, we allow them to jump anyway
    pub coyote_timer: CoyoteTimer,
}

impl Default for JumpState {
    fn default() -> Self {
        Self {
            on_ground: None,
            pre_jump_timer: Default::default(),
            coyote_timer: Default::default(),
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
    jump_state.coyote_timer.jump();
}
