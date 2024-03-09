use bevy::prelude::*;

use crate::GameState;
use super::*;
use super::draggable::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<super::mouse::MouseCoords>()
            .add_systems(PreUpdate, super::mouse::mouse_coords_system)
            .add_event::<CollisionEvent>()
            .add_systems(PreUpdate, emit_collision_events)
            .add_event::<DropEvent>()
            .add_systems(Update, (initiate_drag, update_drag, end_drag));
        if cfg!(debug_assertions) {
            // H to toggle hitbox gizmos
            app
                .add_systems(OnEnter(GameState::Playing), debug_spawn_sample_stuff) // TODO: remove
                .init_gizmo_group::<HitboxGizmos>()
                // .add_systems(Update, log_collision_events);
                // .add_systems(Update, log_drop_events)
                .add_systems(Update, (draw_hitbox_gizmos, update_hitbox_gizmos_config));

        }
    }
}

fn debug_spawn_sample_stuff(
    mut commands: Commands,
    textures: Res<crate::loading::TextureAssets>,
    assets: Res<Assets<Image>>,
) {
    let special_test_mushroom = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 0.0, 0.0, 1.0),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(200.0, 0.0, 0.0)),
            texture: textures.debug_mushroom.clone(),
            ..Default::default()
        },
        Hitbox::new_centered(assets.get(&textures.debug_mushroom).unwrap().size().as_vec2()),
        InLayers::new_single(Layer::Parent),
    )).id();

    for _ in 0..10 {
        let x = rand::random::<f32>() * 800.0 - 400.0;
        let y = rand::random::<f32>() * 600.0 - 300.0;
        let image = textures.debug_mushroom.clone();
        let image_dim = assets.get(&image).unwrap().size();
        let mut spawn = commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                sprite: Sprite {
                    custom_size: Some(image_dim.as_vec2()),
                    ..Default::default()
                },
                texture: image,
                ..Default::default()
            },
            Hitbox::new_centered(image_dim.as_vec2()),
            EmitsCollisions::default(),
            Draggable {
                must_be_contained_in: Some(Layer::Garden.into()),
                special_allowed_entities: vec![special_test_mushroom],
                ..Default::default()
            },
            InLayers::new_single(Layer::Child),
        ));
        if rand::random::<f32>() < 0.5 {
            spawn.insert(DropBlocker);
        }
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            texture: textures.github.clone(),
            ..Default::default()
        },
        Hitbox::new_centered(assets.get(&textures.github).unwrap().size().as_vec2()),
        InLayers::new_single(Layer::Garden),
    ));
}

fn emit_collision_events(
    mut collision_events: EventWriter<CollisionEvent>,
    mut collidables: Query<(Entity, &Transform, &Hitbox, &mut EmitsCollisions)>,
    all_hitboxes: Query<(Entity, &Transform, &Hitbox)>,
) {
    for (collidable_entity, collidable_transform, collidable_hitbox, mut collision_emitter) in collidables.iter_mut() {
        collision_emitter.colliding_with.clear();
        for (other_entity, other_transform, other_hitbox) in all_hitboxes.iter() {
            if collidable_entity == other_entity {
                continue;
            }
            if collidable_hitbox.intersects(other_hitbox, &collidable_transform, &other_transform) {
                collision_events.send(CollisionEvent {
                    collider: collidable_entity,
                    collidee: other_entity,
                });
                collision_emitter.colliding_with.push(other_entity);
            }
        }
    }
}


// debug gizmos

#[derive(Default, Reflect, GizmoConfigGroup)]
struct HitboxGizmos {}

#[allow(dead_code)]
fn log_collision_events(
    mut events: EventReader<CollisionEvent>,
) {
    for event in events.read() {
        info!("Collision: {:?} -> {:?}", event.collider, event.collidee);
    }
}

#[allow(dead_code)]
fn log_drop_events(
    mut events: EventReader<DropEvent>,
) {
    for event in events.read() {
        info!("Drop: {:?}", event.dropped_entity);
    }
}

fn draw_hitbox_gizmos(
    mut gizmos: Gizmos<HitboxGizmos>,
    hitboxes: Query<(&Transform, &Hitbox, Option<&EmitsCollisions>, Option<&DropBlocker>)>,
) {
    for (transform, hitbox, collidable, drop_blocker) in hitboxes.iter() {
        let color = if let Some(collidable) = collidable {
            if collidable.colliding_with.is_empty() {
                Color::rgba(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::rgba(1.0, 0.0, 0.0, 1.0)
            }
        } else {
            Color::rgba(0.2, 0.2, 0.2, 1.0)
        };
        let world_rect = hitbox.world_rect(transform);
        gizmos.rect_2d(
            world_rect.center(),
            0.0,
            world_rect.size(),
            color,
        );
        if drop_blocker.is_some() {
            gizmos.line_2d(
                world_rect.min,
                world_rect.max,
                Color::rgba(1.0, 0.0, 0.0, 0.2),
            );
            gizmos.line_2d(
                Vec2::new(world_rect.min.x, world_rect.max.y),
                Vec2::new(world_rect.max.x, world_rect.min.y),
                Color::rgba(1.0, 0.0, 0.0, 0.2),
            );
        }
    }
}

fn update_hitbox_gizmos_config(
    mut config_store: ResMut<GizmoConfigStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let (config, _) = config_store.config_mut::<HitboxGizmos>();
    if keyboard.just_pressed(KeyCode::KeyH) {
        config.enabled = !config.enabled;
    }
}
