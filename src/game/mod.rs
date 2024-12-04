use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use squaregg::Squaregg;

use crate::SystemState;

mod squaregg; // internal game board state

mod game_ui; // board with tiles - requires InternalGameState

mod finished;
mod playing;
mod starting;

pub fn game_plugin(app: &mut App) {
    app.init_state::<GameState>()
        .insert_resource(InternalGameState(Squaregg::new()))
        .add_plugins(game_ui::game_ui_plugin)
        .add_systems(OnEnter(SystemState::Game), game_setup)
        .add_systems(OnExit(SystemState::Game), game_cleanup)
        .add_plugins((
            starting::starting_plugin,
            playing::playing_plugin,
            finished::finished_plugin,
        ))
        .add_systems(
            Update,
            go_main_menu.run_if(input_just_pressed(KeyCode::Escape)),
        );
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Starting,
    Playing,
    Finished,
    #[default]
    Disabled,
}

#[derive(Resource)]
pub struct InternalGameState(Squaregg);

fn game_setup(
    commands: Commands,
    mut menu_state: ResMut<NextState<GameState>>,
    mut internal_game_state: ResMut<InternalGameState>,
) {
    menu_state.set(GameState::Starting);
    internal_game_state.0.reset();
}

fn game_cleanup(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Disabled);
}

fn go_main_menu(mut system_state: ResMut<NextState<SystemState>>) {
    system_state.set(SystemState::Menu);
}
