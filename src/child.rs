use bevy::prelude::*;
use crate::parents::Species;

/// Size of spawned children.
pub const CHILD_SIZE: f32 = 128.0;
/// Size of hitbox of spawned children.
pub const CHILD_HITBOX_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Child {
    pub parent_entity: Entity,
    pub species: Species
}