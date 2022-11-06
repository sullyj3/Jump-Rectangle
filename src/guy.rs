use crate::input::{make_game_input_map, GameAction};
use crate::physics_object::{Gravity, PhysicsObject};
use crate::platformer::Aabb;
use bevy::prelude::*;
use bevy::utils::Duration;
use leafwing_input_manager::prelude::*;

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
    const COYOTE_TOLERANCE: f32 = 0.3;

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

    fn set_on_ground(&mut self) {
        *self = Self::default();

        if let Some(t) = self.timer.as_mut() {
            t.reset()
        }
    }

    pub fn can_jump(&self) -> bool {
        let Some(timer) = &self.timer else { return false };
        !timer.finished()
    }
}

impl Default for CoyoteTimer {
    fn default() -> Self {
        Self {
            timer: Some(Timer::from_seconds(Self::COYOTE_TOLERANCE, false)),
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct JumpState {
    pub on_ground: Option<f32>,

    // When the user presses the jump key just before hitting the ground, we allow them
    // queue up a jump, which will be triggered when they make contact with the ground
    pub pre_jump_timer: PreJumpTimer,

    // When the payer jumps just after walking off a ledge, we allow them to jump anyway
    pub coyote_timer: CoyoteTimer,
}

impl JumpState {
    pub fn try_jump(
        &mut self,
        physics: &mut PhysicsObject,
        guy_transform: &mut Transform,
    ) {
        enum JumpAction {
            Jump,
            PreJump,
        }

        let should_jump = match self.on_ground {
            Some(..) => JumpAction::Jump,
            None => {
                if self.coyote_timer.can_jump() {
                    JumpAction::Jump
                } else {
                    JumpAction::PreJump
                }
            }
        };

        match should_jump {
            JumpAction::Jump => self.perform_jump(physics, guy_transform),
            JumpAction::PreJump => self.pre_jump_timer.pre_jump(),
        }
    }

    pub fn perform_jump(
        &mut self,
        physics: &mut PhysicsObject,
        guy_transform: &mut Transform,
    ) {
        const JUMP_SPEED: f32 = 600.0;
        physics.velocity.y = JUMP_SPEED;
        guy_transform.scale = GUY_JUMPING_SIZE;
        self.on_ground = None;
        self.coyote_timer.jump();
    }

    pub fn set_on_ground(&mut self, y: f32) {
        self.on_ground = Some(y);
        self.coyote_timer.set_on_ground();
    }
}

// fn index_2d(width: usize, x: usize, y: usize) -> usize {
//     width * y + x
// }

#[derive(Component)]
pub struct Guy {
    pub h_speed: f32,
}

#[derive(Bundle)]
pub struct GuyBundle {
    guy: Guy,
    #[bundle]
    sprite: SpriteBundle,
    aabb: Aabb,
    physics: PhysicsObject,
    jump_state: JumpState,
    gravity: Gravity,
    #[bundle]
    input_manager: InputManagerBundle<GameAction>,
}

pub const GUY_SIZE: Vec2 = Vec2::new(16.0, 16.0);
pub const GUY_JUMPING_SIZE: Vec3 = Vec3::new(14.0, 20.0, 0.0);

impl Default for GuyBundle {
    fn default() -> Self {
        GuyBundle {
            guy: Guy { h_speed: 180. },
            sprite: SpriteBundle {
                transform: Transform {
                    scale: GUY_SIZE.extend(0.0),
                    ..Default::default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.5, 0.5, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            aabb: Aabb::TransformScaleAabb,
            physics: PhysicsObject::default(),
            jump_state: JumpState::default(),
            gravity: Gravity::default(),
            input_manager: InputManagerBundle {
                action_state: ActionState::default(),
                input_map: make_game_input_map(),
            },
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

#[derive(Component)]
pub struct CanFly;
