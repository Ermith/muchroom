use bevy::prelude::*;

use crate::{child::Child, growing::Growable, hitbox::*, loading::TextureAssets, GameState};

const HUNGER_DECREASE_RATE: f32 = 1.0;
const HUNGER_FULL_VALUE: f32 = 10.0;
const BUBBLE_OFFSET: Vec2 = Vec2::splat(48.0);
const BUCKET_SPAWN_POS: Vec2 = Vec2::new(750.0, 90.0);

pub struct HungerPlugin;

#[derive(Component)]
pub struct Hunger {
    value: f32,
    bubble: Option<Entity>,
}

#[derive(Component)]
pub struct Food;

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_bucket)
            .add_systems(Update, (
                handle_hunger_decrease,
                read_on_drop_events
            ).run_if(in_state(GameState::Playing)));
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Self { value: HUNGER_FULL_VALUE, bubble: None }
    }
}

fn spawn_bucket(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: textures.placeholder_bucket.clone(),
            transform: Transform::from_translation(BUCKET_SPAWN_POS.extend(1.0)),
            ..default()
        },
    ));
    commands.spawn((
        SpriteBundle {
            texture: textures.placeholder_food.clone(),
            transform: Transform::from_translation(BUCKET_SPAWN_POS.extend(1.0)),
            sprite: Sprite {
                custom_size: Some(Vec2::splat(32.0)),
                ..default()
            },
            ..default()
        },
        Hitbox::new_centered(Vec2::splat(32.0)),
        InLayers::new_single(Layer::Tool),
        Draggable {
            must_intersect_with: Some(Layer::Child.into()),
            ..default()
        },
        Food,
    ));
}

fn handle_hunger_decrease(
    mut commands: Commands,
    time: Res<Time>, 
    textures: Res<TextureAssets>,
    mut query: Query<(Entity, &mut Hunger, &mut Growable)>
) {
    for (entity, mut hunger, mut growable) in &mut query {
        if hunger.value == 0.0 {
            continue;
        }

        hunger.value -= time.delta_seconds() * HUNGER_DECREASE_RATE;

        if hunger.value < 0.0 {
            hunger.value = 0.0;
        }

        growable.stopped = hunger.value == 0.0;

        if hunger.value == 0.0 && hunger.bubble.is_none() {
            hunger.bubble = Some(commands.spawn(
                SpriteBundle {
                    texture: textures.hunger_bubble.clone(),
                    transform: Transform::from_translation(BUBBLE_OFFSET.extend(1.0)),
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(64.0)),
                        ..default()
                    },
                    ..default()
                },
            ).id());

            commands.entity(entity).add_child(hunger.bubble.unwrap());
            commands.entity(hunger.bubble.unwrap()).set_parent(entity);
        }
    }
}

fn read_on_drop_events(
    mut commands: Commands,
    mut events: EventReader<DropEvent>,
    query: Query<&Food>,
    mut child_query: Query<&mut Hunger, With<Child>>,
    mut food_query: Query<&mut Transform, With<Food>>
) {
    for event in events.read() {
        if query.get(event.dropped_entity).is_err() {
            continue;
        }

        if let Ok(mut transform) = food_query.get_mut(event.dropped_entity) {
            transform.translation = BUCKET_SPAWN_POS.extend(1.0);

            let mut hunger = child_query.get_mut(event.dropped_on_entity).unwrap();

            if hunger.bubble.is_none() {
                continue;
            }

            hunger.value = HUNGER_FULL_VALUE;
            commands.entity(hunger.bubble.unwrap()).despawn();
            hunger.bubble = None;
        }
    }
}