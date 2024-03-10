use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Default)]
pub struct Score(pub i32, pub i32);

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Score>()
            .add_systems(OnEnter(crate::GameState::Playing), (setup_score, setup_score_ui))
            .add_systems(Update, update_score_ui)
            .add_systems(OnExit(crate::GameState::Playing), cleanup_score_ui);
    }
}

fn setup_score(mut score: ResMut<Score>) {
    score.0 = 0;
    score.1 = 0;
}

#[derive(Component, Debug)]
struct ScoreText;

fn setup_score_ui(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Score: 0".to_string(),
                        style: TextStyle {
                            font_size: 40.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(40.0),
                top: Val::Px(10.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ScoreText,
    ));
}

fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Score: {}", score.0);
    }
}

fn cleanup_score_ui(mut commands: Commands, query: Query<Entity, With<ScoreText>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}