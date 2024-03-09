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

use crate::animations::AnimationsPlugin;
use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::hitbox::HitboxPlugin;
use crate::growing::GrowingPlugin;
use crate::parents::ParentsPlugin;
use crate::garden::GardenPlugin;



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
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(world::WorldParams {
                width: 1920.0,
                height: 1080.0,
            })
            .insert_resource(ClearColor(Color::rgb(190.0 / 255.0, 143.0 / 255.0, 96.0 / 255.0)));
        app.init_state::<GameState>().add_plugins((
            LoadingPlugin,
            camera::CameraPlugin {
                scaling_mode: camera::CameraScalingMode::FitBoth,
            },
            bevy_progressbar::ProgressBarPlugin,
            MenuPlugin,
            GardenPlugin,
            ActionsPlugin,
            AnimationsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            HitboxPlugin,
            GrowingPlugin,
            ParentsPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
