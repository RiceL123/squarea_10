use bevy::prelude::*;

use crate::{
    despawn_screen,
    game::{
        playing::TilesPoppedEvent,
        squaregg::{COLS, ROWS},
        GameState, InternalGameState,
    },
    menu::settings::GameConfig,
    SystemState,
};

pub fn score_plugin(app: &mut App) {
    app.add_systems(OnEnter(SystemState::Game), score_setup)
        // .add_systems(Update, update_score.run_if(in_state(GameState::Playing)));
        .add_systems(OnExit(SystemState::Game), despawn_screen::<OnScoreBoard>)
        .observe(update_score);
}

#[derive(Component)]
struct OnScoreBoard;

#[derive(Component)]
struct ScoreText;

fn score_setup(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        OnScoreBoard,
        ScoreText,
        Text2dBundle {
            text: Text::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        ..Default::default()
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        ..Default::default()
                    },
                ),
            ]),
            transform: Transform {
                translation: Vec3::new(
                    -(COLS as f32 / 2.) * (config.tile_size + config.tile_gap),
                    (ROWS as f32 / 2.) * (config.tile_size + config.tile_gap) + 50.,
                    0.,
                ),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn update_score(
    trigger: Trigger<TilesPoppedEvent>,
    internal_game_state: Res<InternalGameState>,
    mut score_board: Query<&mut Text, With<ScoreText>>,
) {
    if let Ok(mut text) = score_board.get_single_mut() {
        text.sections[1].value = internal_game_state.0.score.to_string();
    }

    // println!("score: {:?}", internal_game_state.0.score);
}
