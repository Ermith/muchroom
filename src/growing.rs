use bevy::prelude::*;

use crate::{child::Child, hitbox::{Draggable, DropEvent}, loading::TextureAssets, GameState};

pub const GROW_SPEED: f32 = 1.0;
pub const GROW_DURATION: f32 = 5.0;
pub const GROW_STAGES: usize = 5;

pub struct GrowingPlugin;

#[derive(Component, Default)]
pub struct Growable {
    progress: f32,
    pub stage: usize,
    // TODO: add eyes as second item in pair?
    textures: [(Handle<Image>, ); GROW_STAGES],
    /// Determine if growing is currently stopped.
    pub stopped: bool,
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
        app
            .add_systems(Update, (
                progress_grow,
                read_on_drop_events,
            ).run_if(in_state(GameState::Playing)));
    }
}

fn progress_grow(
    time: Res<Time>,
    mut query: Query<(&mut Growable, &mut Handle<Image>, &mut Draggable, &Child)>
) {
    for (mut growable, mut image, mut draggable, child) in &mut query {
        if growable.stopped || growable.stage == GROW_STAGES - 1 {
            continue;
        }

        growable.progress += time.delta_seconds() * GROW_SPEED;

        if growable.progress >= GROW_DURATION {
            growable.progress -= GROW_DURATION;
            growable.stage += 1;
            *image = growable.textures[growable.stage].0.clone();

            if growable.stage == GROW_STAGES - 1 {
                draggable.special_allowed_entities.push(child.parent_entity);
            }
        }
    }
}

fn read_on_drop_events(
    mut commands: Commands,
    mut events: EventReader<DropEvent>,
    textures: Res<TextureAssets>,
    query: Query<&Child, Without<Growable>>
) {
    for event in events.read() {
        if query.get(event.dropped_entity).is_err() {
            continue;
        }

        commands.entity(event.dropped_entity).insert(Growable::derp(&textures));
    }
}