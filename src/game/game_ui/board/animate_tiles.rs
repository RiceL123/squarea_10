use bevy::{prelude::*, time::Stopwatch};
use rand::random;

use crate::game::GameState;

use super::Tile;

pub fn animate_plugin(app: &mut App) {
    app.add_systems(Update, animate_tiles.run_if(in_state(GameState::Playing)))
        .add_systems(Update, begin_tile_animation)
        .add_event::<StartTileAnimationEvent>();
}

const GRAVITY: f32 = 2000.;

#[derive(Event)]
pub struct StartTileAnimationEvent(pub Vec<(Entity, Transform)>);

#[derive(Component)] // (stopwatch, x-velocity, y-velocity, x_start, y_start)
struct TileAnimating {
    stopwatch: Stopwatch,
    x_velocity: f32,
    y_velocity: f32,
    x_start: f32,
    y_start: f32,
}

// pub fn start_tile_animation(mut commands: Commands, mut tiles: Vec<(Entity, &mut Transform)>) {
//     tiles.iter_mut().for_each(|(e, t)| {
//         commands.entity(*e).insert(TileAnimating {
//             stopwatch: Stopwatch::new(),
//             x_velocity: (random::<f32>() - 0.5) * 1200.,
//             y_velocity: random::<f32>() * 1200.,
//             x_start: t.translation.x,
//             y_start: t.translation.y,
//         });

//         t.translation.z = 100.; // put above ui elements and board
//     });
// }

pub fn begin_tile_animation(
    mut ev_reader: EventReader<StartTileAnimationEvent>,
    mut commands: Commands,
) {
    for event in ev_reader.read() {
        event.0.iter().for_each(|(e, t)| {
            commands.entity(*e).insert(TileAnimating {
                stopwatch: Stopwatch::new(),
                x_velocity: (random::<f32>() - 0.5) * 1200.,
                y_velocity: random::<f32>() * 1200.,
                x_start: t.translation.x,
                y_start: t.translation.y,
            });
        });
    }
}

fn animate_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut sprite_query: Query<(&mut TileAnimating, &mut Transform, Entity), With<Tile>>,
    window: Query<&Window>,
) {
    for (mut tile_animating, mut transform, entity) in &mut sprite_query {
        tile_animating.stopwatch.tick(time.delta());

        let t = tile_animating.stopwatch.elapsed_secs();

        // x = x0 + v * t
        transform.translation.x = tile_animating.x_start + tile_animating.x_velocity * t;

        // y = y0 + v * t - g * (t ^ 2) / 2
        transform.translation.y =
            tile_animating.y_start + tile_animating.y_velocity * t - GRAVITY * t.powi(2) / 2.;

        transform.scale = Vec3::new(transform.scale.x + 0.01, transform.scale.y + 0.01, 0.);

        if let Ok(window) = window.get_single() {
            let x_bounds = window.width() / 2.;
            let y_bounds = window.height() / 2.;

            if transform.translation.y < -y_bounds
                || transform.translation.x > x_bounds
                || transform.translation.x < -x_bounds
            {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            commands.entity(entity).despawn_recursive();
        };

        transform.translation.z = 100.; // put above ui elements and board
    }
}
