use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PhysicsObject {
    pub velocity: Vec2,
    pub old_position: Vec3,
}
