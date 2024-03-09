use bevy::prelude::*;

use crate::{growing::Growable, loading::TextureAssets, GameState};

const HUNGER_DECREASE_RATE: f32 = 1.0;
const HUNGER_FULL_VALUE: f32 = 10.0;
const BUBBLE_OFFSET: Vec2 = Vec2::splat(48.0);

pub struct HungerPlugin;

#[derive(Component)]
pub struct Hunger {
    value: f32,
    bubble: Option<Entity>,
}

impl Plugin for HungerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_hunger_decrease.run_if(in_state(GameState::Playing)));
    }
}

impl Default for Hunger {
    fn default() -> Self {
        Self { value: HUNGER_FULL_VALUE, bubble: None }
    }
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