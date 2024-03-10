use bevy::prelude::*;

use crate::GameState;

/// Time which takes to scale difficulties from start to end values.
const DIFFICULTY_SCALING_TIME: f32 = 10.0 * 60.0;

pub const START_PATIENCE: f32 = 120.0;
const END_PATIENCE: f32 = 35.0;

pub const START_PARENT_SPAWN_TIME: f32 = 20.0;
const END_PARENT_SPAWN_TIME: f32 = 5.0;

pub struct DifficultyPlugin;

#[derive(Resource)]
pub struct Difficulty {
    /// Elapsed time from the start of the game in seconds.
    elapsed_time: f32,
    /// Patience which will be asigned to parents upon spawning.
    pub parent_patience: f32,
    // Time until next parent spawns.
    pub parent_spawn_time: f32,
}

impl Plugin for DifficultyPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Difficulty>()
            .add_systems(OnEnter(GameState::Playing), reset_difficulty)
            .add_systems(Update, (
                update_difficulty,
            ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

impl Default for Difficulty {
    fn default() -> Self {
        Self { 
            elapsed_time: 0.0,
            parent_patience: START_PATIENCE,
            parent_spawn_time: START_PARENT_SPAWN_TIME,
        }
    }
}

fn reset_difficulty(
    mut difficulty: ResMut<Difficulty>,
) {
    *difficulty = Difficulty::default();
}

fn update_difficulty(
    time: Res<Time>,
    mut difficulty: ResMut<Difficulty>,
) {
    difficulty.elapsed_time += time.delta_seconds();

    if difficulty.elapsed_time > DIFFICULTY_SCALING_TIME {
        difficulty.elapsed_time = DIFFICULTY_SCALING_TIME;
    }
    let percentage = difficulty.elapsed_time / DIFFICULTY_SCALING_TIME;

    difficulty.parent_patience = (END_PATIENCE - START_PATIENCE) * percentage + START_PATIENCE;
    difficulty.parent_spawn_time = (END_PARENT_SPAWN_TIME - START_PARENT_SPAWN_TIME) * percentage + START_PARENT_SPAWN_TIME;
}

/*

grow time: GROW_DURATION * GROW_SPEED * GROW_STAGES (25)

nejmelnsi stihnutelnej grow - 35

min range 35-70

start range 60-120



*/