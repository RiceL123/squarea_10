use std::time::Duration;

use crate::{
    squarea_core::{COLS, TILE_GAP, TILE_SIZE},
    GameState,
};

use bevy::prelude::*;

pub struct SquareaTimer;

impl Plugin for SquareaTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_timer);
        app.add_systems(OnEnter(GameState::InGame), restart_timer);
        app.add_systems(Update, count_timer.run_if(in_state(GameState::InGame)));
    }
}

const DURATION: f32 = 100.;

#[derive(Component)]
pub struct TimerText;

#[derive(Component)]
pub struct TimerProgressBar;

#[derive(Resource)]
pub struct CountDown {
    pub timer: Timer,
}

fn setup_timer(mut commands: Commands) {
    commands.insert_resource(CountDown {
        timer: Timer::from_seconds(DURATION, TimerMode::Repeating),
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(50., 500.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                (COLS as f32 + 2.) / 2. * (TILE_SIZE + TILE_GAP),
                0.,
                1.,
            )),
            ..default()
        },
        TimerProgressBar,
    ));

    commands.spawn((
        TimerText,
        TextBundle::from_sections([
            TextSection::new(
                "Time Remaining: ",
                TextStyle {
                    font_size: 30.,
                    color: Color::srgb(1., 0.7, 0.8),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 30.,
                color: Color::srgb(1., 0.7, 0.8),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            right: Val::Px(50.0),
            ..default()
        }),
    ));
}

fn restart_timer(mut countdown: ResMut<CountDown>) {
    countdown.timer.set_elapsed(Duration::from_secs(0));
}

fn count_timer(
    time: Res<Time>,
    mut countdown: ResMut<CountDown>,
    mut timer_progress_bar: Query<&mut Sprite, With<TimerProgressBar>>,
    mut timer_text: Query<&mut Text, With<TimerText>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    countdown.timer.tick(time.delta());
    if !countdown.timer.just_finished() {
        let percentage =
            countdown.timer.elapsed().as_secs_f32() / countdown.timer.duration().as_secs_f32();
        timer_progress_bar.single_mut().custom_size =
            Some(Vec2::new(50., 500. - percentage * 500.));

        timer_progress_bar.single_mut().color = timer_progress_bar
            .single_mut()
            .color
            .to_srgba()
            .with_red(percentage)
            .with_green(1. - percentage)
            .into();

        timer_text.single_mut().sections[1].value =
            format!("{:.2}", countdown.timer.remaining().as_secs_f32())
    } else {
        next_state.set(GameState::Results);
    }
}
