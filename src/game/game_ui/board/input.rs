use bevy::{input::common_conditions::{input_just_pressed, input_just_released, input_pressed}, prelude::*, window::PrimaryWindow};

use crate::game::{squaregg::Position, GameState, InternalGameState};

use super::{conversions::RectBounds, Rectangle, Tile};

pub fn input_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            open_rectangle.run_if(input_just_pressed(MouseButton::Left)),
            extend_rectangle.run_if(input_pressed(MouseButton::Left)),
            close_rectangle.run_if(input_just_released(MouseButton::Left)),
        )
            .chain()
            .run_if(in_state(GameState::Playing)),
    );
}


fn open_rectangle(
    mut rectangle: Query<(&mut Visibility, &mut Transform), With<Rectangle>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera_q.single();
    if let Some(position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if let Ok((mut visibility, mut transform)) = rectangle.get_single_mut() {
            transform.translation = position.extend(1.0);
            *visibility = Visibility::Visible;
        }
    }
}

fn extend_rectangle(
    mut rectangle: Query<(&mut Visibility, &mut Transform), With<Rectangle>>,
    mut tiles: Query<(&Tile, &Transform, &mut Sprite), Without<Rectangle>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera_q.single();
    if let Some(position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        if let Ok((_, mut transform)) = rectangle.get_single_mut() {
            transform.scale = Vec3::new(
                transform.translation.x - position.x,
                transform.translation.y - position.y,
                1.0,
            );

            let bounds = RectBounds::new(&transform);

            tiles
                .iter_mut()
                .for_each(|(_, tile_transform, mut sprite)| {
                    if bounds.contains(tile_transform) {
                        sprite.color = Color::srgb(0.20, 0.8, 0.70)
                    } else {
                        sprite.color = Color::WHITE
                    }
                });

            // *visibility = Visibility::Visible;
        }
    }
}

fn close_rectangle(
    mut commands: Commands,
    mut rectangle: Query<(&mut Visibility, &mut Transform), With<Rectangle>>,
    mut tiles: Query<(Entity, &Transform, &mut Sprite, &Tile), Without<Rectangle>>,
    mut internal_game_state: ResMut<InternalGameState>,
) {
    if let Ok((mut visibility, transform)) = rectangle.get_single_mut() {
        *visibility = Visibility::Hidden;

        let bounds = RectBounds::new(&transform);

        let tiles_selected: Vec<_> = tiles
            .iter_mut()
            .filter(|(_, tile_transform, _, _)| bounds.contains(tile_transform))
            .collect();

        if let Ok(prev_area) = internal_game_state.0.try_pop_tiles(
            &tiles_selected
                .iter()
                .map(|(_, _, _, tile)| Position {
                    row: tile.row as usize,
                    col: tile.col as usize,
                })
                .collect(),
        ) {
            println!("hello");

            tiles_selected.iter().for_each(|(e, _, _, _)| {
                commands.entity(*e).despawn_recursive();
            });
        }

        println!("{:?}", internal_game_state.0);
    }
}
