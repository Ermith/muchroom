use bevy::prelude::*;

/// colliding_with is cleared every frame, and filled with the entities that the Hitbox is colliding with
/// this happens in PreUpdate
/// events are also emitted for each collision, see [CollisionEvent]
#[derive(Component, Debug, Default)]
pub struct EmitsCollisions{
    pub colliding_with: Vec<Entity>
}

/// Get emitted whenever an EmitsCollisions + Hitbox entity collides with another Hitbox entity
/// if both have EmitsCollisions, both will emit a CollisionEvent.
/// Event emitted in PreUpdate
#[derive(Event)]
pub struct CollisionEvent {
    pub collider: Entity,
    pub collidee: Entity,
}