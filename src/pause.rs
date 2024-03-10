use bevy::prelude::*;

use crate::menu::click_music_button;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PausedState {
    #[default]
    Unpaused,
    Paused,
}

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PausedState>()
            .add_systems(OnEnter(crate::GameState::Playing), pause_system_init)
            .add_systems(OnExit(crate::GameState::Playing), pause_system_init)
            .add_systems(Update, pause_toggle_system.run_if(in_state(crate::GameState::Playing)))
            .add_systems(OnEnter(PausedState::Paused), build_pause_menu)
            .add_systems(Update, (pause_menu_system, click_music_button).run_if(in_state(crate::GameState::Playing).and_then(in_state(PausedState::Paused))))
            .add_systems(OnExit(PausedState::Paused), remove_pause_menu)
            ;
    }
}

// for handy copy-pasting:
// .run_if(in_state(crate::PausedState::Unpaused))
// .and_then(in_state(crate::PausedState::Unpaused))

fn pause_system_init(
    mut state: ResMut<NextState<PausedState>>,
) {
    state.set(PausedState::Unpaused);
}

fn pause_toggle_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<PausedState>>,
    mut next_state: ResMut<NextState<PausedState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_state.set(match state.get() {
            PausedState::Unpaused => PausedState::Paused,
            PausedState::Paused => PausedState::Unpaused,
        })
    }
}

#[derive(Component, Debug)]
struct PauseMenu;

#[derive(Component, Debug)]
pub enum ButtonAction {
    ChangePausedState(PausedState),
    ChangeGameState(crate::GameState),
}

fn build_pause_menu(
    mut commands: Commands,
    textures: Res<crate::loading::TextureAssets>,
) {
    // darkness overlay to make the game look darker
    commands.spawn((
        TextBundle { // don't question why this is text, shhhh, it's gonna be ok
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
            ..Default::default()
        },
        PauseMenu,
    ));

    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
        },
        PauseMenu,
    ))
    .with_children(|children| {
        children.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Paused".to_string(),
                        style: TextStyle {
                            font_size: 80.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
        children.spawn(NodeBundle {
            style: Style {
                height: Val::Px(3.0),
                ..default()
            },
            ..default()
        });
        children.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Press Escape to unpause".to_string(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
        children.spawn(NodeBundle {
            style: Style {
                height: Val::Px(30.0),
                ..default()
            },
            ..default()
        });
        children.spawn(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "TODO: HELP TEXT GOES HERE".to_string(),
                        style: TextStyle {
                            font_size: 16.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        });
        children.spawn(NodeBundle {
            style: Style {
                height: Val::Px(30.0),
                ..default()
            },
            ..default()
        });
        let button_colors = crate::menu::ButtonColors::default();
        children.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(140.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: button_colors.clone().normal.into(),
                ..default()
            },
            button_colors.clone(),
            ButtonAction::ChangePausedState(PausedState::Unpaused),
        )).with_children(|children| {
            children.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Resume".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
        
        children.spawn(NodeBundle {
            style: Style {
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        });
        children.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(140.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: button_colors.clone().normal.into(),
                ..default()
            },
            button_colors.clone(),
            ButtonAction::ChangeGameState(crate::GameState::Menu),
        )).with_children(|children| {
            children.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Menu".to_string(),
                            style: TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                    ],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    });

    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::FlexEnd,
                top: Val::Px(25.),
                right: Val::Px(25.),
                width: Val::Percent(100.),
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
        PauseMenu,
    ))
    .with_children(|children| {
        // mute button
        children.spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(50.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            crate::menu::ButtonColors {
                normal: Color::rgba(0.0, 0.0, 0.0, 0.0),
                hovered: Color::rgba(0.0, 0.0, 0.0, 0.0),
            },
            crate::menu::MusicAction::Toggle,
        )).with_children(|parent| {
            parent.spawn(ImageBundle {
                image: textures.music_icon.clone().into(),
                style: Style {
                    width: Val::Px(64.),
                    ..default()
                },
                ..default()
            });
        });
    });
}

fn remove_pause_menu(
    mut commands: Commands,
    query: Query<Entity, With<PauseMenu>>,
) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn pause_menu_system(
    mut next_state: ResMut<NextState<crate::GameState>>,
    mut next_paused_state: ResMut<NextState<PausedState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &crate::menu::ButtonColors,
            Option<&ButtonAction>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, button_colors, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match button_action {
                    Some(ButtonAction::ChangePausedState(paused_state)) => {
                        next_paused_state.set(paused_state.clone());
                    }
                    Some(ButtonAction::ChangeGameState(game_state)) => {
                        next_state.set(game_state.clone());
                    }
                    None => {}
                }
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}