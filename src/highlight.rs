use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::{hitbox::{DragShadow, Draggable, DropEvent, Hitbox, InLayers}, GameState};

const HIGHTLIGHT_Z: f32 = 100.0;
const HIGHLIGHT_COLOR: Color = Color::rgba(0.0, 1.0, 0.0, 0.1);

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
        app.add_systems(Update, (
            handle_highlight_visibility,
            update_highlight_for_draggable,
            read_on_drop_events,
        ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

fn handle_highlight_visibility(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut highlightable_query: Query<(Entity, &mut Highlightable, &Hitbox)>,
) {
    for (entity, mut highlightable, hitbox) in &mut highlightable_query {
        if highlightable.enabled && highlightable.highlight_entity.is_none() {
            highlightable.highlight_entity = Some(commands.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(hitbox.rect.width(), hitbox.rect.height()))),
                    material: materials.add(HIGHLIGHT_COLOR),
                    transform: Transform::from_translation(
                        highlightable.offset.extend(HIGHTLIGHT_Z)
                    ),
                    ..default()
                },
                Highlight,
                crate::GameObject,
            )).id());

            commands.entity(entity).add_child(highlightable.highlight_entity.unwrap());
            commands.entity(highlightable.highlight_entity.unwrap()).set_parent(entity);
        }
        
        if !highlightable.enabled && highlightable.highlight_entity.is_some() {
            commands.entity(highlightable.highlight_entity.unwrap()).despawn();
            highlightable.highlight_entity = None;
        }
    }
}

fn update_highlight_for_draggable(
    drag_shadow_query: Query<&DragShadow>,
    original_entity_query: Query<&Draggable>,
    mut highlitgtable_query: Query<(&mut Highlightable, &InLayers)>
) {
    if let Ok(drag_shadow) = drag_shadow_query.get_single() {
        let draggable = original_entity_query.get(drag_shadow.original_entity).unwrap();

        if draggable.special_allowed_entities.len() > 0 {
            for entity in draggable.special_allowed_entities.iter() {
                if let Ok((mut highlightable, _)) = highlitgtable_query.get_mut(*entity) {
                    highlightable.enabled = true;
                }
            }
        }

        if let Some(desired_layers) = draggable.must_intersect_with {
            for (mut highlightable, layers) in highlitgtable_query.iter_mut() {
                if layers.intersects_layer_set(desired_layers) {
                    highlightable.enabled = true;
                }
            }
        }

        if let Some(desired_layers) = draggable.must_be_contained_in {
            for (mut highlightable, layers) in highlitgtable_query.iter_mut() {
                if layers.intersects_layer_set(desired_layers) {
                    highlightable.enabled = true;
                }
            }
        }
    }
    else {
        for (mut highlightable, _) in highlitgtable_query.iter_mut() {
            highlightable.enabled = false;
        }
    }
}

fn read_on_drop_events(
    mut events: EventReader<DropEvent>,
    mut query: Query<&mut Highlightable>,
) {
    if events.read().any(|_| true) {
        for mut highlightable in query.iter_mut() {
            highlightable.enabled = false;
        }
    }
}