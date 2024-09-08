use crate::squarea_core::{COLS, TILE_GAP, TILE_SIZE};
use bevy::{prelude::*, time::common_conditions::on_timer};

pub struct SquareaTimer;

impl Plugin for SquareaTimer {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_timer));
        app.add_systems(Update, count_timer);
    }
}
#[derive(Component)]
pub struct TimerBox;

#[derive(Resource)]
pub struct CountDown {
    pub timer: Timer,
}

fn setup_timer(mut commands: Commands) {
    commands.insert_resource(CountDown {
        timer: Timer::from_seconds(100., TimerMode::Once),
    });

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.60, 0.6, 0.90),
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
        TimerBox,
    ));
}

fn count_timer(
    time: Res<Time>,
    mut countdown: ResMut<CountDown>,
    mut timer_box: Query<(&mut Sprite), With<TimerBox>>,
) {
    countdown.timer.tick(time.delta());
    if !countdown.timer.finished() {
        // Print the percent complete the main timer is.
        // println!("{:?}", countdown.timer);

        let percentage =
            countdown.timer.elapsed().as_secs_f32() / countdown.timer.duration().as_secs_f32();
        timer_box.single_mut().custom_size = Some(Vec2::new(50., 500. - percentage * 500.))
    }
}
