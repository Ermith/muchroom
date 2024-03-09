use bevy::prelude::*;
use crate::parents::Species;

#[derive(Component)]
pub struct Child {
    pub parent_entity: Entity,
    pub species: Species
}