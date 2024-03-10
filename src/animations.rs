use bevy::{prelude::*, window::WindowTheme};

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

    #[allow(dead_code)]
    pub fn change_frames(&mut self, frames: Vec<Handle<Image>>) {
        self.frames = frames;
        self.timer = 0.0;
        self.is_changed = true;
    }
}

pub fn update_animations(
    mut animation_query: Query<(&mut Animation, &mut Handle<Image>)>,
    time: Res<Time>
) {
    for (mut animation, mut sprite) in animation_query.iter_mut() {
        if animation.frames.len() == 0 { continue; }

        animation.timer += time.delta_seconds();

        if animation.timer > animation.frame_period || animation.is_changed {
            animation.timer %= animation.frame_period;

            animation.frame_index = (animation.frame_index + 1) % animation.frames.len();
            *sprite = animation.frames[animation.frame_index].clone();

            animation.is_changed = false;
        }
    }
}

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation: Animation,
    pub sprite_sheet: SpriteSheetBundle
}

impl AnimationBundle {
    pub fn new(frames: Vec<Handle<Image>>, frame_period: f32, scale: f32, z: f32) -> Self {
        let t = Transform::from_translation(Vec3::new(0.0, 0.0, z)).with_scale(Vec3::new(scale, scale, scale));
        let first = frames[0].clone();

        Self {
            animation: Animation::new(frames, frame_period),
            sprite_sheet: SpriteSheetBundle {
                transform: t,
                texture: first,
                ..default()
            }
        }
    }

    pub fn new_with_size(frames: Vec<Handle<Image>>, frame_period: f32, size: f32, z: f32) -> Self {
        let t = Transform::from_translation(Vec3::new(0.0, 0.0, z));
        let first = frames[0].clone();

        Self {
            animation: Animation::new(frames, frame_period),
            sprite_sheet: SpriteSheetBundle {
                transform: t,
                texture: first,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                ..default()
            }
        }
    }
}