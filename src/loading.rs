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
                .load_collection::<TextureAssets>()
                
        )
        .init_resource::<AnimationAssets>()
        .init_resource::<TextureAssets>()
        .add_systems(OnEnter(GameState::Loading), init_animation_resource);
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

    // Parent Walking Animations
    //====================================

    // Derp
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_1.png")]
    pub derp_parent_walking_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_2.png")]
    pub derp_parent_walking_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_3.png")]
    pub derp_parent_walking_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_4.png")]
    pub derp_parent_walking_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_5.png")]
    pub derp_parent_walking_body_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_body_6.png")]
    pub derp_parent_walking_body_6: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/walking/derp_parent_walking_eyes.png")]
    pub derp_parent_walking_eyes: Handle<Image>,

    // Psycho
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_body_1.png")]
    pub psycho_parent_walking_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_body_2.png")]
    pub psycho_parent_walking_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_body_3.png")]
    pub psycho_parent_walking_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_body_4.png")]
    pub psycho_parent_walking_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_body_5.png")]
    pub psycho_parent_walking_body_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/walking/psycho_parent_walking_eyes.png")]
    pub psycho_parent_walking_eyes: Handle<Image>,

    // Poser
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_1.png")]
    pub poser_parent_walking_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_2.png")]
    pub poser_parent_walking_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_3.png")]
    pub poser_parent_walking_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_4.png")]
    pub poser_parent_walking_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_5.png")]
    pub poser_parent_walking_body_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_body_6.png")]
    pub poser_parent_walking_body_6: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/walking/poser_parent_walking_eyes.png")]
    pub poser_parent_walking_eyes: Handle<Image>,

    // Parent Patient Animations
    //====================================

    // Derp
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_body.png")]
    pub derp_parent_patient_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_1.png")]
    pub derp_parent_patient_eyes_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_2.png")]
    pub derp_parent_patient_eyes_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_3.png")]
    pub derp_parent_patient_eyes_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_4.png")]
    pub derp_parent_patient_eyes_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_5.png")]
    pub derp_parent_patient_eyes_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/patient/derp_parent_patient_eyes_6.png")]
    pub derp_parent_patient_eyes_6: Handle<Image>,

    // Psycho
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_body.png")]
    pub psycho_parent_patient_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_eyes_1.png")]
    pub psycho_parent_patient_eyes_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_eyes_2.png")]
    pub psycho_parent_patient_eyes_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_eyes_3.png")]
    pub psycho_parent_patient_eyes_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_eyes_4.png")]
    pub psycho_parent_patient_eyes_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/patient/psycho_parent_patient_eyes_5.png")]
    pub psycho_parent_patient_eyes_5: Handle<Image>,

    // Poser
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_1.png")]
    pub poser_parent_patient_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_2.png")]
    pub poser_parent_patient_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_3.png")]
    pub poser_parent_patient_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_4.png")]
    pub poser_parent_patient_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_5.png")]
    pub poser_parent_patient_body_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_body_6.png")]
    pub poser_parent_patient_body_6: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_eyes_1.png")]
    pub poser_parent_patient_eyes_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_eyes_2.png")]
    pub poser_parent_patient_eyes_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_eyes_3.png")]
    pub poser_parent_patient_eyes_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_eyes_4.png")]
    pub poser_parent_patient_eyes_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/patient/poser_parent_patient_eyes_5.png")]
    pub poser_parent_patient_eyes_5: Handle<Image>,

    // Parent Nervous Animations
    //====================================
    
    // Derp
    #[asset(path = "textures/mushrooms/derp/nervous/derp_parent_nervous_body_1.png")]
    pub derp_parent_nervous_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/nervous/derp_parent_nervous_body_2.png")]
    pub derp_parent_nervous_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/nervous/derp_parent_nervous_body_3.png")]
    pub derp_parent_nervous_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/nervous/derp_parent_nervous_body_4.png")]
    pub derp_parent_nervous_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/derp/nervous/derp_parent_nervous_eyes.png")]
    pub derp_parent_nervous_eyes: Handle<Image>,

    // Psycho
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_body.png")]
    pub psycho_parent_nervous_body: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_eyes_1.png")]
    pub psycho_parent_nervous_eyes_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_eyes_2.png")]
    pub psycho_parent_nervous_eyes_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_eyes_3.png")]
    pub psycho_parent_nervous_eyes_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_eyes_4.png")]
    pub psycho_parent_nervous_eyes_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/psycho/nervous/psycho_parent_nervous_eyes_5.png")]
    pub psycho_parent_nervous_eyes_5: Handle<Image>,

    // Poser
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_1.png")]
    pub poser_parent_nervous_body_1: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_2.png")]
    pub poser_parent_nervous_body_2: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_3.png")]
    pub poser_parent_nervous_body_3: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_4.png")]
    pub poser_parent_nervous_body_4: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_5.png")]
    pub poser_parent_nervous_body_5: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_body_6.png")]
    pub poser_parent_nervous_body_6: Handle<Image>,
    #[asset(path = "textures/mushrooms/poser/nervous/poser_parent_nervous_eyes.png")]
    pub poser_parent_nervous_eyes: Handle<Image>,
}

impl Default for TextureAssets {
    fn default() -> Self {
        Self {
            bevy: Default::default(),
            github: Default::default(),
            debug_mushroom: Default::default(),
            placeholder_parent: Default::default(),
            derp_spores: Default::default(),
            derp_baby_body: Default::default(),
            derp_child_body: Default::default(),
            derp_teenager_body: Default::default(),
            derp_parent_body: Default::default(),
            derp_parent_walking_body_1: Default::default(),
            derp_parent_walking_body_2: Default::default(),
            derp_parent_walking_body_3: Default::default(),
            derp_parent_walking_body_4: Default::default(),
            derp_parent_walking_body_5: Default::default(),
            derp_parent_walking_body_6: Default::default(),
            derp_parent_walking_eyes: Default::default(),
            psycho_parent_walking_body_1: Default::default(),
            psycho_parent_walking_body_2: Default::default(),
            psycho_parent_walking_body_3: Default::default(),
            psycho_parent_walking_body_4: Default::default(),
            psycho_parent_walking_body_5: Default::default(),
            psycho_parent_walking_eyes: Default::default(),
            poser_parent_walking_body_1: Default::default(),
            poser_parent_walking_body_2: Default::default(),
            poser_parent_walking_body_3: Default::default(),
            poser_parent_walking_body_4: Default::default(),
            poser_parent_walking_body_5: Default::default(),
            poser_parent_walking_body_6: Default::default(),
            poser_parent_walking_eyes: Default::default(),
            derp_parent_patient_body: Default::default(),
            derp_parent_patient_eyes_1: Default::default(),
            derp_parent_patient_eyes_2: Default::default(),
            derp_parent_patient_eyes_3: Default::default(),
            derp_parent_patient_eyes_4: Default::default(),
            derp_parent_patient_eyes_5: Default::default(),
            derp_parent_patient_eyes_6: Default::default(),
            psycho_parent_patient_body: Default::default(),
            psycho_parent_patient_eyes_1: Default::default(),
            psycho_parent_patient_eyes_2: Default::default(),
            psycho_parent_patient_eyes_3: Default::default(),
            psycho_parent_patient_eyes_4: Default::default(),
            psycho_parent_patient_eyes_5: Default::default(),
            poser_parent_patient_body_1: Default::default(),
            poser_parent_patient_body_2: Default::default(),
            poser_parent_patient_body_3: Default::default(),
            poser_parent_patient_body_4: Default::default(),
            poser_parent_patient_body_5: Default::default(),
            poser_parent_patient_body_6: Default::default(),
            poser_parent_patient_eyes_1: Default::default(),
            poser_parent_patient_eyes_2: Default::default(),
            poser_parent_patient_eyes_3: Default::default(),
            poser_parent_patient_eyes_4: Default::default(),
            poser_parent_patient_eyes_5: Default::default(),
            derp_parent_nervous_body_1: Default::default(),
            derp_parent_nervous_body_2: Default::default(), 
            derp_parent_nervous_body_3: Default::default(),
            derp_parent_nervous_body_4: Default::default(),
            derp_parent_nervous_eyes: Default::default(),
            psycho_parent_nervous_body: Default::default(),
            psycho_parent_nervous_eyes_1: Default::default(),
            psycho_parent_nervous_eyes_2: Default::default(),
            psycho_parent_nervous_eyes_3: Default::default(),
            psycho_parent_nervous_eyes_4: Default::default(),
            psycho_parent_nervous_eyes_5: Default::default(),
            poser_parent_nervous_body_1: Default::default(),
            poser_parent_nervous_body_2: Default::default(),
            poser_parent_nervous_body_3: Default::default(),
            poser_parent_nervous_body_4: Default::default(),
            poser_parent_nervous_body_5: Default::default(),
            poser_parent_nervous_body_6: Default::default(),
            poser_parent_nervous_eyes: Default::default()
        }
    }
}

#[derive(Resource)]
pub struct AnimationAssets {
    // Parent Walking Animations
    pub derp_parent_walking_body: Vec<Handle<Image>>,
    pub psycho_parent_walking_body: Vec<Handle<Image>>,
    pub poser_parent_walking_body: Vec<Handle<Image>>,

    pub derp_parent_walking_eyes: Vec<Handle<Image>>,
    pub psycho_parent_walking_eyes: Vec<Handle<Image>>,
    pub poser_parent_walking_eyes: Vec<Handle<Image>>,

    // Parent Patient Animations
    pub derp_parent_patient_body: Vec<Handle<Image>>,
    pub psycho_parent_patient_body: Vec<Handle<Image>>,
    pub poser_parent_patient_body: Vec<Handle<Image>>,

    pub derp_parent_patient_eyes: Vec<Handle<Image>>,
    pub psycho_parent_patient_eyes: Vec<Handle<Image>>,
    pub poser_parent_patient_eyes: Vec<Handle<Image>>,

    // Parent Nervous Animations
    pub derp_parent_nervous_body: Vec<Handle<Image>>,
    pub psycho_parent_nervous_body: Vec<Handle<Image>>,
    pub poser_parent_nervous_body: Vec<Handle<Image>>,

    pub derp_parent_nervous_eyes: Vec<Handle<Image>>,
    pub psycho_parent_nervous_eyes: Vec<Handle<Image>>,
    pub poser_parent_nervous_eyes: Vec<Handle<Image>>,
}

impl Default for AnimationAssets {
    fn default() -> Self {
        Self { derp_parent_walking_body:Default::default(),
            psycho_parent_walking_body: Default::default(),
            poser_parent_walking_body: Default::default(),
            derp_parent_walking_eyes: Default::default(),
            psycho_parent_walking_eyes: Default::default(),
            poser_parent_walking_eyes: Default::default(),
            derp_parent_patient_body: Default::default(),
            psycho_parent_patient_body: Default::default(),
            poser_parent_patient_body: Default::default(),
            derp_parent_patient_eyes: Default::default(),
            psycho_parent_patient_eyes: Default::default(),
            poser_parent_patient_eyes: Default::default(),
            derp_parent_nervous_body: Default::default(),
            psycho_parent_nervous_body:Default::default(),
            poser_parent_nervous_body: Default::default(),
            derp_parent_nervous_eyes: Default::default(),
            psycho_parent_nervous_eyes: Default::default(),
            poser_parent_nervous_eyes: Default::default()
        }
    }
}

pub fn init_animation_resource(
    mut animation_assets: ResMut<AnimationAssets>,
    texture_assets: Res<TextureAssets>
) {
    // Parent Walking Animations
    //==================================

    // Derp
    let vec = &mut animation_assets.derp_parent_walking_body;
    vec.push(texture_assets.derp_parent_walking_body_1.clone());
    vec.push(texture_assets.derp_parent_walking_body_2.clone());
    vec.push(texture_assets.derp_parent_walking_body_3.clone());
    vec.push(texture_assets.derp_parent_walking_body_4.clone());
    vec.push(texture_assets.derp_parent_walking_body_5.clone());
    vec.push(texture_assets.derp_parent_walking_body_6.clone());
    animation_assets.derp_parent_nervous_eyes.push(
        texture_assets.derp_parent_walking_eyes.clone()
    );

    // Psycho
    let vec = &mut animation_assets.psycho_parent_walking_body;
    vec.push(texture_assets.psycho_parent_walking_body_1.clone());
    vec.push(texture_assets.psycho_parent_walking_body_2.clone());
    vec.push(texture_assets.psycho_parent_walking_body_3.clone());
    vec.push(texture_assets.psycho_parent_walking_body_4.clone());
    vec.push(texture_assets.psycho_parent_walking_body_5.clone());
    animation_assets.psycho_parent_nervous_eyes.push(
        texture_assets.psycho_parent_walking_eyes.clone()
    );

    // Poser
    let vec = &mut animation_assets.poser_parent_walking_body;
    vec.push(texture_assets.poser_parent_walking_body_1.clone());
    vec.push(texture_assets.poser_parent_walking_body_2.clone());
    vec.push(texture_assets.poser_parent_walking_body_3.clone());
    vec.push(texture_assets.poser_parent_walking_body_4.clone());
    vec.push(texture_assets.poser_parent_walking_body_5.clone());
    vec.push(texture_assets.poser_parent_walking_body_6.clone());
    animation_assets.poser_parent_nervous_eyes.push(
        texture_assets.poser_parent_walking_eyes.clone()
    );

    // Parent Patient Animations
    //==================================

    // Derp
    let vec = &mut animation_assets.derp_parent_patient_eyes;
    vec.push(texture_assets.derp_parent_patient_eyes_1.clone());
    vec.push(texture_assets.derp_parent_patient_eyes_2.clone());
    vec.push(texture_assets.derp_parent_patient_eyes_3.clone());
    vec.push(texture_assets.derp_parent_patient_eyes_4.clone());
    vec.push(texture_assets.derp_parent_patient_eyes_5.clone());
    vec.push(texture_assets.derp_parent_patient_eyes_6.clone());
    animation_assets.derp_parent_patient_body.push(
        texture_assets.derp_parent_patient_body.clone()
    );

    // Psycho
    let vec = &mut animation_assets.psycho_parent_patient_eyes;
    vec.push(texture_assets.psycho_parent_patient_eyes_1.clone());
    vec.push(texture_assets.psycho_parent_patient_eyes_2.clone());
    vec.push(texture_assets.psycho_parent_patient_eyes_3.clone());
    vec.push(texture_assets.psycho_parent_patient_eyes_4.clone());
    vec.push(texture_assets.psycho_parent_patient_eyes_5.clone());
    animation_assets.psycho_parent_patient_body.push(
        texture_assets.psycho_parent_patient_body.clone()
    );
    
    // Poser
    let vec = &mut animation_assets.poser_parent_patient_body;
    vec.push(texture_assets.poser_parent_patient_body_1.clone());
    vec.push(texture_assets.poser_parent_patient_body_2.clone());
    vec.push(texture_assets.poser_parent_patient_body_3.clone());
    vec.push(texture_assets.poser_parent_patient_body_4.clone());
    vec.push(texture_assets.poser_parent_patient_body_5.clone());
    vec.push(texture_assets.poser_parent_patient_body_6.clone());

    let vec = &mut animation_assets.poser_parent_patient_eyes;
    vec.push(texture_assets.poser_parent_patient_eyes_1.clone());
    vec.push(texture_assets.poser_parent_patient_eyes_2.clone());
    vec.push(texture_assets.poser_parent_patient_eyes_3.clone());
    vec.push(texture_assets.poser_parent_patient_eyes_4.clone());
    vec.push(texture_assets.poser_parent_patient_eyes_5.clone());
    
    // Parent Patient Animations
    //==================================

    // Derp
    let vec = &mut animation_assets.derp_parent_nervous_body;
    vec.push(texture_assets.derp_parent_nervous_body_1.clone());
    vec.push(texture_assets.derp_parent_nervous_body_2.clone());
    vec.push(texture_assets.derp_parent_nervous_body_3.clone());
    vec.push(texture_assets.derp_parent_nervous_body_4.clone());
    animation_assets.derp_parent_patient_eyes.push(
        texture_assets.derp_parent_nervous_eyes.clone()
    );

    // Psycho
    let vec = &mut animation_assets.psycho_parent_nervous_eyes;
    vec.push(texture_assets.psycho_parent_nervous_eyes_1.clone());
    vec.push(texture_assets.psycho_parent_nervous_eyes_2.clone());
    vec.push(texture_assets.psycho_parent_nervous_eyes_3.clone());
    vec.push(texture_assets.psycho_parent_nervous_eyes_4.clone());
    vec.push(texture_assets.psycho_parent_nervous_eyes_5.clone());
    animation_assets.psycho_parent_nervous_body.push(
        texture_assets.psycho_parent_nervous_body.clone()
    );
    
    // Poser
    let vec = &mut animation_assets.poser_parent_nervous_body;
    vec.push(texture_assets.poser_parent_nervous_body_1.clone());
    vec.push(texture_assets.poser_parent_nervous_body_2.clone());
    vec.push(texture_assets.poser_parent_nervous_body_3.clone());
    vec.push(texture_assets.poser_parent_nervous_body_4.clone());
    vec.push(texture_assets.poser_parent_nervous_body_5.clone());
    vec.push(texture_assets.poser_parent_nervous_body_6.clone());

    animation_assets.poser_parent_nervous_eyes.push(
        texture_assets.poser_parent_nervous_eyes.clone()
    );
}

