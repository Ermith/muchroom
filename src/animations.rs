use bevy::prelude::*;



#[derive(Component)]
pub struct Animation {
    pub frames: Vec<Handle<Image>>,
    pub frame_index: usize,
    pub frame_period: f32,
    pub timer: f32,
}

impl Animation {
    pub fn new(frames: Vec<Handle<Image>>, frame_period: f32) -> Self {
        Self {
            frames,
            frame_period,
            frame_index: 0_usize,
            timer: 0_f32
        }
    }
}

