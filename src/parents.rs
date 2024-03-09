use std::time::Duration;

use rand::prelude::*;

use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

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

//pub type ParentList = [MAX_PARENTS; Entity];

pub struct ParentsPlugin;

#[derive(Component, Default)]
pub struct Parent {
    queue_index: usize,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PathWalker {
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
            .add_systems(Update, (handle_random_parent_spawning, move_walkers).run_if(in_state(GameState::Playing)));
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

fn move_walkers(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Transform, &PathWalker)>) {
    for (entity, mut transform, walker) in &mut query {
        let direction = (walker.destination.extend(0.0) - transform.translation).normalize();
        transform.translation += direction * PARENT_WALK_SPEED * time.delta_seconds();

        if Vec2::distance(transform.translation.xy(), walker.destination) < PARENT_WALK_SPEED * time.delta_seconds() {
            transform.translation = walker.destination.extend(0.0);
            commands.entity(entity).remove::<PathWalker>();
        }
    }
}

/*
* TODO:
* - random spawning
* - lose condition
* - variances and childrens
*/