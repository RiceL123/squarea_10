use bevy::prelude::*;

use crate::despawn_screen;

use super::{background, default_text_style, spawn_button, MenuButtonAction, MenuState};

pub fn settings_plugin(app: &mut App) {
    app
    .add_systems(OnEnter(MenuState::Settings), settings_setup)
    .add_systems(OnExit(MenuState::Settings), despawn_screen::<OnSettingsMenuScreen>)
    .insert_resource(GameConfig::default());
}

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Resource, Debug)]
pub struct GameConfig {
    pub tile_size: f32,
    pub tile_gap: f32,
    pub tile_text_color: Color
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { tile_size: 40., tile_gap: 5., tile_text_color: Color::hsl(0., 0., 0.1) }
    }
}

fn settings_setup(mut commands: Commands, asset_server: Res<AssetServer>, config: ResMut<GameConfig>) {
    commands
        .spawn((OnSettingsMenuScreen, background()))
        .with_children(|parent| {
            spawn_button(parent, MenuButtonAction::MainMenu, "Back To Menu");
            parent.spawn(default_text_style("settings"));
            parent.spawn(default_text_style(format!("{:?}", config).to_string().as_str()));
        });
}
