use bevy::prelude::*;

#[derive(Component)]
pub struct Child {
    pub parent_entity: Entity,
}