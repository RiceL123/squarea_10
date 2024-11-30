use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

use super::{squaregg::{Area, Position}, GameState};

pub fn playing_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), game_playing_setup)
        .add_systems(OnExit(GameState::Playing), despawn_screen::<OnPlaying>);
}

// recurisvely despawn onPlaying elements on exit
#[derive(Component)]
struct OnPlaying;

#[derive(Event)]
pub struct TilesPoppedEvent {
    pub tiles: Vec<Position>
}

fn game_playing_setup(mut commands: Commands) {

}
