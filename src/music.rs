use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{loading::AudioAssets, GameState};

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnExit(GameState::Loading), spawn_music);
    }
}

#[derive(Component)]
pub struct MusicPlayer;

#[derive(Resource)]
pub struct MusicAudio(pub Handle<AudioInstance>);

fn spawn_music(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
) {
    let handle = audio
        .play(audio_assets.mushroom_dance.clone())
        .looped()
        .with_volume(0.3)
        .handle();
    commands.insert_resource(MusicAudio(handle));
}