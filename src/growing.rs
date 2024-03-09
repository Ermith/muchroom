use bevy::prelude::*;

use crate::{loading::TextureAssets, GameState};

pub const GROW_SPEED: f32 = 1.0;
pub const GROW_DURATION: f32 = 5.0;
pub const GROW_STAGES: usize = 5;

pub struct GrowingPlugin;

#[derive(Component, Default)]
pub struct Growable {
    progress: f32,
    stage: usize,
    // TODO: add eyes as second item in pair?
    textures: [(Handle<Image>, ); GROW_STAGES],
}

impl Growable {
    pub fn derp(textures: &TextureAssets) -> Self {
        Self {
            textures: [
                (textures.derp_spores.clone(), ),
                (textures.derp_baby_body.clone(), ),
                (textures.derp_child_body.clone(), ),
                (textures.derp_teenager_body.clone(), ),
                (textures.derp_parent_body.clone(), ),
            ],
            ..default()
        }
    }
}

impl Plugin for GrowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, progress_grow.run_if(in_state(GameState::Playing)));
    }
}

fn progress_grow(time: Res<Time>, mut query: Query<(&mut Growable, &mut Handle<Image>)>) {
    for (mut growable, mut image) in &mut query {
        if growable.stage == GROW_STAGES - 1 {
            continue;
        }

        growable.progress += time.delta_seconds() * GROW_SPEED;

        if growable.progress >= GROW_DURATION {
            growable.progress -= GROW_DURATION;
            growable.stage += 1;

            *image = growable.textures[growable.stage].0.clone();
        }
    }
}