use bevy::prelude::*;

use crate::GameState;
use crate::hitbox::*;

pub struct HitboxPlugin;

impl Plugin for HitboxPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CollisionEvent>()
            .add_systems(PreUpdate, emit_collision_events);
        if cfg!(debug_assertions) {
            // H to toggle hitbox gizmos
            app
                .add_systems(OnEnter(GameState::Playing), debug_spawn_sample_stuff) // TODO: remove
                .init_gizmo_group::<HitboxGizmos>()
                .add_systems(Update, (draw_hitbox_gizmos, update_hitbox_gizmos_config));
                // .add_systems(Update, log_collision_events);
        }
    }
}

fn debug_spawn_sample_stuff(
    mut commands: Commands,
    textures: Res<crate::loading::TextureAssets>,
    assets: Res<Assets<Image>>,
) {
    for _ in 0..10 {
        let x = rand::random::<f32>() * 800.0 - 400.0;
        let y = rand::random::<f32>() * 600.0 - 300.0;
        let image = textures.debug_mushroom.clone();
        let image_dim = assets.get(&image).unwrap().size();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                texture: image,
                ..Default::default()
            },
            Hitbox::new_offsetless(image_dim.as_vec2()),
            EmitsCollisions::default(),
        ));
    }
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

fn draw_hitbox_gizmos(
    mut gizmos: Gizmos<HitboxGizmos>,
    hitboxes: Query<(&Transform, &Hitbox, Option<&EmitsCollisions>)>,
) {
    for (transform, hitbox, collidable) in hitboxes.iter() {
        let color = if let Some(collidable) = collidable {
            if collidable.colliding_with.is_empty() {
                Color::rgba(0.0, 1.0, 0.0, 1.0)
            } else {
                Color::rgba(1.0, 0.0, 0.0, 1.0)
            }
        } else {
            Color::rgba(0.2, 0.2, 0.2, 1.0)
        };
        gizmos.rect_2d(
            transform.translation.truncate() + hitbox.offset(),
            0.0,
            hitbox.rect.size(),
            color,
        );
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
