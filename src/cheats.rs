use bevy::prelude::*;

pub struct CheatsPlugin;

impl Plugin for CheatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, cheats_system);
    }
}

fn cheats_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<crate::GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        game_state.set(crate::GameState::Playing);
    }
    if keyboard_input.just_pressed(KeyCode::F2) {
        game_state.set(crate::GameState::Menu);
    }
    if keyboard_input.just_pressed(KeyCode::F3) {
        game_state.set(crate::GameState::GameOver);
    }
}