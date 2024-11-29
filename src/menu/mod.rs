use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

pub(crate) mod settings;
mod about;

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(SystemState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_plugins((settings::settings_plugin, about::about_plugin))
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(SystemState::Menu)),
        );
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    About,
    #[default]
    Disabled,
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

#[derive(Component)]
struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Play,
    Settings,
    About,
    MainMenu,
    Quit,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((OnMainMenuScreen, background()))
        .with_children(|parent| {
            spawn_button(parent, MenuButtonAction::Play, "play");
            spawn_button(parent, MenuButtonAction::Settings, "settings");
            spawn_button(parent, MenuButtonAction::About, "about");
        });
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut image, selected) in &mut interaction_query {
        image.color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON,
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON,
            (Interaction::Hovered, None) => HOVERED_BUTTON,
            (Interaction::None, None) => NORMAL_BUTTON,
        }
    }
}


fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<SystemState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(SystemState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::MainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::About => menu_state.set(MenuState::About),
            }
        }
    }
}


pub fn spawn_button(parent: &mut ChildBuilder<'_>, component: impl Component, text: &str) {
    parent
        .spawn((component, button()))
        .with_children(|button| {
            button.spawn(default_text_style(text));
        });
}

fn background() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(40.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..Default::default()
        },
        z_index: ZIndex::Global(50),
        background_color: Color::srgba(0., 0.2, 0.2, 0.5).into(),
        ..Default::default()
    }
}

pub fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(300.0),
            height: Val::Px(80.0),
            border: UiRect::all(Val::Px(5.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(5.)),
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        border_radius: BorderRadius::all(Val::Px(10.)),
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        ..default()
    }
}

fn default_text_style(text: &str) -> TextBundle {
    TextBundle::from_section(text, TextStyle { ..default() })
}