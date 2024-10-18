use bevy::prelude::*;

use crate::{sound::HitEvent, squarea_core::Score, GameState};

pub struct Results;

impl Plugin for Results {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_menu);
        app.add_systems(OnEnter(GameState::Results), enter_results);
        app.add_systems(Update, (retry_button_system, mainmenu_button_system));
    }
}

#[derive(Component)]
pub struct ResultsNode;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
struct RetryButton;

#[derive(Component)]
struct QuitButton;

fn load_menu(mut commands: Commands) {
    commands
        .spawn((
            ResultsNode,
            NodeBundle {
                style: Style {
                    display: Display::None,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(40.),
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..default()
                },
                z_index: ZIndex::Global(100),
                background_color: Color::srgba(0.1, 0., 0.1, 0.3).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                ScoreText,
                TextBundle::from_section("Score: ", TextStyle { ..default() }),
            ));

            parent
                .spawn((
                    RetryButton,
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
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("retry", TextStyle { ..default() }));
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
                        background_color: Color::srgba(0.1, 0.1, 0.1, 0.8).into(),
                        ..default()
                    },
                ))
                .with_children(|button| {
                    button.spawn(TextBundle::from_section("menu", TextStyle { ..default() }));
                });
        });
}

fn retry_button_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<RetryButton>),
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

fn mainmenu_button_system(
    mut game_state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut play_sfx: EventWriter<HitEvent>,
) {
    if let Ok(query) = &mut interaction_query.get_single_mut() {
        match *query.0 {
            Interaction::Pressed => {
                game_state.set(GameState::MainMenu);
                play_sfx.send(HitEvent(1));
            }
            Interaction::Hovered => {
                query.1 .0 = Color::srgb(0.2, 0.2, 0.9);
            }
            _ => {
                query.1 .0 = Color::srgb(0.9, 0.9, 0.9);
            }
        }
    }
}

fn enter_results(
    mut commands: Commands,
    score: Res<Score>,
    mut scpre_text: Query<&mut Text, With<ScoreText>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(mut x) = scpre_text.get_single_mut() {
        x.sections[0].value = format!("Score: {}", score.value);
    }

    commands.spawn((AudioBundle {
        source: asset_server.load("finished.ogg"),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    },));
}
