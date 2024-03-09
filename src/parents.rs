use std::time::Duration;

use rand::prelude::*;

use bevy::prelude::*;

use crate::{hitbox::*, loading::TextureAssets, GameState};

pub const MAX_PARENTS: usize = 5;
pub const MIN_PARENT_SPAWN_TIME: f32 = 10.0;
pub const MAX_PARENT_SPAWN_TIME: f32 = 30.0;
// Maybe in future replace with texture size?
pub const PARENT_SIZE: Vec2 = Vec2::new(128.0, 256.0);
pub const PARENT_WALK_SPEED: f32 = 100.0;
/// Distance from parent spawn to the start of parent waiting queue.
pub const PARENT_QUEUE_OFFSET: f32 = 256.0;
/// Gap between parents in the parent waiting queue.
pub const PARENT_GAP: f32 = 10.0;
/// Time after which will parent run out of patience, which results in game over.
pub const PARENT_MAX_PATIENCE: f32 = 120.0;

pub struct ParentsPlugin;

#[derive(Component)]
pub struct Parent {
    /// Position of parent in parent queue/
    queue_index: usize,
    patience_timer: Timer,
}

/// Occur when parent receive its child.
#[derive(Event)]
pub struct ReceiveChildEvent {
    pub parent_entity: Entity,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
/// Walks to destination. Upon reaching destination this component is removed.
struct PathWalker {
    destination: Vec2,
}

#[derive(Resource)]
struct ParentSpawnTimer(Timer);

#[derive(Resource, Default)]
struct ParentQueue([bool; MAX_PARENTS]);

impl Plugin for ParentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ParentSpawnTimer(
                Timer::from_seconds(
                    rand::thread_rng().gen_range(MIN_PARENT_SPAWN_TIME..=MAX_PARENT_SPAWN_TIME),
                    TimerMode::Repeating,
                )
            ))
            .init_resource::<ParentQueue>()
            .add_event::<ReceiveChildEvent>()
            .add_systems(Update, (
                handle_random_parent_spawning,
                move_walkers,
                update_patience,
                read_receive_child_events,
            ).run_if(in_state(GameState::Playing)));
    }
}

impl Default for Parent {
    fn default() -> Self {
        Self { queue_index: 0, patience_timer: Timer::from_seconds(PARENT_MAX_PATIENCE, TimerMode::Once) }
    }
}

fn handle_random_parent_spawning(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ParentSpawnTimer>,
    mut parent_queue: ResMut<ParentQueue>,
    textures: Res<TextureAssets>,
    window_query: Query<&Window>
) {
    let avaible_slot = parent_queue.0.iter().position(|&slot| !slot);
    if avaible_slot.is_none() {
        return;
    }
    let avaible_slot = avaible_slot.unwrap();

    timer.0.tick(time.delta());
    if timer.0.just_finished() || parent_queue.0.iter().all(|&slot| !slot) {
        timer.0.set_duration(Duration::from_secs_f32(
            rand::thread_rng().gen_range(MIN_PARENT_SPAWN_TIME..=MAX_PARENT_SPAWN_TIME)
        ));
        timer.0.reset();

        let window = window_query.single();
        let spawn_pos = Vec3::new(
            -window.width() / 2.0 - PARENT_SIZE.x / 2.0,
            window.height() / 2.0 - PARENT_SIZE.y / 2.0,
            1.0
        );
        parent_queue.0[avaible_slot] = true;
        commands.spawn((
            Parent {
                queue_index: avaible_slot,
                ..default()
            },
            SpriteBundle {
                texture: textures.placeholder_parent.clone(),
                transform: Transform::from_translation(spawn_pos),
                ..default()
            },
            PathWalker {
                destination: spawn_pos.xy() 
                    + Vec2::X * (PARENT_QUEUE_OFFSET + (PARENT_SIZE.x + PARENT_GAP) * avaible_slot as f32),
            }
        ));
    }
}

fn move_walkers(
    mut commands: Commands, 
    time: Res<Time>, 
    textures: Res<TextureAssets>,
    mut query: Query<(Entity, &mut Transform, &PathWalker)>
) {
    for (entity, mut transform, walker) in &mut query {
        let direction = (walker.destination.extend(0.0) - transform.translation).normalize();
        transform.translation += direction * PARENT_WALK_SPEED * time.delta_seconds();

        if Vec2::distance(transform.translation.xy(), walker.destination) < PARENT_WALK_SPEED * time.delta_seconds() {
            transform.translation = walker.destination.extend(0.0);
            commands.entity(entity).remove::<PathWalker>();

            commands.spawn((
                SpriteSheetBundle {
                    texture: textures.derp_spores.clone(),
                    transform: transform.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(64.0)),
                        ..default()
                    },
                    ..default()
                },
                Hitbox::new_centered(Vec2::splat(32.0)),
                EmitsCollisions::default(),
                //Draggable::default(),
            ));
        }
    }
}

fn update_patience(time: Res<Time>, mut query: Query<&mut Parent>) {
    for mut parent in &mut query {
        parent.patience_timer.tick(time.delta());

        // TODO: update animation

        if parent.patience_timer.just_finished() {
            // TODO: Game Over
            println!("GAME OVER!");
        }
    }
}

fn read_receive_child_events(
    mut commands: Commands,
    mut parent_queue: ResMut<ParentQueue>,
    mut events: EventReader<ReceiveChildEvent>,
    query: Query<&Parent>,
) {
    for event in events.read() {
        let parent = query.get(event.parent_entity).unwrap();
        parent_queue.0[parent.queue_index] = false;

        // TODO: animation?, increase score/currency

        commands.entity(event.parent_entity).despawn();
    }
}