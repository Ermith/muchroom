use bevy::prelude::*;

pub struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_animations);
    }
}


#[derive(Component)]
pub struct Animation {
    pub frames: Vec<Handle<Image>>,
    pub frame_index: usize,
    pub frame_period: f32,
    pub timer: f32,
    pub is_changed: bool
}

impl Animation {
    pub fn new(frames: Vec<Handle<Image>>, frame_period: f32) -> Self {
        Self {
            frames,
            frame_period,
            frame_index: 0_usize,
            timer: 0_f32,
            is_changed: false
        }
    }
}

pub fn update_animations(
    mut animation_query: Query<(&mut Animation, &mut Handle<Image>)>,
    time: Res<Time>
) {
    for (mut animation, mut sprite) in animation_query.iter_mut() {
        animation.timer += time.delta_seconds();

        if animation.timer > animation.frame_period || animation.is_changed {
            animation.timer %= animation.frame_period;

            animation.frame_index = (animation.frame_index + 1) % animation.frames.len();
            *sprite = animation.frames[animation.frame_index].clone();
        }
    }
}