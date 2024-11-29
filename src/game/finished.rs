use bevy::prelude::*;

use crate::{despawn_screen, menu::spawn_button, SystemState};

use super::GameState;

pub fn finished_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Finished), game_finished_setup)
        .add_systems(OnExit(GameState::Finished), despawn_screen::<OnFinished>)
        .add_systems(
            Update,
            (results_action, button_system).run_if(in_state(GameState::Finished)),
        );
}

// recurisvely despawn onFinished elements on exit
#[derive(Component)]
struct OnFinished;

fn game_finished_setup(mut commands: Commands) {
    commands
        .spawn((OnFinished, background()))
        .with_children(|parent| {
            spawn_score(parent);
            spawn_button(parent, ReultsButtonAction::Play, "Retry");
            spawn_button(parent, ReultsButtonAction::Menu, "Menu");
            spawn_button(parent, ReultsButtonAction::Share, "Share");
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
        background_color: Color::srgb(0., 0.2, 0.2).into(),
        ..Default::default()
    }
}

fn spawn_score(parent: &mut ChildBuilder<'_>) {
    parent.spawn(TextBundle::from_section("good job?? or bad job nice", TextStyle { ..default() }));
}

// All actions that can be triggered from a button click
#[derive(Component)]
enum ReultsButtonAction {
    Play,
    Menu,
    Share,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
struct SelectedOption;

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

fn results_action(
    interaction_query: Query<
        (&Interaction, &ReultsButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut system_state: ResMut<NextState<SystemState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                ReultsButtonAction::Menu => {
                    system_state.set(SystemState::Menu);
                }
                ReultsButtonAction::Play => {
                    game_state.set(GameState::Starting);
                }
                ReultsButtonAction::Share => { eprintln!("todo!()")},
            }
        }
    }
}
