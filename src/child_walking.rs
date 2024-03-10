use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::hitbox::{Hitbox, InLayers};

pub const DEFAULT_CHILD_MAX_SPEED: f32 = 100.0;
pub const CHILD_WALKING_CHANGE: f32 = 2.0;

#[derive(Component, Debug)]
pub struct ChildWalking {
    pub max_speed: f32,
    pub velocity: Vec2,
    can_move_next_step: bool,
}

impl Default for ChildWalking {
    fn default() -> Self {
        ChildWalking {
            max_speed: DEFAULT_CHILD_MAX_SPEED,
            velocity: Vec2::ZERO,
            can_move_next_step: true,
        }
    }
}

pub struct ChildWalkingPlugin;

impl Plugin for ChildWalkingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, child_walking_system.run_if(in_state(crate::GameState::Playing).and_then(in_state(crate::PausedState::Unpaused))));
    }
}

fn child_walking_system(
    time: Res<Time>,
    mut query: Query<(Option<&mut ChildWalking>, &mut Transform, &Hitbox, &InLayers)>,
) {
    println!("child_walking_system 1");
    for (walking, _, _, _) in &mut query.iter_mut() {
        let Some(mut walking) = walking else { continue };

        walking.can_move_next_step = true;

        walking.velocity += time.delta_seconds() * Vec2::from_angle(thread_rng().gen_range(0.0..std::f32::consts::PI * 2.0) as f32) * CHILD_WALKING_CHANGE;

        if walking.velocity.length() > walking.max_speed {
            walking.velocity = walking.velocity.normalize() * walking.max_speed;
        }
    }
    println!("child_walking_system 2");
    let mut combinations = query.iter_combinations_mut::<2>();
    while let Some([mut one, mut two]) = combinations.fetch_next() {
        let mut one = &mut one;
        let mut two = &mut two;
        for _ in 0..2 {
            std::mem::swap(&mut one, &mut two);
            let ((Some(ref mut walking), transform, hitbox, layers), (maybe_other_walking, other_transform, other_hitbox, other_layers)) = (&mut one, &two) else { continue };
            if !walking.can_move_next_step {
                continue;
            }

            let mut next_transform = transform.clone();
            next_transform.translation += walking.velocity.extend(0.0);

            let mut other_next_transform = (*other_transform).clone();
            if let Some(other_walking) = maybe_other_walking {
                other_next_transform.translation += other_walking.velocity.extend(0.0);
            }

            let share_layers = layers.intersects(other_layers);
            let overlap_hitboxes = hitbox.intersects(other_hitbox, &next_transform, &*other_transform);
            let overlap_hitboxes_next = hitbox.intersects(other_hitbox, &next_transform, &other_next_transform);

            let is_other_garden = other_layers.contains(crate::hitbox::Layer::Garden);
            let is_contained_in_other = other_hitbox.contains_entirely(hitbox, &*other_transform, &next_transform);

            let bounced_off_other = share_layers && (overlap_hitboxes || overlap_hitboxes_next);
            let bounced_off_garden_border = is_other_garden && !is_contained_in_other;

            if bounced_off_other || bounced_off_garden_border {
                walking.velocity = -walking.velocity;
                walking.can_move_next_step = false;
            }
        }
    }

    println!("child_walking_system 3");
    for (walking, mut transform, _, _) in &mut query {
        if let Some(walking) = walking {
            if walking.can_move_next_step {
                transform.translation += walking.velocity.extend(0.0);
                println!("child_walking_system moving {:?}", walking.velocity);
            }
        }
    }
    println!("child_walking_system 4");
    println!("");
}