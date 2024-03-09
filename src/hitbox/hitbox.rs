use bevy::prelude::*;

/// Rect is in relative coordinates, with the origin at the transform.position of the entity
#[derive(Component, Debug, Clone)]
pub struct Hitbox {
    pub rect: Rect,
}

impl Hitbox {
    #[allow(dead_code)]
    pub fn new(rect: Rect) -> Self {
        Hitbox { rect }
    }

    #[allow(dead_code)]
    pub fn new_offsetless(size: Vec2) -> Self {
        Hitbox {
            rect: Rect::new(0.0, 0.0, size.x, size.y),
        }
    }

    #[allow(dead_code)]
    pub fn new_centered(size: Vec2) -> Self {
        Hitbox {
            rect: Rect::from_center_size(Vec2::ZERO, size),
        }
    }

    pub fn world_rect(&self, transform: &Transform) -> Rect {
        let min = transform.translation.truncate() + self.rect.min;
        let max = transform.translation.truncate() + self.rect.max;
        Rect::new(min.x, min.y, max.x, max.y)
    }

    #[allow(dead_code)]
    pub fn intersects(&self, other: &Hitbox, transform: &Transform, other_transform: &Transform) -> bool {
        let world_rect = self.world_rect(transform);
        let other_world_rect = other.world_rect(other_transform);
        !world_rect.intersect(other_world_rect).is_empty()
    }

    #[allow(dead_code)]
    pub fn contains_entirely(&self, other: &Hitbox, transform: &Transform, other_transform: &Transform) -> bool {
        let world_rect = self.world_rect(transform);
        let other_world_rect = other.world_rect(other_transform);
        world_rect.min.x <= other_world_rect.min.x &&
            world_rect.min.y <= other_world_rect.min.y &&
            world_rect.max.x >= other_world_rect.max.x &&
            world_rect.max.y >= other_world_rect.max.y
    }

    #[allow(dead_code)]
    pub fn offset(&self) -> Vec2 {
        self.rect.min
    }
}