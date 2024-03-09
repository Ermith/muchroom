use bevy::prelude::*;

use crate::{child::Child, growing::Growable, hitbox::*, loading::TextureAssets, GameState};

const HUNGER_DECREASE_RATE: f32 = 1.0;
const HUNGER_FULL_VALUE: f32 = 15.0;
const HUNGER_BUBBLE_OFFSET: Vec2 = Vec2::splat(48.0);
const FOOD_SOURCE_SPAWN_POS: Vec2 = Vec2::new(750.0, 90.0);

const THIRST_DECREASE_RATE: f32 = 1.0;
const THIRST_FULL_VALUE: f32 = 10.0;
const THIRST_BUBBLE_OFFSET: Vec2 = Vec2::new(-48.0, 48.0);
const WATER_SOURCE_SPAWN_POS: Vec2 = Vec2::new(750.0, -250.0);

/// Size of food/water.
const ITEM_SIZE: Vec2 = Vec2::splat(128.0);
/// Size of hunger/thirst bubble.
const BUBBLE_SIZE: Vec2 = Vec2::splat(64.0);
const SOURCE_SIZE: Vec2 = Vec2::splat(128.0);
const HITBOX_SIZE: Vec2 = Vec2::splat(128.0);

pub struct NeedsPlugin;

#[derive(Component)]
pub struct Needs {
    hunger: f32,
    hunger_bubble: Option<Entity>,

    thirst: f32,
    thirst_bubble: Option<Entity>,
}

#[derive(Component)]
struct Food;

#[derive(Component)]
struct Water;

impl Plugin for NeedsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), spawn_bucket)
            .add_systems(Update, (
                handle_needs_decrease,
                read_on_drop_events
            ).run_if(in_state(GameState::Playing)));
    }
}

impl Default for Needs {
    fn default() -> Self {
        Self {
            hunger: HUNGER_FULL_VALUE,
            hunger_bubble: None,

            thirst: THIRST_FULL_VALUE,
            thirst_bubble: None,
        }
    }
}

fn spawn_bucket(mut commands: Commands, textures: Res<TextureAssets>) {
    // spawn food source
    commands.spawn((
        SpriteBundle {
            texture: textures.bucket_full.clone(),
            sprite: Sprite {
                custom_size: Some(SOURCE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(FOOD_SOURCE_SPAWN_POS.extend(1.0)),
            ..default()
        },
    ));

    // spawn food into food source
    commands.spawn((
        SpriteBundle {
            texture: textures.worm.clone(),
            transform: Transform::from_translation(FOOD_SOURCE_SPAWN_POS.extend(-10.0)),
            sprite: Sprite {
                custom_size: Some(ITEM_SIZE),
                ..default()
            },
            ..default()
        },
        Hitbox::new_centered(HITBOX_SIZE),
        InLayers::new_single(Layer::Tool),
        Draggable {
            must_intersect_with: Some(Layer::Child.into()),
            ..default()
        },
        Food,
    ));

    // spawn water source
    commands.spawn((
        SpriteBundle {
            texture: textures.placeholder_water_source.clone(),
            sprite: Sprite {
                custom_size: Some(SOURCE_SIZE),
                ..default()
            },
            transform: Transform::from_translation(WATER_SOURCE_SPAWN_POS.extend(1.0)),
            ..default()
        },
    ));

    // spawn water into water source
    commands.spawn((
        SpriteBundle {
            texture: textures.placeholder_water.clone(),
            transform: Transform::from_translation(WATER_SOURCE_SPAWN_POS.extend(-10.0)),
            sprite: Sprite {
                custom_size: Some(ITEM_SIZE),
                ..default()
            },
            ..default()
        },
        Hitbox::new_centered(HITBOX_SIZE),
        InLayers::new_single(Layer::Tool),
        Draggable {
            must_intersect_with: Some(Layer::Child.into()),
            ..default()
        },
        Water,
    ));
}

fn handle_needs_decrease(
    mut commands: Commands,
    time: Res<Time>, 
    textures: Res<TextureAssets>,
    mut query: Query<(Entity, &mut Needs, &mut Growable)>
) {
    for (entity, mut needs, mut growable) in &mut query {
        needs.hunger -= time.delta_seconds() * HUNGER_DECREASE_RATE;
        needs.thirst -= time.delta_seconds() * THIRST_DECREASE_RATE;

        growable.stopped = needs.hunger < 0.0 || needs.thirst < 0.0;

        if needs.hunger < 0.0 && needs.hunger_bubble.is_none() {
            needs.hunger_bubble = Some(commands.spawn(
                SpriteBundle {
                    texture: textures.bubble_worm.clone(),
                    transform: Transform::from_translation(HUNGER_BUBBLE_OFFSET.extend(1.0)),
                    sprite: Sprite {
                        custom_size: Some(BUBBLE_SIZE),
                        ..default()
                    },
                    ..default()
                },
            ).id());

            commands.entity(entity).add_child(needs.hunger_bubble.unwrap());
            commands.entity(needs.hunger_bubble.unwrap()).set_parent(entity);
        }

        if needs.thirst < 0.0 && needs.thirst_bubble.is_none() {
            needs.thirst_bubble = Some(commands.spawn(
                SpriteBundle {
                    texture: textures.placeholder_thirst_bubble.clone(),
                    transform: Transform::from_translation(THIRST_BUBBLE_OFFSET.extend(1.0)),
                    sprite: Sprite {
                        custom_size: Some(BUBBLE_SIZE),
                        ..default()
                    },
                    ..default()
                },
            ).id());

            commands.entity(entity).add_child(needs.thirst_bubble.unwrap());
            commands.entity(needs.thirst_bubble.unwrap()).set_parent(entity);
        }
    }
}

fn read_on_drop_events(
    mut commands: Commands,
    mut events: EventReader<DropEvent>,
    mut child_query: Query<&mut Needs, With<Child>>,
    mut food_query: Query<&mut Transform, (With<Food>, Without<Water>)>,
    mut water_query: Query<&mut Transform, (With<Water>, Without<Food>)>,
) {
    for event in events.read() {
        if let Ok(mut transform) = food_query.get_mut(event.dropped_entity) {
            transform.translation = FOOD_SOURCE_SPAWN_POS.extend(-10.0);

            let mut needs = child_query.get_mut(event.dropped_on_entity).unwrap();

            if needs.hunger_bubble.is_none() {
                continue;
            }

            needs.hunger = HUNGER_FULL_VALUE;
            commands.entity(needs.hunger_bubble.unwrap()).despawn();
            needs.hunger_bubble = None;
        }

        if let Ok(mut transform) = water_query.get_mut(event.dropped_entity) {
            transform.translation = WATER_SOURCE_SPAWN_POS.extend(-10.0);

            let mut needs = child_query.get_mut(event.dropped_on_entity).unwrap();

            if needs.thirst_bubble.is_none() {
                continue;
            }

            needs.thirst = THIRST_FULL_VALUE;
            commands.entity(needs.thirst_bubble.unwrap()).despawn();
            needs.thirst_bubble = None;
        }
    }
}