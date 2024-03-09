use bevy::prelude::*;
use enumset::EnumSet;

use crate::hitbox::Hitbox;
use super::{collisions::EmitsCollisions, InLayers, Layer};

/// Dropping is blocked by entities with DropBlocker in layers that overlap with the draggable entity
/// Example:
///  - you want a tool to be droppable only on children: set must_intersect_with to new_single(Child), set the tool's layers to new_single(Tool)
///  - you want a child to be draggable only inside the garden but also onto parents:
///     set must_be_contained_in to new_single(Garden), set the child's layers to new_single(Child)
///     set special_allowed_entities vec![parent]
#[derive(Component, Debug, Default)]
pub struct Draggable {
    pub drag_shadow: Option<Entity>,
    /// if set, the entity can only be dropped if it is entirely contained in a *single hitbox* with an overlapping layer
    pub must_be_contained_in: Option<EnumSet<Layer>>,
    /// if set, the entity can only be dropped if it intersects with a hitbox with an overlapping layer
    pub must_intersect_with: Option<EnumSet<Layer>>,
    /// if set, overlapping with any of these entities allows a drop; this overrides all other conditions
    pub special_allowed_entities: Vec<Entity>,
}

#[derive(Component, Debug)]
pub struct DragShadow {
    pub offset: Vec2,
    pub original_entity: Entity,
}

/// Blocks dropping if collides with a drag-shadow, needs hitbox
#[derive(Component, Debug)]
pub struct DropBlocker;

pub fn initiate_drag(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<super::mouse::MouseCoords>,
    mut query: Query<(Entity, &Transform, &Hitbox, &mut Draggable, &Handle<Image>)>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let mouse_pos: Vec2 = mouse_coords.as_ref().into();

    for (entity, transform, hitbox, mut draggable, image) in query.iter_mut() {
        if hitbox.world_rect(transform).contains(mouse_pos) {
            let offset = transform.translation.truncate() - mouse_pos;
            let drag_shadow_entity = commands.spawn((
                DragShadow {
                    offset,
                    original_entity: entity,
                },
                SpriteBundle {
                    texture: image.clone(),
                    sprite: Sprite {
                        color: Color::rgba(1.5, 1.5, 1.5, 0.5),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(transform.translation).with_scale(Vec3::splat(1.3)),
                    ..Default::default()
                },
                hitbox.clone(),
                EmitsCollisions::default(),
            )).id();
            draggable.drag_shadow = Some(drag_shadow_entity);
            return; // at most one drag-start!
        }
    }
}

pub fn update_drag(
    mouse_coords: Res<super::mouse::MouseCoords>,
    mut query: Query<(&DragShadow, &mut Transform)>,
) {
    for (drag_shadow, mut transform) in query.iter_mut() {
        transform.translation = (mouse_coords.0 + drag_shadow.offset).extend(5.0);
    }
}

pub fn end_drag(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut draggables: Query<(Entity, &Hitbox, &mut Draggable, &mut Transform, Option<&DropBlocker>, &InLayers), Without<DragShadow>>,
    non_draggable_hitboxes: Query<(&Hitbox, Option<&DropBlocker>, &InLayers, &Transform), Without<Draggable>,>,
    drag_shadows: Query<(Entity, &DragShadow, &Transform, &EmitsCollisions)>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) {
        return;
    }

    for (drag_shadow_entity, drag_shadow, drag_shadow_transform, EmitsCollisions{colliding_with}) in drag_shadows.iter() {
        let original_entity = drag_shadow.original_entity;
        let (_, dragged_hitbox, dragged_draggable, _, _, dragged_in_layers) = draggables.get(original_entity).unwrap();
        let mut collides_with_blocker = false;
        let mut is_contained_in_target = dragged_draggable.must_be_contained_in.is_none();
        let mut intersects_with_target = dragged_draggable.must_intersect_with.is_none();
        let mut special_entity_collision = false;
        for collision in colliding_with {
            if dragged_draggable.special_allowed_entities.contains(collision) {
                special_entity_collision = true;
                break;
            }
            if *collision == original_entity { continue };
            let (collided_hitbox, blocker, collided_layers, collided_transform) = 
            if let Ok((hitbox, blocker, in_layers, transform)) = non_draggable_hitboxes.get(*collision) {
                (hitbox, blocker, in_layers, transform)
            } else if let Ok((_, hitbox, _, transform, blocker, in_layers)) = draggables.get(*collision) {
                (hitbox, blocker, in_layers, transform)
            } else {
                continue;
            };
            if blocker.is_some() && collided_layers.intersects(dragged_in_layers) {
                collides_with_blocker = true;
            }
            if let Some(must_be_contained_in) = dragged_draggable.must_be_contained_in.as_ref() {
                if collided_layers.intersects_layer_set(*must_be_contained_in) && collided_hitbox.contains_entirely(dragged_hitbox, &drag_shadow_transform, &collided_transform) {
                    is_contained_in_target = true;
                }
            }
            if let Some(must_intersect_with) = dragged_draggable.must_intersect_with.as_ref() {
                if collided_layers.intersects_layer_set(*must_intersect_with) && collided_hitbox.intersects(dragged_hitbox, &drag_shadow_transform, &collided_transform) {
                    intersects_with_target = true;
                }
            }
        }
        let succeeded_drop = special_entity_collision || (!collides_with_blocker && is_contained_in_target && intersects_with_target);
        if succeeded_drop {
            if let Ok((_, _, mut draggable, mut transform, _, _)) = draggables.get_mut(original_entity) {
                let orig_z = transform.translation.z;
                transform.translation = drag_shadow_transform.translation;
                transform.translation.z = orig_z;
                draggable.drag_shadow = None;
            }
        }
        commands.entity(drag_shadow_entity).despawn();
    }
}