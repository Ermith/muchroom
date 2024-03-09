use bevy::prelude::*;

use crate::hitbox::Hitbox;
use super::collisions::EmitsCollisions;

#[derive(Component, Debug, Default)]
pub struct Draggable {
    pub drag_shadow: Option<Entity>,
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
    mut draggables: Query<(Entity, &mut Draggable, &mut Transform, Option<&DropBlocker>), Without<DragShadow>>,
    blockers: Query<(&Hitbox, &DropBlocker), Without<Draggable>>,
    drag_shadows: Query<(Entity, &DragShadow, &Transform, &EmitsCollisions)>,
) {
    if !mouse_buttons.just_released(MouseButton::Left) {
        return;
    }

    for (drag_shadow_entity, drag_shadow, drag_shadow_transform, EmitsCollisions{colliding_with}) in drag_shadows.iter() {
        let original_entity = drag_shadow.original_entity;
        let mut succeeded_drop = true;
        for collision in colliding_with {
            if *collision == original_entity { continue };
            let collides_with_undraggable_blockers = blockers.get(*collision).is_ok();
            let collides_with_draggable_blockers = draggables.get(*collision).ok().map(|(_, _, _, blocker)| blocker.is_some()).unwrap_or(false);
            if collides_with_undraggable_blockers || collides_with_draggable_blockers {
                succeeded_drop = false;
                break;
            }
        } 
        if succeeded_drop {
            if let Ok((_, mut draggable, mut transform, _)) = draggables.get_mut(original_entity) {
                let orig_z = transform.translation.z;
                transform.translation = drag_shadow_transform.translation;
                transform.translation.z = orig_z;
                draggable.drag_shadow = None;
            }
        }
        commands.entity(drag_shadow_entity).despawn();
    }
}