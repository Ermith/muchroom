use bevy::prelude::*;

use crate::GameState;

const MIN_SCALE: f32 = 0.9;
const MAX_SCALE: f32 = 1.4;
const PULSE_SPEED: f32 = 0.9;

pub struct PulsingPlugin;

/// Adds pulsing effect (scaling up and down) to an entity.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Pulsing;

#[derive(Resource)]
struct GlobalPulse {
    value: f32,
    direction: f32,
}

impl Plugin for PulsingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GlobalPulse>()
            .add_systems(Update, (
                apply_pulse
            ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

impl Default for GlobalPulse {
    fn default() -> Self {
        Self { value: 1.0, direction: 1.0 }
    }
}

fn apply_pulse(
    time: Res<Time>,
    mut global_pulse: ResMut<GlobalPulse>,
    mut query: Query<&mut Transform, With<Pulsing>>,
) {
    global_pulse.value += global_pulse.direction * time.delta_seconds() * PULSE_SPEED;

    if global_pulse.value > MAX_SCALE {
        global_pulse.value = MAX_SCALE;
        global_pulse.direction = -1.0;
    }

    if global_pulse.value < MIN_SCALE {
        global_pulse.value = MIN_SCALE;
        global_pulse.direction = 1.0;
    }

    for mut transform in query.iter_mut() {
        transform.scale = Vec2::splat(global_pulse.value).extend(1.0);
    }
}