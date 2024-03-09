use bevy::prelude::*;
use crate::{loading::TextureAssets, GameState};

pub struct GardenPlugin;

impl Plugin for GardenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), 
                setup_garden_background.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Default)]
pub struct Garden;

fn setup_garden_background(
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    commands
        .spawn((
            SpriteBundle {
                texture: textures.garden_background.clone(),
                transform: Transform::from_xyz(0., 0., -5.0),
                ..default()
            },
            Garden
        ));
}