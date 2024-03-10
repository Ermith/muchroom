use bevy::{prelude::*, render::view::visibility};
use enumset::EnumSet;

use crate::hitbox::Hitbox;
use super::{collisions::EmitsCollisions, InLayers, Layer};

const DRAGGABLE_SCALE: f32 = 1.15;

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
    /// whether the drag shadow should be transparent
    pub drag_opaque: bool,
    pub no_hover_shadow: bool,
    pub hover_shadow: Option<Entity>,
}

#[derive(Component, Debug)]
pub struct DragShadow {
    pub offset: Vec2,
    pub original_entity: Entity,
}

#[derive(Component, Debug)]
pub struct HoverShadow{
    pub original_entity: Entity,
}

/// Blocks dropping if collides with a drag-shadow, needs hitbox
#[derive(Component, Debug)]
pub struct DropBlocker;

#[derive(Event, Debug)]
pub struct DropEvent {
    pub dropped_entity: Entity,
    pub dropped_on_entity: Entity,
}

pub fn initiate_drag(
    mut commands: Commands,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<super::mouse::MouseCoords>,
    mut query: Query<(Entity, &Transform, &Hitbox, &mut Draggable, &Handle<Image>, &Sprite, &mut Visibility)>,
    mut hover_shadows: Query<(Entity, &HoverShadow, &mut Transform), Without<Draggable>>,
) {
    let mouse_pos: Vec2 = mouse_coords.as_ref().into();

    for (hover_shadow_entity, HoverShadow{original_entity}, _) in &mut hover_shadows.iter() {
        if !query.get(*original_entity).is_ok() {
            commands.entity(hover_shadow_entity).despawn();
        }
    }

    let mut found_overlap = false;

    for (entity, transform, hitbox, mut draggable, image, sprite, mut visibility) in query.iter_mut() {
        if !found_overlap && hitbox.world_rect(transform).contains(mouse_pos) {
            if mouse_buttons.just_pressed(MouseButton::Left) {
                let offset = Vec2::ZERO; //transform.translation.truncate() - mouse_pos;
                let drag_shadow_entity = commands.spawn((
                    DragShadow {
                        offset,
                        original_entity: entity,
                    },
                    SpriteBundle {
                        texture: image.clone(),
                        sprite: Sprite {
                            custom_size: sprite.custom_size.and_then(|size| Some(size * DRAGGABLE_SCALE)),
                            ..sprite.clone()
                        },
                        transform: Transform::from_translation(transform.translation),
                        ..Default::default()
                    },
                    hitbox.clone(),
                    EmitsCollisions::default(),
                    crate::GameObject,
                )).id();
                draggable.drag_shadow = Some(drag_shadow_entity);
            } else {
                if draggable.hover_shadow.is_none() {
                    let mut translation = transform.translation;
                    translation.z = 5.0;
                    let hover_shadow_entity = commands.spawn((
                        HoverShadow {
                            original_entity: entity,
                        },
                        SpriteBundle {
                            texture: image.clone(),
                            sprite: Sprite {
                                custom_size: sprite.custom_size.and_then(|size| Some(size * DRAGGABLE_SCALE)),
                                color: Color::rgba(1.1, 1.1, 1.1, 1.0),
                                ..sprite.clone()
                            },
                            transform: Transform::from_translation(translation),
                            ..Default::default()
                        },
                        crate::GameObject,
                    )).id();
                    draggable.hover_shadow = Some(hover_shadow_entity);
                    *visibility = Visibility::Hidden;
                } else {
                    let (_, _, mut transform) = hover_shadows.get_mut(draggable.hover_shadow.unwrap()).unwrap();
                    transform.translation = transform.translation.truncate().extend(5.0);
                }
            }
            found_overlap = true;
        } else {
            if let Some(hover_shadow_entity) = draggable.hover_shadow {
                commands.entity(hover_shadow_entity).despawn();
                draggable.hover_shadow = None;
                *visibility = Visibility::Visible;
            }
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
    mut drop_events: ResMut<Events<DropEvent>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut draggables: Query<(Entity, &Hitbox, &mut Draggable, &mut Transform, Option<&DropBlocker>, &InLayers), Without<DragShadow>>,
    non_draggable_hitboxes: Query<(&Hitbox, Option<&DropBlocker>, &InLayers, &Transform), Without<Draggable>>,
    mut drag_shadows: Query<(Entity, &DragShadow, &Transform, &EmitsCollisions, &mut Sprite)>,
) {
    let released_drag = mouse_buttons.just_released(MouseButton::Left);

    for (drag_shadow_entity, drag_shadow, drag_shadow_transform, EmitsCollisions{colliding_with}, mut sprite) in &mut drag_shadows {
        let original_entity = drag_shadow.original_entity;
        let (_, dragged_hitbox, dragged_draggable, _, _, dragged_in_layers) = draggables.get(original_entity).unwrap();
        let mut collides_with_blocker = false;
        let mut is_contained_in_target = dragged_draggable.must_be_contained_in.is_none();
        let mut intersects_with_target = dragged_draggable.must_intersect_with.is_none();
        let mut special_entity_collision = false;
        let mut target = Entity::PLACEHOLDER;
        let mut target_position = Vec3::ZERO;
        for collision in colliding_with {
            if dragged_draggable.special_allowed_entities.contains(collision) {
                special_entity_collision = true;
                target = *collision;
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
                if collided_layers.intersects_layer_set(*must_be_contained_in) && collided_hitbox.contains_entirely(dragged_hitbox, &collided_transform, &drag_shadow_transform) {
                    is_contained_in_target = true;
                    if target == Entity::PLACEHOLDER {
                        target = *collision;
                        target_position = collided_transform.translation;
                    } else {
                        let current_distance = (drag_shadow_transform.translation - target_position).length();
                        let new_distance = (drag_shadow_transform.translation - collided_transform.translation).length();
                        if new_distance < current_distance {
                            target = *collision;
                            target_position = collided_transform.translation;
                        }
                    }
                }
            }
            if let Some(must_intersect_with) = dragged_draggable.must_intersect_with.as_ref() {
                if collided_layers.intersects_layer_set(*must_intersect_with) && collided_hitbox.intersects(dragged_hitbox, &collided_transform, &drag_shadow_transform) {
                    intersects_with_target = true;
                    if target == Entity::PLACEHOLDER {
                        target = *collision;
                        target_position = collided_transform.translation;
                    } else {
                        let current_distance = (drag_shadow_transform.translation - target_position).length();
                        let new_distance = (drag_shadow_transform.translation - collided_transform.translation).length();
                        if new_distance < current_distance {
                            target = *collision;
                            target_position = collided_transform.translation;
                        }
                    }
                }
            }
        }
        let succeeded_drop = special_entity_collision || (!collides_with_blocker && is_contained_in_target && intersects_with_target);
        let drag_opaque = dragged_draggable.drag_opaque;
        if let Ok((_, _, mut draggable, mut transform, _, _)) = draggables.get_mut(original_entity) {
            if released_drag && succeeded_drop {
                let orig_z = transform.translation.z;
                transform.translation = drag_shadow_transform.translation;
                transform.translation.z = orig_z;
                draggable.drag_shadow = None;
                drop_events.send(DropEvent {
                    dropped_entity: original_entity,
                    dropped_on_entity: target,
                });
                commands.entity(drag_shadow_entity).despawn();
            } else if !released_drag{
                sprite.color = if succeeded_drop { 
                    if !drag_opaque {
                        Color::rgba(1.5, 1.5, 1.5, 0.7)
                    } else {
                        Color::WHITE
                    }
                } else {
                    if !drag_opaque {
                        Color::rgba(1.2, 0.5, 0.5, 0.7)
                    } else {
                        Color::rgba(1.0, 0.5, 0.5, 1.0)
                    }
                };
            } else {
                commands.entity(drag_shadow_entity).despawn();
            }
        } else {
            commands.entity(drag_shadow_entity).despawn();
        }
    }
}