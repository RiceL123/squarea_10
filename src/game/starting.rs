use bevy::prelude::*;

use crate::{despawn_screen, SystemState};

use super::GameState;

pub fn starting_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Starting), game_starting_setup)
        .add_systems(OnExit(GameState::Starting), despawn_screen::<OnStarting>)
        .add_systems(Update, countdown.run_if(in_state(GameState::Starting)));
}

// recurisvely despawn onstarting elements on exit
#[derive(Component)]
struct OnStarting;

// Newtype to use a `Timer` for this screen as a resource
#[derive(Resource, Deref, DerefMut)]
struct StartingTimer(Timer);

fn game_starting_setup(mut commands: Commands) {
    commands
        .spawn((OnStarting, background()))
        .with_children(|parent| {
            spawn_count_down(parent);
        });

    commands.insert_resource(StartingTimer(Timer::from_seconds(3.0, TimerMode::Once)));
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
        // background_color: Color::srgb(0., 0.2, 0.2).into(),
        ..Default::default()
    }
}

fn spawn_count_down(parent: &mut ChildBuilder<'_>) {
    parent.spawn(TextBundle::from_section(
        "starting!!!",
        TextStyle { ..default() },
    ));
}

fn go_main_menu(mut system_state: ResMut<NextState<SystemState>>) {
    system_state.set(SystemState::Menu);
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<StartingTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::Playing);
    }
}
