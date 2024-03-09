use std::time::Duration;

use rand::prelude::*;
use bevy::prelude::*;
use bevy_progressbar::{ProgressBar, ProgressBarBundle, ProgressBarMaterial};

use crate::{child::Child, growing::Growable, hitbox::*, loading::{AnimationAssets, TextureAssets}, GameState};
use crate::animations::Animation;

pub const MAX_PARENTS: usize = 5;
pub const MIN_PARENT_SPAWN_TIME: f32 = 1.0;
pub const MAX_PARENT_SPAWN_TIME: f32 = 3.0;

// Maybe in future replace with texture size?
pub const PARENT_SIZE: Vec2 = Vec2::new(128.0, 256.0);
pub const PARENT_WALK_SPEED: f32 = 100.0;
/// Distance from parent spawn to the start of parent waiting queue.
pub const PARENT_QUEUE_OFFSET: f32 = 256.0;
/// Gap between parents in the parent waiting queue.
pub const PARENT_GAP: f32 = 10.0;
/// Time after which will parent run out of patience, which results in game over.
pub const PARENT_MAX_PATIENCE: f32 = 120.0;
/// Y position of parent spawn.
pub const PARENT_SPAWN_Y: f32 = 400.0;
/// X position of the start of the parent queue.
pub const PARENT_QUEUE_X: f32 = -950.0;

/// Size of spawned children.
pub const CHILD_SIZE: f32 = 64.0;
/// Size of hitbox of spawned children.
pub const CHILD_HITBOX_SIZE: f32 = 48.0;

/// How much the bar color wobble tends to return to normalcy
const FLOATY_NORMALCY_BIAS: f32 = 0.015;
/// How much the color of the bar wobbles
const FLOATY_COLOR_SCALE: f32 = 0.03;
/// Color section count of the bar
const BAR_SECTIONS: usize = 200;
/// Height of patience bar in pixels.
const BAR_HEIGHT: f32 = 20.0;
/// Y offset of patience bar from parent.
const BAR_OFFSET: f32 = 85.0;

pub struct ParentsPlugin;

#[derive(Component)]
pub struct Parent {
    /// Position of parent in parent queue/
    queue_index: usize,
    patience_timer: Timer,
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
            .add_systems(Update, (
                handle_random_parent_spawning,
                move_walkers,
                update_patience,
                read_on_drop_events,
            ).run_if(in_state(GameState::Playing)));
    }
}

impl Default for Parent {
    fn default() -> Self {
        Self { queue_index: 0, patience_timer: Timer::from_seconds(PARENT_MAX_PATIENCE, TimerMode::Once) }
    }
}

#[derive(Component, Debug)]
pub struct PatienceBar;

#[derive(Component, Debug)]
pub struct HasPatienceBar(Entity);

fn handle_random_parent_spawning(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ParentSpawnTimer>,
    mut parent_queue: ResMut<ParentQueue>,
    textures: Res<TextureAssets>,
    window_query: Query<&Window>,
    mut bar_materials: ResMut<Assets<ProgressBarMaterial>>,
    animation_assets: Res<AnimationAssets>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let avaible_slot = parent_queue.0.iter().position(|&slot| !slot);
    if avaible_slot.is_none() {
        return;
    }
    let avaible_slot = avaible_slot.unwrap();

    timer.0.tick(time.delta());
    if timer.0.just_finished() || parent_queue.0.iter().all(|&slot| !slot) {
        timer.0.set_duration(Duration::from_secs_f32(
            rand::thread_rng().gen_range(MIN_PARENT_SPAWN_TIME..=MAX_PARENT_SPAWN_TIME)
        ));
        timer.0.reset();

        let (camera, camera_transform) = camera.single();
        let window = window_query.single();
        let spawn_x = camera.viewport_to_world_2d(camera_transform, Vec2::new(-PARENT_SIZE.x, 0.0)).unwrap().x;
        let spawn_pos = Vec3::new(
            spawn_x,
            PARENT_SPAWN_Y,
            1.0
        );
        parent_queue.0[avaible_slot] = true;

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
            width: Val::Px(PARENT_SIZE.x - 4.0),
            height: Val::Px(BAR_HEIGHT - 4.0),
            ..bevy_utils::default()
        };

        let bar_container = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Px(PARENT_SIZE.x),
                height: Val::Px(BAR_HEIGHT),
                top: Val::Px(BAR_OFFSET),
                border: UiRect::all(Val::Px(2.)),
                ..bevy_utils::default()
            },
            border_color: Color::rgb(0.4, 0.2, 0.2).into(),
            background_color: Color::rgb(0.2, 0.1, 0.1).into(),
            ..default()
        }).id();

        let patience_bar = commands.spawn((
            ProgressBarBundle::new(
                bar_style,
                bar_bar,
                &mut bar_materials,
            ),
            PatienceBar,
        )).id();

        commands.get_entity(bar_container).unwrap().add_child(patience_bar);

        let mut t = Transform::from_translation(spawn_pos);
        t.scale = Vec3::new(0.1, 0.1, 0.1);
        let parent = commands.spawn((
            Parent {
                queue_index: avaible_slot,
                ..default()
            },
            SpatialBundle {
                transform: Transform::from_translation(spawn_pos),
                ..default()
            },
            Walker {
                destination: Vec2::new(PARENT_QUEUE_X, PARENT_SPAWN_Y)
                    + Vec2::X * (PARENT_QUEUE_OFFSET + (PARENT_SIZE.x + PARENT_GAP) * avaible_slot as f32),
            },
            InLayers::new_single(Layer::Parent),
            HasPatienceBar(patience_bar),
        )).id();

        let animation_body = commands.spawn((
            SpriteSheetBundle {
                transform: Transform::from_scale(Vec3::new(0.2,0.2,0.2)),
                texture: textures.bevy.clone(),
                ..default()
            },
            Animation::new(
                animation_assets.derp_parent_walking_body.clone(), 0.15
        ))).id();

        let animation_eyes = commands.spawn((
            SpriteBundle {
                transform: Transform::from_scale(Vec3::new(0.2,0.2,0.2)),
                texture: textures.bevy.clone(),
                ..default()
            },
            Animation::new(
                animation_assets.derp_parent_walking_eyes.clone(), 0.15,
        ))).id();

        commands.entity(parent).push_children(&[animation_body, animation_eyes]);
    }
}

fn move_walkers(
    mut commands: Commands, 
    time: Res<Time>, 
    textures: Res<TextureAssets>,
    mut query: Query<(Entity, &mut Transform, &Walker)>
) {
    for (entity, mut transform, walker) in &mut query {
        let direction = (walker.destination.extend(0.0) - transform.translation).normalize();
        transform.translation += direction * PARENT_WALK_SPEED * time.delta_seconds();

        if Vec2::distance(transform.translation.xy(), walker.destination) < PARENT_WALK_SPEED * time.delta_seconds() {
            transform.translation = walker.destination.extend(0.0);
            commands.entity(entity).remove::<Walker>();

            commands.entity(entity).insert(Hitbox::new_centered(Vec2::splat(128.0)));

            commands.spawn((
                SpriteSheetBundle {
                    texture: textures.derp_spores.clone(),
                    transform: *transform,
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(CHILD_SIZE)),
                        ..default()
                    },
                    ..default()
                },
                Hitbox::new_centered(Vec2::splat(CHILD_HITBOX_SIZE)),
                EmitsCollisions::default(),
                Draggable {
                    must_be_contained_in: Some(Layer::Garden.into()),
                    ..default()
                },
                InLayers::new_single(Layer::Child),
                Child {
                    parent_entity: entity,
                }
            ));
        }
    }
}

fn update_patience(
    time: Res<Time>,
    mut query: Query<(&mut Parent, &Transform, Option<&HasPatienceBar>)>,
    mut bars: Query<(&mut ProgressBar, &bevy::prelude::Parent), (With<PatienceBar>, Without<Parent>)>,
    mut styles: Query<&mut Style, (Without<Parent>, Without<ProgressBar>)>,
    camera: Query<(&Camera, &GlobalTransform), (With<Camera2d>, Without<Parent>, Without<PatienceBar>)>,
) {
    // moving the bar really shouldn't be here but I'm too lazy to refactor it
    let (camera, camera_trans) = camera.single();
    for (mut parent, trans, patience_bar) in &mut query {
        if let Some(patience_bar) = patience_bar {
            if let Ok((mut bar, ui_parent)) = bars.get_mut(patience_bar.0) {
                bar.set_progress(parent.patience_timer.fraction_remaining());
                while bar.sections.len() > (parent.patience_timer.fraction_remaining() * BAR_SECTIONS as f32) as usize {
                    bar.sections.pop();
                }
                let bar_trans = trans.translation - Vec3::Y * BAR_OFFSET;
                let mut bar_pos = camera.world_to_viewport(camera_trans, bar_trans).unwrap();
                let mut style = styles.get_mut(ui_parent.get()).unwrap();
                if let Val::Px(w) = style.width {
                    bar_pos.x -= w / 2.0;
                }
                if let Val::Px(h) = style.height {
                    bar_pos.y -= h / 2.0;
                }
                style.left = Val::Px(bar_pos.x);
                style.top = Val::Px(bar_pos.y);
            }
        }

        parent.patience_timer.tick(time.delta());

        // TODO: update animation

        if parent.patience_timer.just_finished() {
            // TODO: Game Over
            println!("GAME OVER!");
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
) {
    for event in events.read() {
        if let Ok(children) = child_query.get(event.dropped_entity) {
            if children.parent_entity.index() != event.dropped_on_entity.index() {
                continue;
            }

            let (parent, maybe_bar) = parent_query.get(children.parent_entity).unwrap();

            parent_queue.0[parent.queue_index] = false;
            commands.entity(children.parent_entity).despawn();
            commands.entity(event.dropped_entity).despawn();

            if let Some(bar) = maybe_bar {
                let (_, bar_parent_border) = bars.get(bar.0).unwrap();
                commands.entity(bar_parent_border.get()).despawn();
                commands.entity(bar.0).despawn();
            }
        }
    }
}