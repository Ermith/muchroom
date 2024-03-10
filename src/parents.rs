use std::time::Duration;

use rand::prelude::*;
use bevy::prelude::*;
use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial};

use crate::{
    animations::AnimationBundle,
    child::*,
    growing::Growable,
    highlight::Highlightable,
    hitbox::*,
    loading::*,
    needs::*,
    pulsing::Pulsing,
    GameState
};

pub const MAX_PARENTS: usize = 13;
pub const MIN_PARENT_SPAWN_TIME: f32 = 10.0;
pub const MAX_PARENT_SPAWN_TIME: f32 = 30.0;

// Maybe in future replace with texture size?
pub const PARENT_SIZE: Vec2 = Vec2::new(128.0, 256.0);
pub const PARENT_WALK_SPEED: f32 = 100.0;
/// Gap between parents in the parent waiting queue.
pub const PARENT_GAP: f32 = 10.0;
/// Time after which will parent run out of patience, which results in game over.
pub const PARENT_MAX_PATIENCE: f32 = 120.0;
/// Score received at max patiance.
pub const PARENT_MAX_PATIENCE_SCORE: f32 = 10.0;
/// Y position of parent spawn.
pub const PARENT_SPAWN_Y: f32 = 400.0;
/// X position of the start of the parent queue.
pub const PARENT_QUEUE_X: f32 = -850.0;

/// How much the bar color wobble tends to return to normalcy
const FLOATY_NORMALCY_BIAS: f32 = 0.015;
/// How much the color of the bar wobbles
const FLOATY_COLOR_SCALE: f32 = 0.03;
/// Color section count of the bar
const BAR_SECTIONS: usize = 200;
/// Height of patience bar in pixels.
const BAR_HEIGHT: f32 = 20.0;
/// Width of the patience bar in pixels.
const BAR_WIDTH: f32 = PARENT_SIZE.x - 20.0;
/// Y offset of patience bar from parent.
const BAR_OFFSET: f32 = 70.0;

#[derive(Clone, Copy, Eq, PartialEq)]
enum ParentState {
    Walking,
    Patient,
    Nervous
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Species {
    Derp,
    Psycho,
    Poser
}

pub struct ParentsPlugin;

#[derive(Component)]
pub struct Parent {
    /// Position of parent in parent queue/
    queue_index: usize,
    patience_timer: Timer,
    state: ParentState,
    species: Species,
    is_changed: bool
}

#[derive(Component)]
#[component(storage = "SparseSet")]
/// Walks to destination. Upon reaching destination this component is removed.
struct Walker {
    destination: Vec2,
}

#[derive(Resource)]
struct ParentSpawnTimer(Timer);

#[derive(Resource, Default)]
struct ParentQueue([bool; MAX_PARENTS]);

impl Plugin for ParentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ParentSpawnTimer(
                Timer::from_seconds(
                    rand::thread_rng().gen_range(MIN_PARENT_SPAWN_TIME..=MAX_PARENT_SPAWN_TIME),
                    TimerMode::Repeating,
                )
            ))
            .init_resource::<ParentQueue>()
            .add_systems(OnEnter(GameState::Playing), cleanup_parent_system)
            .add_systems(OnExit(GameState::Playing), cleanup_parent_system) // better safe than sorry
            .add_systems(Update, (
                handle_random_parent_spawning,
                move_walkers,
                update_patience,
                read_on_drop_events,
                update_parent_animations,
            ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

impl Default for Parent {
    fn default() -> Self {
        Self { 
            queue_index: 0,
            patience_timer: Timer::from_seconds(PARENT_MAX_PATIENCE, TimerMode::Once),
            state: ParentState::Walking,
            species: Species::Derp,
            is_changed: false
        }
    }
}

#[derive(Component, Debug)]
pub struct PatienceBar;

#[derive(Component, Debug)]
pub struct HasPatienceBar(Entity);

fn cleanup_parent_system(
    mut parent_queue: ResMut<ParentQueue>,
    mut parent_spawn_timer: ResMut<ParentSpawnTimer>,
) {
    parent_queue.0 = [false; MAX_PARENTS];
    parent_spawn_timer.0.reset();
}

fn handle_random_parent_spawning(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ParentSpawnTimer>,
    mut parent_queue: ResMut<ParentQueue>,
    mut bar_materials: ResMut<Assets<ProgressBarMaterial>>,
    animation_assets: Res<AnimationAssets>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let available_slots_indices = parent_queue.0.iter().enumerate().filter_map(|(i, &slot)| if !slot { Some(i) } else { None }).collect::<Vec<_>>();
    if available_slots_indices.is_empty() { return; }
    let fraction_empty: f64 = available_slots_indices.len() as f64 / MAX_PARENTS as f64;
    let pick_first = thread_rng().gen_bool(fraction_empty * fraction_empty);
    let picked_slot = if pick_first {
        available_slots_indices.first().copied().unwrap()
    } else {
        available_slots_indices.choose(&mut rand::thread_rng()).copied().unwrap()
    };

    timer.0.tick(time.delta());
    if timer.0.just_finished() || parent_queue.0.iter().all(|&slot| !slot) {
        timer.0.set_duration(Duration::from_secs_f32(
            rand::thread_rng().gen_range(MIN_PARENT_SPAWN_TIME..=MAX_PARENT_SPAWN_TIME)
        ));
        timer.0.reset();

        let (camera, camera_transform) = camera.single();
        let spawn_x = camera.viewport_to_world_2d(camera_transform, Vec2::new(-PARENT_SIZE.x, 0.0)).unwrap().x;
        let spawn_pos = Vec3::new(
            spawn_x,
            PARENT_SPAWN_Y,
            50.0
        );
        parent_queue.0[picked_slot] = true;

        let mut floaty_shift = Vec3::new(0.0, 0.0, 0.0);
        let bar_colors = (0..BAR_SECTIONS).map(|i| {
            let p = i as f32 / BAR_SECTIONS as f32;
            let r = 0.6 + (1.0 - p) * 0.4 + floaty_shift.x;
            let g = 0.3 + p * 0.4 + floaty_shift.y;
            let b = 0.2 + floaty_shift.z;

            floaty_shift += Vec3::new(
                rand::thread_rng().gen_range(-1.0..=1.0) * FLOATY_COLOR_SCALE - floaty_shift.x * FLOATY_NORMALCY_BIAS,
                rand::thread_rng().gen_range(-1.0..=1.0) * FLOATY_COLOR_SCALE - floaty_shift.y * FLOATY_NORMALCY_BIAS,
                rand::thread_rng().gen_range(-1.0..=1.0) * FLOATY_COLOR_SCALE - floaty_shift.z * FLOATY_NORMALCY_BIAS,
            );

            (1, Color::rgb(r, g, b))
        }).collect::<Vec<_>>();
        let mut bar_bar = ProgressBar::new(bar_colors);
        bar_bar.set_progress(1.0);
        let bar_style = Style {
            position_type: PositionType::Absolute,
            width: Val::Vw((BAR_WIDTH - 4.0) / crate::WINDOW_WIDTH * 100.0),
            height: Val::Vh((BAR_HEIGHT - 4.0) / crate::WINDOW_HEIGHT * 100.0),
            ..bevy_utils::default()
        };

        let bar_container = commands.spawn((NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Vw(BAR_WIDTH / crate::WINDOW_WIDTH * 100.0),
                    height: Val::Vh(BAR_HEIGHT / crate::WINDOW_HEIGHT * 100.0),
                    top: Val::Px(BAR_OFFSET),
                    left: Val::Px(-10000.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..bevy_utils::default()
                },
                border_color: Color::rgb(0.4, 0.2, 0.2).into(),
                background_color: Color::rgb(0.2, 0.1, 0.1).into(),
                ..default()
            },
            crate::GameObject,
        )).id();

        let patience_bar = commands.spawn((
            ProgressBarBundle::new(
                bar_style,
                bar_bar,
                &mut bar_materials,
            ),
            PatienceBar,
            crate::GameObject,
        )).id();

        commands.get_entity(bar_container).unwrap().add_child(patience_bar);

        let species = match rand::thread_rng().next_u32() % 3 {
            0 => Species::Derp,
            1 => Species::Psycho,
            2 => Species::Poser,
            _ => Species::Derp
        };

        spawn_parent(&mut commands, &animation_assets, species, spawn_pos, picked_slot, patience_bar);
    }
}

fn spawn_parent(
    commands: &mut Commands,
    animation_assets: &AnimationAssets,
    species: Species,
    spawn_pos: Vec3,
    slot: usize,
    patience_bar: Entity
) {
    let parent = commands.spawn((
        Parent {
            queue_index: slot,
            species,
            ..default()
        },
        SpatialBundle {
            transform: Transform::from_translation(spawn_pos),
            ..default()
        },
        Walker {
            destination: Vec2::new(PARENT_QUEUE_X, PARENT_SPAWN_Y)
                + Vec2::X * ((PARENT_SIZE.x + PARENT_GAP) * slot as f32),
        },
        InLayers::new_single(Layer::Parent),
        HasPatienceBar(patience_bar),
        crate::GameObject,
        Highlightable::default(),
    )).id();

    spawn_animations(parent, commands, animation_assets, species, ParentState::Walking);
}

fn spawn_animations(
    parent: Entity,
    commands: &mut Commands,
    animation_assets: &AnimationAssets,
    species: Species,
    state: ParentState
) {
    let animation_body = get_animation(animation_assets, species, state, false);
    let animation_eyes = get_animation(animation_assets, species, state, true);
    
    let animation_body = commands.spawn((
        AnimationBundle::new(animation_body, 0.15, 0.2, 0.0),
        )).id();

    let animation_eyes = commands.spawn((
        AnimationBundle::new(animation_eyes, 0.15, 0.2, 0.1),
        )).id();

    commands.entity(parent).push_children(&[animation_body, animation_eyes]);
}

fn get_animation(animation_assets: &AnimationAssets, species: Species, state: ParentState, eyes: bool) -> Vec<Handle<Image>> {
    match species {
        Species::Derp => get_derp_animation(animation_assets, state, eyes),
        Species::Psycho => get_psycho_animation(animation_assets, state, eyes),
        Species::Poser => get_poser_animation(animation_assets, state, eyes)
    }
}

fn get_derp_animation(animation_assets: &AnimationAssets, state: ParentState, eyes: bool) -> Vec<Handle<Image>> {
    match state {
        ParentState::Walking =>
        if eyes {
            animation_assets.derp_parent_walking_eyes.clone()
        } else {
            animation_assets.derp_parent_walking_body.clone()
        },

        ParentState::Patient =>
        if eyes {
            animation_assets.derp_parent_patient_eyes.clone()
        } else {
            animation_assets.derp_parent_patient_body.clone()
        },

        ParentState::Nervous =>
        if eyes {
            animation_assets.derp_parent_nervous_eyes.clone()
        } else {
            animation_assets.derp_parent_nervous_body.clone()
        },
    }
}

fn get_psycho_animation(animation_assets: &AnimationAssets, state: ParentState, eyes: bool) -> Vec<Handle<Image>> {
    match state {
        ParentState::Walking =>
        if eyes {
            animation_assets.psycho_parent_walking_eyes.clone()
        } else {
            animation_assets.psycho_parent_walking_body.clone()
        },

        ParentState::Patient =>
        if eyes {
            animation_assets.psycho_parent_patient_eyes.clone()
        } else {
            animation_assets.psycho_parent_patient_body.clone()
        },

        ParentState::Nervous =>
        if eyes {
            animation_assets.psycho_parent_nervous_eyes.clone()
        } else {
            animation_assets.psycho_parent_nervous_body.clone()
        },
    }
}

fn get_poser_animation(animation_assets: &AnimationAssets, state: ParentState, eyes: bool) -> Vec<Handle<Image>> {
    match state {
        ParentState::Walking =>
        if eyes {
            animation_assets.poser_parent_walking_eyes.clone()
        } else {
            animation_assets.poser_parent_walking_body.clone()
        },

        ParentState::Patient =>
        if eyes {
            animation_assets.poser_parent_patient_eyes.clone()
        } else {
            animation_assets.poser_parent_patient_body.clone()
        },

        ParentState::Nervous =>
        if eyes {
            animation_assets.poser_parent_nervous_eyes.clone()
        } else {
            animation_assets.poser_parent_nervous_body.clone()
        },
    }
}

fn update_parent_animations(
    mut commands: Commands,
    mut parent_query: Query<(Entity, &mut Parent)>,
    children_query: Query<&Children>,
    animation_assets: Res<AnimationAssets>
) {
    for (entity, mut parent) in parent_query.iter_mut() {
        if !parent.is_changed { continue; }

        for child in children_query.iter_descendants(entity) {
            if let Some(mut child_commands) = commands.get_entity(child) {
                child_commands.despawn();
            }
        }

        spawn_animations(entity, &mut commands, &animation_assets, parent.species, parent.state);
        parent.is_changed = false;
    }
}

fn move_walkers(
    mut commands: Commands, 
    time: Res<Time>, 
    textures: Res<TextureAssets>,
    mut query: Query<(Entity, &mut Parent, &mut Transform, &Walker)>
) {
    for (entity, mut parent, mut transform, walker) in &mut query {
        let direction = (walker.destination.extend(0.0) - transform.translation).normalize();
        transform.translation += direction * PARENT_WALK_SPEED * time.delta_seconds();

        if Vec2::distance(transform.translation.xy(), walker.destination) < PARENT_WALK_SPEED * time.delta_seconds() {
            transform.translation = walker.destination.extend(0.0);
            commands.entity(entity).remove::<Walker>();

            commands.entity(entity).insert(Hitbox::new_centered(Vec2::splat(128.0)));

            let spores_texture = match parent.species {
                Species::Derp => textures.derp_spores.clone(),
                Species::Psycho => textures.psycho_spores.clone(),
                Species::Poser => textures.poser_spores.clone()
            };

            let mut spore_transform = *transform;
            spore_transform.translation += Vec3::new(0.0, 0.0, 1.5);
            let mut anim =AnimationBundle::new_with_size(vec![ spores_texture.clone() ], 0.15, CHILD_HITBOX_SIZE, 0.15);
            anim.sprite_sheet.transform = spore_transform;

            let child_entity = commands.spawn((
                anim,
                Hitbox::new_centered(Vec2::splat(CHILD_HITBOX_SIZE)),
                EmitsCollisions::default(),
                Draggable {
                    must_be_contained_in: Some(Layer::Garden.into()),
                    ..default()
                },
                InLayers::new_single(Layer::Child),
                Child {
                    parent_entity: entity,
                    species: parent.species
                },
                Needs::default(),
                DropBlocker,
                crate::GameObject,
                Pulsing,
            )).id();


            
            let mut anim = AnimationBundle::new_with_size(vec![ textures.nothing.clone() ], 0.1, CHILD_HITBOX_SIZE, 0.6);
            anim.sprite_sheet.transform = Transform::from_translation(Vec3::new(0.0, 50.0, 0.1));
            let eyes_visual = commands.spawn((
                EyesVisual,
                anim
            )).id();

            commands.entity(child_entity).add_child(eyes_visual);

            parent.state = ParentState::Patient;
            parent.is_changed = true;
        }
    }
}

fn update_patience(
    time: Res<Time>,
    mut query: Query<(&mut Parent, &Transform, Option<&HasPatienceBar>, Option<&Walker>)>,
    mut bars: Query<(&mut ProgressBar, &bevy::prelude::Parent), (With<PatienceBar>, Without<Parent>)>,
    mut styles: Query<&mut Style, (Without<Parent>, Without<ProgressBar>)>,
    camera: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Parent>, Without<PatienceBar>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // moving the bar really shouldn't be here but I'm too lazy to refactor it
    let (camera, camera_trans) = camera.single();
    for (mut parent, trans, patience_bar, walker) in &mut query {
        if let Some(patience_bar) = patience_bar {
            if let Ok((mut bar, ui_parent)) = bars.get_mut(patience_bar.0) {
                bar.set_progress(parent.patience_timer.fraction_remaining());
                while bar.sections.len() > (parent.patience_timer.fraction_remaining() * BAR_SECTIONS as f32) as usize {
                    bar.sections.pop();
                }
                let bar_trans = trans.translation - Vec3::Y * BAR_OFFSET - Vec3::new(BAR_WIDTH / 2.0, BAR_HEIGHT / 2.0, 0.0);
                let bar_pos = camera.world_to_viewport(camera_trans, bar_trans).unwrap();
                let mut style = styles.get_mut(ui_parent.get()).unwrap();
                style.left = Val::Px(bar_pos.x);
                style.top = Val::Px(bar_pos.y);
            }
        }

        // no impatience until you arrive
        if walker.is_some() { continue };

        parent.patience_timer.tick(time.delta());

        if parent.patience_timer.remaining() < parent.patience_timer.duration() / 2 && parent.state != ParentState::Nervous {
            parent.state = ParentState::Nervous;
            parent.is_changed = true;
        }

        if parent.patience_timer.just_finished() {
            next_state.set(GameState::GameOver);
        }
    }
}

fn read_on_drop_events(
    mut commands: Commands,
    mut parent_queue: ResMut<ParentQueue>,
    mut events: EventReader<DropEvent>,
    child_query: Query<&Child, With<Growable>>,
    parent_query: Query<(&Parent, Option<&HasPatienceBar>)>,
    bars: Query<(&ProgressBar, &bevy::prelude::Parent)>,
    mut score: ResMut<crate::score::Score>,
) {
    for event in events.read() {
        if let Ok(children) = child_query.get(event.dropped_entity) {
            if children.parent_entity.index() != event.dropped_on_entity.index() {
                continue;
            }
            let (parent, maybe_bar) = parent_query.get(children.parent_entity).unwrap();

            let remains = parent.patience_timer.remaining().as_secs_f32();
            let max_score_mult = remains / PARENT_MAX_PATIENCE;

            parent_queue.0[parent.queue_index] = false;
            commands.entity(children.parent_entity).despawn_recursive();
            commands.entity(event.dropped_entity).despawn_recursive();

            if let Some(bar) = maybe_bar {
                let (_, bar_parent_border) = bars.get(bar.0).unwrap();
                commands.entity(bar_parent_border.get()).despawn_recursive();
                commands.entity(bar.0).despawn_recursive();
            }

            score.0 += (PARENT_MAX_PATIENCE_SCORE * max_score_mult) as i32;
            score.1 += 1;
        }
    }
}