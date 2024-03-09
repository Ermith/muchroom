use bevy::prelude::*;

use crate::GameState;

/// Deleted on leaving Playing state
#[derive(Component, Debug)]
pub struct GameObject;

fn game_object_cleanup_system(
    mut commands: Commands,
    mut query: Query<(Entity, &GameObject)>,
) {
    for (entity, _) in query.iter_mut() {
        commands.entity(entity).despawn_recursive()
    }
}

pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), game_object_cleanup_system);
    }
}