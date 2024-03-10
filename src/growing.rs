use bevy::{prelude::*, sprite::Anchor};

use crate::{animations::{Animation, AnimationBundle}, child::{BodyVisual, Child, EyesVisual, CHILD_SIZE}, hitbox::{Draggable, DropEvent, Hitbox}, loading::{AnimationAssets, TextureAssets}, pulsing::Pulsing, GameState};
use crate::parents::Species;

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
    pub stopped_by_needs: bool,
    pub stopped_by_psycho: bool
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

    pub fn psycho(textures: &TextureAssets) -> Self {
        Self {
            textures: [
                (textures.psycho_spores.clone(), ),
                (textures.psycho_baby_body.clone(), ),
                (textures.psycho_child_body.clone(), ),
                (textures.psycho_teenager_body.clone(), ),
                (textures.psycho_parent_body.clone(), ),
            ],
            ..default()
        }
    }

    pub fn poser(textures: &TextureAssets) -> Self {
        Self {
            textures: [
                (textures.poser_spores.clone(), ),
                (textures.poser_baby_body.clone(), ),
                (textures.poser_child_body.clone(), ),
                (textures.poser_teenager_body.clone(), ),
                (textures.poser_parent_body.clone(), ),
            ],
            ..default()
        }
    }
}

impl Plugin for GrowingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<HypnoDespawnEvent>()
            .add_systems(Update, (
                progress_grow,
                read_on_drop_events,
                read_hypno_despawn_events,
                update_hypnotism,
                update_child_visual,
            ).run_if(in_state(GameState::Playing).and_then(in_state(crate::PausedState::Unpaused)))
        );
    }
}

fn progress_grow(
    mut commands: Commands,
    animation_assets: Res<AnimationAssets>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Growable, &mut Draggable, &Child, &mut Sprite, &mut Hitbox)>,
) {
    for (entity, mut growable, mut draggable, child, mut sprite, mut hitbox) in &mut query {
        if growable.stopped_by_psycho || growable.stopped_by_needs || growable.stage == GROW_STAGES - 1 {
            continue;
        }

        growable.progress += time.delta_seconds() * GROW_SPEED;

        if growable.progress >= GROW_DURATION {
            growable.progress -= GROW_DURATION;
            growable.stage += 1;

            if growable.stage == 1 {
                hitbox.rect.min.y += CHILD_SIZE / 4.0;
                hitbox.rect.max.y += CHILD_SIZE / 4.0;

                sprite.anchor = Anchor::BottomCenter;
            }

            if growable.stage == GROW_STAGES - 1 {
                draggable.special_allowed_entities.push(child.parent_entity);

                
                if child.species == Species::Psycho {
                    add_hypnotic_behaviour(&mut commands, entity, &animation_assets);
                }
            }
        }
    }
}

fn update_child_visual(
    mut commands: Commands,
    texture_assets: Res<TextureAssets>,
    mut child_query: Query<(Entity, &Child, &Growable, &mut Animation)>,
    children_query: Query<&Children>,
    mut animation_query: Query<&mut Animation, (With<EyesVisual>, Without<Child>)>
) {
    for (entity, mushroom_child, growable, mut animation) in child_query.iter_mut() {
        animation.change_frames(get_child_frames(
            &texture_assets,
            mushroom_child.species,
            growable.stage,
            false
        ));

        // should be only 1
        let children = children_query.get(entity).unwrap();
        for child in children.iter() {
            if let Ok(mut anim) = animation_query.get_mut(*child) {
                anim.change_frames(get_child_frames(
                    &texture_assets,
                    mushroom_child.species,
                    growable.stage,
                    true
                ));
            }

        }
    }
}


fn get_child_frames(texture_assets: &TextureAssets, species: Species, growth: usize, eyes: bool) -> Vec<Handle<Image>> {
    match species {
        Species::Derp => get_child_frames_derp(texture_assets, growth, eyes),
        Species::Psycho => get_child_frames_psycho(texture_assets, growth, eyes),
        Species::Poser => get_child_frames_poser(texture_assets, growth, eyes),
    }
}

fn get_child_frames_derp(texture_assets: &TextureAssets, growth: usize, eyes: bool) -> Vec<Handle<Image>> {
    match growth {
        0 => if eyes { vec![ texture_assets.nothing.clone()] }
                else { vec![ texture_assets.derp_spores.clone()] },
        1 => if eyes { vec![ texture_assets.derp_baby_eyes.clone()] }
                else { vec![ texture_assets.derp_baby_body.clone()] },
        2 => if eyes { vec![ texture_assets.derp_child_eyes.clone()] }
                else { vec![ texture_assets.derp_child_body.clone()] },
        3 => if eyes { vec![ texture_assets.derp_teenager_eyes.clone()] }
                else { vec![ texture_assets.derp_teenager_body.clone()] },
        4 => if eyes { vec![ texture_assets.derp_parent_eyes.clone()] }
                else { vec![ texture_assets.derp_parent_body.clone()] },

        _ => if eyes { vec![ texture_assets.derp_baby_eyes.clone()] }
                else { vec![ texture_assets.derp_baby_body.clone()] },
    }
}

fn get_child_frames_psycho(texture_assets: &TextureAssets, growth: usize, eyes: bool) -> Vec<Handle<Image>> {
    match growth {
        0 => if eyes { vec![ texture_assets.nothing.clone()] }
                else { vec![ texture_assets.psycho_spores.clone()] },
        1 => if eyes { vec![ texture_assets.psycho_baby_eyes.clone()] }
                else { vec![ texture_assets.psycho_baby_body.clone()] },
        2 => if eyes { vec![ texture_assets.psycho_child_eyes.clone()] }
                else { vec![ texture_assets.psycho_child_body.clone()] },
        3 => if eyes { vec![ texture_assets.psycho_teenager_eyes.clone()] }
                else { vec![ texture_assets.psycho_teenager_body.clone()] },
        4 => if eyes { vec![ texture_assets.psycho_parent_eyes.clone()] }
                else { vec![ texture_assets.psycho_parent_body.clone()] },

        _ => if eyes { vec![ texture_assets.psycho_baby_eyes.clone()] }
                else { vec![ texture_assets.psycho_baby_body.clone()] },
    }
}

fn get_child_frames_poser(texture_assets: &TextureAssets, growth: usize, eyes: bool) -> Vec<Handle<Image>> {
    match growth {
        0 => if eyes { vec![ texture_assets.nothing.clone()] }
                else { vec![ texture_assets.poser_spores.clone()] },
        1 => if eyes { vec![ texture_assets.poser_baby_eyes.clone()] }
                else { vec![ texture_assets.poser_baby_body.clone()] },
        2 => if eyes { vec![ texture_assets.poser_child_eyes.clone()] }
                else { vec![ texture_assets.poser_child_body.clone()] },
        3 => if eyes { vec![ texture_assets.poser_teenager_eyes.clone()] }
                else { vec![ texture_assets.poser_teenager_body.clone()] },
        4 => if eyes { vec![ texture_assets.poser_parent_eyes.clone()] }
                else { vec![ texture_assets.poser_parent_body.clone()] },

        _ => if eyes { vec![ texture_assets.poser_baby_eyes.clone()] }
                else { vec![ texture_assets.poser_baby_body.clone()] },
    }
}

fn read_on_drop_events(
    mut commands: Commands,
    mut events: EventReader<DropEvent>,
    texture_assets: Res<TextureAssets>,
    mut query: Query<(&Child, &mut Transform), Without<Growable>>
) {
    for event in events.read() {
        if let Ok((child, mut transform)) = query.get_mut(event.dropped_entity) {
            let textures = match child.species {
                Species::Derp => Growable::derp(&texture_assets),
                Species::Psycho => Growable::psycho(&texture_assets),
                Species::Poser => Growable::poser(&texture_assets)
            };

            commands.entity(event.dropped_entity)
                .insert(textures)
                .remove::<Pulsing>();
            transform.scale = Vec3::splat(1.0);
        }
    }
}

#[derive(Component)]
pub struct HypnoBehaviour {
    pub range: f32,
}

#[derive(Event, Debug)]
pub struct HypnoDespawnEvent {
    pub parent: Entity,
}

fn update_hypnotism(
    mut victim_query: Query<(&mut Growable, &Transform)>,
    hypno_query: Query<(&GlobalTransform, &HypnoBehaviour)>
) {
    for (mut victim_growable, _) in victim_query.iter_mut() {
        victim_growable.stopped_by_psycho = false;
    }

    for (transform, hypno_behaviour) in hypno_query.iter() {
        for (mut victim_growable, victim_transform) in victim_query.iter_mut() {
            if (transform.translation() - victim_transform.translation).length() < hypno_behaviour.range {
                victim_growable.stopped_by_psycho = true;
            }
        }
    }
}

fn read_hypno_despawn_events(
    mut commands: Commands,
    mut events: EventReader<HypnoDespawnEvent>,
    query: Query<&Children, With<HypnoBehaviour>>
) {
    for event in events.read() {
        for hypno_child in query.iter_descendants(event.parent) {
            commands.entity(hypno_child).despawn();
        }
    }
}

fn add_hypnotic_behaviour(
    commands: &mut Commands,
    parent: Entity,
    animation_assets: &AnimationAssets,
) {
    let e = commands.spawn((
        HypnoBehaviour { range: 500.0 },
        AnimationBundle::new(animation_assets.hypnotic_effect.clone(), 0.15, 0.5, 1.0)
    )).id();

    commands.entity(parent).add_child(e);
}