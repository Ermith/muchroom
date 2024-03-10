use bevy::prelude::*;
use crate::parents::Species;

pub const SPORES_SIZE_MODIFIER: f32 = 0.5;
/// Size of spawned children.
pub const CHILD_SIZE: f32 = 130.0;
/// Size of hitbox of spawned children.
pub const CHILD_HITBOX_SIZE: f32 = 90.0;

#[derive(Component)]
pub struct Child {
    pub parent_entity: Entity,
    pub species: Species,
}

#[derive(Component)]
pub struct EyesVisual;

#[derive(Component)]
pub struct BodyVisual;
