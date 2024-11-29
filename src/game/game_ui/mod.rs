use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

use super::InternalGameState;

mod board;
mod options;
mod score;
mod timer;

#[derive(Component)]
pub struct OnGameUI;

pub fn game_ui_plugin(app: &mut App) {
    app.add_plugins((
        board::board_plugin,
        score::score_plugin,
        timer::timer_plugin,
        options::options_plugin,
    ))
    .add_systems(OnEnter(SystemState::Game), game_ui_setup)
    .add_systems(OnExit(SystemState::Game), despawn_screen::<OnGameUI>);
}

fn game_ui_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    internal_game_state: Res<InternalGameState>,
) {
    // let icon = asset_server.load("squaregg-chan.png");
    // // Display the logo
    // commands
    //     .spawn((
    //         NodeBundle {
    //             style: Style {
    //                 align_items: AlignItems::Center,
    //                 justify_content: JustifyContent::Center,
    //                 width: Val::Percent(100.0),
    //                 height: Val::Percent(100.0),
    //                 ..default()
    //             },
    //             ..default()
    //         },
    //         OnGameUI,
    //     ))
    //     .with_children(|parent| {
    //         parent.spawn(ImageBundle {
    //             style: Style {
    //                 // This will set the logo to be width, and auto adjust its height
    //                 width: Val::Px(700.0),
    //                 ..default()
    //             },
    //             image: UiImage::new(icon),
    //             ..default()
    //         });

    //         parent.spawn(TextBundle::from_section(
    //             "Game Board!!!",
    //             TextStyle { ..default() },
    //         ));

    //         // internal_game_state.0.board.iter().for_each(|row| {
    //         //     parent.spawn(TextBundle::from_section(
    //         //         format!("{:?}", row),
    //         //         TextStyle { ..default() },
    //         //     ));
    //         // });
    //     });
}
