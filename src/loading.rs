use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
    #[asset(path = "textures/debug_mushroom.png")]
    pub debug_mushroom: Handle<Image>,
    #[asset(path = "textures/parent_placeholder.png")]
    pub placeholder_parent: Handle<Image>,
    
    #[asset(path = "textures/mushrooms/derp/derp_spores.png")]
    pub derp_spores: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/derp_baby_body.png")]
    pub derp_baby_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/derp_child_body.png")]
    pub derp_child_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/derp_teenager_body.png")]
    pub derp_teenager_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/derp_parent_body.png")]
    pub derp_parent_body: Handle<Image>,
}
