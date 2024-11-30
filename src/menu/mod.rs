use bevy::prelude::*;
use selection_option::SelectionButton;

use crate::{despawn_screen, SystemState};

mod about;
mod selection_option;
pub(crate) mod settings;

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(Startup, load_images)
        .add_systems(OnEnter(SystemState::Menu), menu_setup)
        .add_systems(
            OnExit(SystemState::Menu),
            despawn_screen::<OnMainMenuScreen>,
        )
        .add_systems(OnEnter(MenuState::Loading), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_plugins((
            settings::settings_plugin,
            about::about_plugin,
            selection_option::selection_option,
        ))
        .add_systems(
            Update,
            menu_intro_animation.run_if(in_state(MenuState::IntroSequence)),
        )
        .add_systems(
            Update,
            check_squarea_loaded.run_if(in_state(MenuState::Loading)),
        )
        .add_systems(Update, (menu_action).run_if(in_state(SystemState::Menu)));
}

#[derive(Resource)]
pub struct MenuImages {
    // egg: Handle<Image>,
    squarea_chan: Handle<Image>,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Loading,
    IntroSequence,
    Main,
    Settings,
    About,
    #[default]
    Disabled,
}

#[derive(Resource)]
pub struct MenuIntroSequenceTimer(Timer);

fn load_images(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MenuImages {
        squarea_chan: asset_server.load("squaregg-chan.png"),
    });
}

fn check_squarea_loaded(
    menu_image: Res<MenuImages>,
    asset_server: Res<AssetServer>,
    mut menu_state: ResMut<NextState<MenuState>>,
    // mut ev_writer: EventWriter<MenuIntroEvent>,
    mut commands: Commands,
) {
    match asset_server.load_state(menu_image.squarea_chan.id()) {
        bevy::asset::LoadState::Loaded => {
            // ev_writer.send(MenuIntroEvent);
            commands.insert_resource(MenuIntroSequenceTimer(Timer::from_seconds(
                3.,
                TimerMode::Once,
            )));
            menu_state.set(MenuState::IntroSequence)
        }
        _ => {}
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Loading);
}

#[derive(Component)]
pub struct OnMainMenuScreen;

// All actions that can be triggered from a button click
#[derive(Component, Eq, PartialEq, Clone)]
pub enum MenuButtonAction {
    Play,
    Settings,
    About,
    Quit,
    MainMenu,
}

#[derive(Component)]
struct BackgroundImage;

#[derive(Component)]
struct ButtonContainer;

fn main_menu_setup(mut commands: Commands, menu_res: Res<MenuImages>) {
    commands
        .spawn((OnMainMenuScreen, background()))
        .with_children(|background| {
            background
                .spawn(menu_art_container())
                .with_children(|art_container| {
                    let squarea_chan = cover_image(menu_res);
                    art_container.spawn((BackgroundImage, squarea_chan));
                });
            background
                .spawn((ButtonContainer, buttons_container()))
                .with_children(|button_container| {
                    spawn_button(button_container, MenuButtonAction::Play, "Play");
                    spawn_button(button_container, MenuButtonAction::Settings, "Settings");
                    spawn_button(button_container, MenuButtonAction::About, "About");
                    spawn_button(button_container, MenuButtonAction::Quit, "Quit");
                });
        });
}

fn menu_action(
    curr_menu_state: Res<State<MenuState>>,
    commands: Commands,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    app_exit_events: EventWriter<AppExit>,
    menu_state: ResMut<NextState<MenuState>>,
    game_state: ResMut<NextState<SystemState>>,
    menu: Query<Entity, With<OnMainMenuScreen>>,
    selected_button: Query<&mut SelectionButton>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            apply_menu_action(
                curr_menu_state,
                menu_button_action,
                app_exit_events,
                menu_state,
                game_state,
                selected_button,
                menu,
                commands
            );
            return;
        }
    }
}

pub fn apply_menu_action(
    curr_menu_state: Res<State<MenuState>>,
    menu_button_action: &MenuButtonAction,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<SystemState>>,
    mut selected_button: Query<&mut SelectionButton>,
    menu: Query<Entity, With<OnMainMenuScreen>>,
    commands: Commands
) {
    match menu_button_action {
        MenuButtonAction::Quit => {
            app_exit_events.send(AppExit::Success);
        }
        MenuButtonAction::Play => {
            game_state.set(SystemState::Game);
            menu_state.set(MenuState::Disabled);
        }
        MenuButtonAction::Settings => {
            if *curr_menu_state.get() == MenuState::IntroSequence {
                despawn_screen(menu, commands);
            }
            selected_button.single_mut().0 = MenuButtonAction::MainMenu;
            menu_state.set(MenuState::Settings)
        }
        MenuButtonAction::MainMenu => {
            selected_button.single_mut().0 = MenuButtonAction::Play;
            menu_state.set(MenuState::Loading)
        }
        MenuButtonAction::About => {
            if *curr_menu_state.get() == MenuState::IntroSequence {
                despawn_screen(menu, commands);
            }
            selected_button.single_mut().0 = MenuButtonAction::MainMenu;
            menu_state.set(MenuState::About)
        }
    }
}

pub fn spawn_button(parent: &mut ChildBuilder<'_>, component: impl Component, text: &str) {
    parent.spawn((component, button())).with_children(|button| {
        button.spawn(default_text_style(text));
    });
}

fn background() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(40.),
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..Default::default()
        },
        z_index: ZIndex::Global(i32::MIN),
        background_color: Color::srgba(0., 0.2, 0.2, 0.0).into(),
        ..Default::default()
    }
}

fn menu_art_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            column_gap: Val::Px(40.),
            height: Val::Percent(100.),
            width: Val::Percent(40.),
            ..Default::default()
        },
        z_index: ZIndex::Global(i32::MIN),
        background_color: Color::srgb(0.2, 0.0, 0.2).into(),
        ..Default::default()
    }
}

fn cover_image(images: Res<MenuImages>) -> ImageBundle {
    ImageBundle {
        image: UiImage {
            texture: images.squarea_chan.clone(),
            color: Color::default().with_alpha(0.9),
            ..default()
        },
        style: Style {
            height: Val::Percent(100.),
            width: Val::Auto,
            ..Default::default()
        },
        ..Default::default()
    }
}

fn buttons_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(40.),
            height: Val::Percent(100.),
            width: Val::Auto,
            ..Default::default()
        },
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

fn menu_intro_animation(
    time: Res<Time>,
    mut timer: ResMut<MenuIntroSequenceTimer>,
    mut bg_query: Query<&mut Style, (Without<ButtonContainer>, With<BackgroundImage>)>,
    mut buttons_query: Query<&mut Style, (With<ButtonContainer>, Without<BackgroundImage>)>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        commands.remove_resource::<MenuIntroSequenceTimer>();
        menu_state.set(MenuState::Main);
        return;
    }

    for mut style in &mut bg_query {
        style.right = Val::Px(timer.0.remaining_secs() * 100.);
    }

    for mut style in &mut buttons_query {
        style.left = Val::Px(timer.0.remaining_secs() * 100.);
    }
}
