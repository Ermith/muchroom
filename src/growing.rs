use bevy::prelude::*;

pub const GROW_SPEED: f32 = 1.0;
pub const GROW_DURATION: f32 = 5.0;
pub const GROW_FINAL_STAGE: u8 = 2;

pub struct GrowingPlugin;

#[derive(Component, Default)]
pub struct Growable {
    progress: f32,
    stage: u8,
}

impl Plugin for GrowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, progress_grow);
    }
}

fn progress_grow(time: Res<Time>, mut query: Query<&mut Growable>) {
    for mut growable in &mut query {
        if growable.stage == GROW_FINAL_STAGE {
            continue;
        }

        growable.progress += time.delta_seconds() * GROW_SPEED;

        if growable.progress >= GROW_DURATION {
            growable.progress -= GROW_DURATION;
            growable.stage += 1;

            // TODO: change texture/animation
        }
    }
}