use bevy::prelude::*;

use crate::despawn_screen;

use super::{default_text_style, spawn_button, MenuButtonAction, MenuState};

pub fn settings_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::Settings), settings_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            despawn_screen::<OnSettingsMenuScreen>,
        )
        .insert_resource(GameConfig::default());
}

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Resource, Debug)]
pub struct GameConfig {
    pub tile_size: f32,
    pub tile_gap: f32,
    pub tile_text_color: Color,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            tile_size: 40.,
            tile_gap: 5.,
            tile_text_color: Color::hsl(0., 0., 0.1),
        }
    }
}

fn settings_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: ResMut<GameConfig>,
) {
    commands
        .spawn((OnSettingsMenuScreen, background()))
        .with_children(|parent| {
            spawn_button(parent, MenuButtonAction::MainMenu, "Back To Menu");
            parent.spawn(default_text_style("settings"));
            parent.spawn(default_text_style(
                format!("{:?}", config).to_string().as_str(),
            ));
        });
}

fn background() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..Default::default()
        },
        z_index: ZIndex::Global(i32::MIN),
        background_color: Color::srgba(0., 0.2, 0.2, 0.0).into(),
        ..Default::default()
    }
}
