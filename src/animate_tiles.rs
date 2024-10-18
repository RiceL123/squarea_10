use crate::squarea_core::{PopTiles, COLS, ROWS, TILE_GAP, TILE_SIZE};
use bevy::{prelude::*, time::Stopwatch};
use rand::random;

pub struct AnimateTiles;

#[derive(Component)] // (stopwatch, x-velocity, y-velocity, x_start, y_start)
struct TileMotion(Stopwatch, f32, f32, f32, f32);

#[derive(Component)]
pub struct Animating;

impl Plugin for AnimateTiles {
    fn build(&self, app: &mut App) {
        app.observe(start_animate_tiles);
        app.add_systems(Update, animate_tiles);
    }
}

fn start_animate_tiles(trigger: Trigger<PopTiles>, mut commands: Commands) {
    trigger.event().0.iter().for_each(|(e, p)| {
        let x_velocity = (random::<f32>() - 0.5) * 1200.;
        let y_velocity = random::<f32>() * 1200.;

        let x_start = (TILE_SIZE + TILE_GAP) * (p.col as f32 + 0.5)
            - ((TILE_SIZE + TILE_GAP) * COLS as f32 / 2.);
        let y_start = (TILE_SIZE + TILE_GAP) * (p.row as f32 + 0.5)
            - ((TILE_SIZE + TILE_GAP) * ROWS as f32 / 2.);

        commands.entity(*e).insert((
            Animating,
            TileMotion(Stopwatch::new(), x_velocity, y_velocity, x_start, y_start),
        ));
    });
}

const GRAVITY: f32 = 2000.;

fn animate_tiles(
    mut commands: Commands,
    time: Res<Time>,
    mut sprite_position: Query<(&mut TileMotion, &mut Transform, Entity), With<Animating>>,
) {
    for (mut v, mut transform, entity) in &mut sprite_position {
        v.0.tick(time.delta());

        // x = x0 + v * t
        transform.translation.x = v.3 + v.1 * v.0.elapsed_secs();

        // y = y0 + v * t - g * (t ^ 2) / 2
        transform.translation.y =
            v.4 + v.2 * v.0.elapsed_secs() - GRAVITY * v.0.elapsed_secs().powi(2) / 2.;

        if transform.translation.y > 2000.
            || transform.translation.y < -2000.
            || transform.translation.x > 2000.
            || transform.translation.x < -2000.
        {
            commands.entity(entity).despawn_recursive();
        }

        transform.translation.z = 50.;

        transform.scale = Vec3::new(1.1, 1.1, 10.);
    }
}
