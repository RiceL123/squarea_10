use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    // prelude::{
    //     in_state, App, AppExtStates, ClearColor, Color, DefaultPlugins, IntoSystemConfigs,
    //     KeyCode, OnEnter, OnExit, Query, ResMut, States, Style, Update,
    //     With,
    // },
    ui::Display,
    winit::WinitSettings,
};
use menu::MenuNode;
use results::ResultsNode;
use sound::HitEvent;

#[derive(States, Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Results,
}

mod animate_tiles;
mod area_multiplier;
mod combo_multiplier;
mod conversions;
mod menu;
mod results;
mod sound;
mod squarea_core;
mod timer;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, init_game)
        .add_plugins(squarea_core::SquareaCore)
        .add_plugins(combo_multiplier::ComboMultiplier)
        .add_plugins(area_multiplier::AreaMultiplier)
        .add_plugins(animate_tiles::AnimateTiles)
        .add_plugins(timer::SquareaTimer)
        .add_plugins(menu::Menu)
        .add_plugins(results::Results)
        .add_plugins(sound::Sound)
        .add_systems(OnEnter(GameState::MainMenu), show_menu)
        .add_systems(OnExit(GameState::MainMenu), hide_menu)
        .add_systems(OnEnter(GameState::Results), show_results)
        .add_systems(OnExit(GameState::Results), hide_results)
        .add_systems(
            Update,
            toggle_game_state.run_if(input_just_pressed(KeyCode::Escape)),
        )
        .insert_resource(ClearColor(Color::srgb(0.99, 0.5, 0.5)))
        .run();
}

fn init_game(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn show_menu(mut menu: Query<&mut Style, With<MenuNode>>) {
    if let Ok(x) = &mut menu.get_single_mut() {
        x.display = Display::Flex;
    };
}

fn hide_menu(mut menu: Query<&mut Style, With<MenuNode>>) {
    if let Ok(x) = &mut menu.get_single_mut() {
        x.display = Display::None;
    };
}

fn show_results(mut results: Query<&mut Style, With<ResultsNode>>) {
    if let Ok(x) = &mut results.get_single_mut() {
        x.display = Display::Flex;
    };
}

fn hide_results(mut results: Query<&mut Style, With<ResultsNode>>) {
    if let Ok(x) = &mut results.get_single_mut() {
        x.display = Display::None;
    };
}

fn toggle_game_state(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ev_writer: EventWriter<HitEvent>,
) {
    match state.get() {
        GameState::InGame => next_state.set(GameState::MainMenu),
        GameState::Results => next_state.set(GameState::MainMenu),
        GameState::MainMenu => next_state.set(GameState::InGame),
    }

    ev_writer.send(HitEvent(1));
}
