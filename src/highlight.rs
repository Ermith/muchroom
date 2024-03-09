use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{hitbox::Hitbox, GameState};

const HIGHLIGHT_ALPHA: f32 = 0.3;
const HIGHTLIGHT_Z: f32 = 100.0;

pub struct HighlightPlugin;

#[derive(Component, Default)]
pub struct Highlightable {
    pub enabled: bool,
    pub offset: Vec2,
    pub highlight_entity: Option<Entity>,
}

#[derive(Component)]
struct Highlight;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_highlight_visibility.run_if(in_state(GameState::Playing)));
    }
}

fn handle_highlight_visibility(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut highlightable_query: Query<(&mut Highlightable, &Hitbox, &Transform)>,
) {
    for (mut highlightable, hitbox, transform) in &mut highlightable_query {
        if highlightable.enabled && highlightable.highlight_entity.is_none() {
            highlightable.highlight_entity = Some(commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(hitbox.rect.width(), hitbox.rect.height()))),
                    material: materials.add(Color::GREEN.with_a(HIGHLIGHT_ALPHA)),
                    transform: Transform::from_translation(
                        transform.translation + highlightable.offset.extend(HIGHTLIGHT_Z)
                    ),
                    ..default()
                },
                Highlight,
            )).id());
        }
        
        if !highlightable.enabled && highlightable.highlight_entity.is_some() {
            commands.entity(highlightable.highlight_entity.unwrap()).despawn();
            highlightable.highlight_entity = None;
        }
    }
}