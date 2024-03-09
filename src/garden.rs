use bevy::prelude::*;
use crate::{highlight::Highlightable, loading::TextureAssets, GameState};

pub struct GardenPlugin;

impl Plugin for GardenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Playing), setup_garden_background);
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
            Garden,
            crate::hitbox::Hitbox::new(Rect::new(-900.0,  -500.0, 600.0, 270.0)),
            crate::hitbox::InLayers::new_single(crate::hitbox::Layer::Garden),
            crate::GameObject,
            Highlightable {
                enabled: false,
                offset: Vec2::new(-150.0, -115.0),
                ..default()
            }
        ));
}