use bevy::prelude::*;

use crate::{
    despawn_screen,
    game::{
        playing::TilesPoppedEvent,
        squaregg::{COLS, ROWS},
        InternalGameState,
    },
    menu::settings::GameConfig,
    SystemState,
};

pub fn score_plugin(app: &mut App) {
    app.add_systems(OnEnter(SystemState::Game), score_setup)
        // .add_systems(Update, update_score.run_if(in_state(GameState::Playing)));
        .add_systems(OnExit(SystemState::Game), despawn_screen::<OnScoreBoard>)
        .add_observer(update_score);
        // .add_systems(Update, update_score);
}

#[derive(Component)]
struct OnScoreBoard;

#[derive(Component)]
struct ScoreText;

fn score_setup(mut commands: Commands, config: Res<GameConfig>) {
    commands
        .spawn((
            OnScoreBoard,
            Text2d::new("Score: "),
            Transform {
                translation: Vec3::new(
                    -(COLS as f32 / 2.) * (config.tile_size + config.tile_gap),
                    (ROWS as f32 / 2.) * (config.tile_size + config.tile_gap) + 50.,
                    0.,
                ),
                ..default()
            },
            // Text2dBundle {
            //     text: Text::from_sections([
            //         TextSection::new(
            //             "Score: ",
            //             TextStyle {
            //                 ..Default::default()
            //             },
            //         ),
            //         TextSection::new(
            //             "0",
            //             TextStyle {
            //                 ..Default::default()
            //             },
            //         ),
            //     ]),
            //     transform: Transform {
            //         translation: Vec3::new(
            //             -(COLS as f32 / 2.) * (config.tile_size + config.tile_gap),
            //             (ROWS as f32 / 2.) * (config.tile_size + config.tile_gap) + 50.,
            //             0.,
            //         ),
            //         ..Default::default()
            //     },
            //     ..Default::default()
            // },
        ))
        .with_child((ScoreText, TextSpan::new("0")));
}

fn update_score(
    // mut ev_reader: EventReader<TilesPoppedEvent>,
    trigger: Trigger<TilesPoppedEvent>,
    internal_game_state: Res<InternalGameState>,
    // mut score_board: Query<&mut Text, With<ScoreText>>,
    score_board: Query<Entity, With<ScoreText>>,
    mut text_writer: Text2dWriter,
) {
    // for _ in ev_reader.read() {
    //     if let Ok(text_entity) = score_board.get_single() {
    //         // text.sections[1].value = internal_game_state.0.score.to_string();
    //         *text_writer.text(text_entity, 0) = internal_game_state.0.score.to_string();
    //     }
    
    //     // println!("score: {:?}", internal_game_state.0.score);
    // }

    if let Ok(text_entity) = score_board.get_single() {
        // text.sections[1].value = internal_game_state.0.score.to_string();
        *text_writer.text(text_entity, 0) = internal_game_state.0.score.to_string();
    }
}


// fn update_score(
//     trigger: Trigger<TilesPoppedEvent>,
//     internal_game_state: Res<InternalGameState>,
//     score_board: Query<Entity, With<ScoreText>>,
//     mut text_writer: Text2dWriter,
// ) {
//     if let Ok(text_entity) = score_board.get_single() {
//         // text.sections[1].value = internal_game_state.0.score.to_string();
//         *text_writer.text(text_entity, 0) = internal_game_state.0.score.to_string();
//     }
// }

