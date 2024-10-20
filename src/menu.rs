use bevy::prelude::*;

use crate::{sound::HitEvent, GameState};

pub struct Menu;

impl Plugin for Menu {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_menu);
        app.add_systems(Update, (play_button_system, quit_button_system));
    }
}

#[derive(Component)]
pub struct MenuNode;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct QuitButton;

fn load_menu(mut commands: Commands) {
    commands
        .spawn((
            MenuNode,
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(40.),
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(50),
                background_color: Color::srgb(0., 0.2, 0.2).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    PlayButton,
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
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("play", TextStyle { ..default() }));
                });

            parent
                .spawn((
                    QuitButton,
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
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("quit", TextStyle { ..default() }));
                });
        });
}

fn play_button_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut play_sfx: EventWriter<HitEvent>,
) {
    if let Ok(query) = &mut interaction_query.get_single_mut() {
        match *query.0 {
            Interaction::Pressed => {
                game_state.set(GameState::InGame);
                play_sfx.send(HitEvent(1));
            }
            Interaction::Hovered => {
                query.1 .0 = Color::srgb(0.1, 0.8, 0.1);
            }
            _ => {
                query.1 .0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}

fn quit_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(query) = &mut interaction_query.get_single_mut() {
        match *query.0 {
            Interaction::Pressed => {
                exit.send(AppExit::Success);
            }
            Interaction::Hovered => {
                query.1 .0 = Color::srgb(0.9, 0.2, 0.2);
            }
            _ => {
                query.1 .0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}
