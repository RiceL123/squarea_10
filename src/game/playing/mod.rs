use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

use super::GameState;

pub fn playing_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), game_playing_setup)
        .add_systems(OnExit(GameState::Playing), despawn_screen::<OnStarting>);
}

// recurisvely despawn onstarting elements on exit
#[derive(Component)]
struct OnStarting;

fn game_playing_setup(mut commands: Commands) {

}
