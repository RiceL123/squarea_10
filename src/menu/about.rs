use ::bevy::prelude::*;

use crate::despawn_screen;

use super::{default_text_style, spawn_button, MenuButtonAction, MenuState};

pub fn about_plugin(app: &mut App) {
    app.add_systems(OnEnter(MenuState::About), about_setup)
        .add_systems(
            OnExit(MenuState::About),
            despawn_screen::<OnAboutMenuScreen>,
        );
}

#[derive(Component)]
struct OnAboutMenuScreen;

fn about_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((OnAboutMenuScreen, background()))
        .with_children(|parent| {
            spawn_button(parent, MenuButtonAction::MainMenu, "Back To Menu");
            parent.spawn(default_text_style(
                "Game by RiceL123!!! WHoooo!! the goat!!",
            ));
            parent.spawn(default_text_style("Art by RiceL123!!! WHoooo!! the goat!!"));
            parent.spawn(default_text_style(
                "Music by RiceL123!!! WHoooo!! the goat!!",
            ));
        });
}

fn background() -> (Node, ZIndex, BackgroundColor) {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(20.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),

            ..Default::default()
        },
        ZIndex(i32::MIN),
        BackgroundColor(Color::srgba(0., 0.2, 0.2, 0.0)),
    )
}
