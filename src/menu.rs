use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(OnEnter(GameState::GameOver), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu).or_else(in_state(GameState::GameOver))))
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(OnExit(GameState::GameOver), cleanup_menu);
    }
}

#[derive(Component, Clone, Debug)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct Menu;

fn setup_menu(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    game_state: Res<State<GameState>>,
    score: Res<crate::score::Score>,
) {
    info!("menu");

    let game_state = game_state.as_ref().get();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            let button_colors = ButtonColors::default();
            // GAME OVER text
            children.spawn(( 
                TextBundle::from_section(
                    match game_state {
                        GameState::Menu => "THERE IS NOT MUCH ROOM IN THIS DAYCARE",
                        GameState::GameOver => "GAME OVER",
                        _ => unreachable!(),
                    },
                    TextStyle {
                        font_size: 80.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                        ..default()
                    },
                ),
            ));
            if game_state == &GameState::GameOver {
                children.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                });
                let text = if score.0 == 1 {
                    "You managed to take care of only one fungus!".to_string()
                } else {
                    format!("You managed to take care of {} fungi!", score.0)
                };
                children.spawn(( 
                    TextBundle::from_section(
                        text,
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ),
                ));
            }
            // spacing
            children.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(100.0),
                    ..default()
                },
                ..default()
            });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(140.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.clone().normal.into(),
                        ..Default::default()
                    },
                    button_colors.clone(),
                    ChangeState(GameState::Playing),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        match game_state {
                            GameState::Menu => "Play",
                            GameState::GameOver => "Retry",
                            _ => unreachable!(),
                        },
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            if game_state == &GameState::GameOver {
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
                            ..Default::default()
                        },
                        background_color: button_colors.clone().normal.into(),
                        ..Default::default()
                    },
                    button_colors.clone(),
                    ChangeState(GameState::Menu),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Menu",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            }
            if !cfg!(target_arch = "wasm32") {
                children.spawn(NodeBundle {
                    style: Style {
                        height: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                });
                children
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(140.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.clone().normal.into(),
                            ..Default::default()
                        },
                        button_colors.clone(),
                        ExitGame,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Exit",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        ));
                    });
            }
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    bottom: Val::Px(5.),
                    width: Val::Percent(100.),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            Menu,
        ))
        .with_children(|children| {
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        ..default()
                    },
                    OpenLink("https://bevyengine.org"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made with Bevy",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.bevy.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://itch.io/jam/spring-game-jam-matfyz-2024"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Made in 48 hours during Matfyz Spring Game Jam 2024",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
            children
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(170.0),
                            height: Val::Px(50.0),
                            justify_content: JustifyContent::SpaceAround,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(5.)),
                            ..default()
                        },
                        background_color: Color::NONE.into(),
                        ..Default::default()
                    },
                    ButtonColors {
                        normal: Color::NONE,
                        hovered: Color::rgb(0.25, 0.25, 0.25),
                    },
                    OpenLink("https://github.com/Ermith/muchroom"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Open source",
                        TextStyle {
                            font_size: 15.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                    parent.spawn(ImageBundle {
                        image: textures.github.clone().into(),
                        style: Style {
                            width: Val::Px(32.),
                            ..default()
                        },
                        ..default()
                    });
                });
        });
}

#[derive(Component)]
struct ChangeState(GameState);

#[derive(Component)]
struct OpenLink(&'static str);

#[derive(Component)]
struct ExitGame;

fn click_play_button(
    mut next_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &ButtonColors,
            Option<&ChangeState>,
            Option<&OpenLink>,
            Option<&ExitGame>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut exit_event_writer: EventWriter<bevy::app::AppExit>,
) {
    for (interaction, mut color, button_colors, change_state, open_link, exit_game) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                if let Some(state) = change_state {
                    next_state.set(state.0.clone());
                } else if let Some(link) = open_link {
                    if let Err(error) = webbrowser::open(link.0) {
                        warn!("Failed to open link {error:?}");
                    }
                } else if exit_game.is_some() {
                    exit_event_writer.send(bevy::app::AppExit);
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

fn cleanup_menu(mut commands: Commands, menu: Query<Entity, With<Menu>>) {
    for entity in menu.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
