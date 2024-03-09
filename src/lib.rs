#![allow(clippy::type_complexity)]

mod animations;
mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod hitbox;
mod growing;
mod parents;
mod garden;
mod child;
mod camera;
mod world;
mod needs;
mod game_object;

use crate::animations::AnimationsPlugin;
use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
// use crate::player::PlayerPlugin;
use crate::hitbox::HitboxPlugin;
use crate::growing::GrowingPlugin;
use crate::parents::ParentsPlugin;
use crate::garden::GardenPlugin;
use crate::needs::NeedsPlugin;
use crate::game_object::GameObjectPlugin;
pub use crate::game_object::GameObject;

pub const WINDOW_WIDTH: f32 = 1920.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    GameOver,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum PausedState {
    #[default]
    Unpaused,
    Paused,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(world::WorldParams {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
            })
            .insert_resource(ClearColor(Color::rgb(190.0 / 255.0, 143.0 / 255.0, 96.0 / 255.0)));
        app
            .init_state::<GameState>()
            .init_state::<PausedState>()
            .add_plugins((
            LoadingPlugin,
            camera::CameraPlugin {
                scaling_mode: camera::CameraScalingMode::FitBoth,
            },
            bevy_progressbar::ProgressBarPlugin,
            GameObjectPlugin,
            MenuPlugin,
            GardenPlugin,
            ActionsPlugin,
            AnimationsPlugin,
            InternalAudioPlugin,
            // PlayerPlugin,
            HitboxPlugin,
            GrowingPlugin,
            ParentsPlugin,
            NeedsPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
