use bevy::input::common_conditions::{input_just_pressed, input_just_released, input_pressed};
use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::game::squaregg::{Position, COLS, ROWS};
use crate::game::GameState;
use crate::menu::settings::GameConfig;
use crate::{game::InternalGameState, SystemState};

mod input;
mod animate_tiles;
mod conversions;

pub fn board_plugin(app: &mut App) {
    // if board_setup scheduled on OnEnter(SystemState::Playing), tiles might render previous board
    app.add_systems(OnEnter(GameState::Playing), board_setup) 
        .add_systems(OnExit(GameState::Playing), board_cleanup)
        .add_plugins((input::input_plugin, animate_tiles::animate_plugin));
}

#[derive(Component)]
pub struct Rectangle;

#[derive(Component)]
pub struct PrevRectangle;

#[derive(Component)]
struct Tile {
    row: i32,
    col: i32,
}

fn board_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    internal_game_state: Res<InternalGameState>,
    config: Res<GameConfig>,
) {
    spawn_tiles(&mut commands, asset_server, internal_game_state, config);
    spawn_rectangle(&mut commands);
    spawn_prev_rectangle(&mut commands);
}

fn spawn_tiles(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    internal_game_state: Res<InternalGameState>,
    config: Res<GameConfig>,
) {
    let offset_x = -((COLS - 1) as f32) * (config.tile_size + config.tile_gap) / 2.;
    let offset_y = -((ROWS - 1) as f32) * (config.tile_size + config.tile_gap) / 2.;

    internal_game_state
        .0
        .board
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            row.iter().enumerate().for_each(|(col_index, val)| {
                if let Some(val) = val {
                    let pos = Vec2::new(
                        offset_x + col_index as f32 * (config.tile_size + config.tile_gap),
                        offset_y + row_index as f32 * (config.tile_size + config.tile_gap),
                    );

                    commands
                        .spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: Some(Vec2::new(
                                        config.tile_size,
                                        config.tile_size,
                                    )),
                                    color: Color::hsl(0.2, 0.2, 0.9),
                                    ..default()
                                },
                                transform: Transform::from_translation(pos.extend(0.0)),
                                ..default()
                            },
                            Tile {
                                row: row_index as i32,
                                col: col_index as i32,
                            },
                        ))
                        .with_children(|builder| {
                            builder.spawn(Text2dBundle {
                                text: Text::from_section(
                                    format!("{}", val),
                                    TextStyle {
                                        color: config.tile_text_color,
                                        ..default()
                                    },
                                ),
                                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                                ..default()
                            });
                        });
                }
            });
        });
}

fn spawn_rectangle(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(500., 500., 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgba(0.9, 0.8, 0.7, 0.2),
                anchor: Anchor::TopRight,
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        Rectangle,
    ));
}

fn spawn_prev_rectangle(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                color: Color::srgba(0.9, 0.8, 0.2, 0.2),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PrevRectangle,
    ));
}

fn board_cleanup(
    tile_query: Query<Entity, With<Tile>>,
    rectangle_query: Query<Entity, With<Rectangle>>,
    prev_rectangle_query: Query<Entity, With<PrevRectangle>>,
    mut commands: Commands,
) {
    for entity in &tile_query {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &rectangle_query {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &prev_rectangle_query {
        commands.entity(entity).despawn_recursive();
    }
}
