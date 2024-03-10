use bevy::prelude::*;

use crate::GameState;

pub struct PulsingPlugin;

/// Adds pulsing effect (scaling up and down) to an entity.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Pulsing {
    pub speed: f32,
    pub min: f32,
    pub max: f32,
    pub direction: f32,
}

impl Plugin for PulsingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                apply_pulse
            ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

impl Default for Pulsing {
    fn default() -> Self {
        Self { 
            speed: 1.0,
            min: 0.8,
            max: 1.2,
            direction: 1.0,
        }   
    }
}

fn apply_pulse(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Pulsing)>,
) {
    for (mut transform, mut pulsing) in query.iter_mut() {
        transform.scale += Vec2::splat(pulsing.direction * time.delta_seconds() * pulsing.speed).extend(0.0);

        if transform.scale.x > pulsing.max {
            transform.scale = Vec2::splat(pulsing.max).extend(1.0);
            pulsing.direction = -1.0;
        }

        if transform.scale.x < pulsing.min {
            transform.scale = Vec2::splat(pulsing.min).extend(1.0);
            pulsing.direction = 1.0;
        }
    }
}