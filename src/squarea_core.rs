use std::{default, time::Duration};

use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Anchor,
    time::common_conditions::on_timer,
    window::PrimaryWindow,
};

use crate::{conversions::RectBounds, GameState};
use rand::{thread_rng, Rng};

pub struct SquareaCore;

impl Plugin for SquareaCore {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), (cleanup_board, init_board));
        app.add_systems(
            Update,
            (
                open_rectangle.run_if(input_just_pressed(MouseButton::Left)),
                extend_rectangle.run_if(input_pressed(MouseButton::Left)),
                close_rectangle.run_if(input_just_released(MouseButton::Left)),
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        );
        app.add_event::<PopTiles>();
        app.observe(pop_tiles);
        app.add_systems(
            Update,
            refresh_scoreboard.run_if(on_timer(Duration::from_secs(3))),
        );
    }
}

pub const ROWS: usize = 11;
pub const COLS: usize = 18;

pub const TILE_SIZE: f32 = 40.;
pub const TILE_GAP: f32 = 10.;

#[derive(Component)]
struct CleanUp;

#[derive(Component, Debug)]
pub struct Tile {
    pub value: u8,
    pub position: Position,
}

#[derive(Component)]
pub struct Rectangle;

#[derive(Resource, Debug)]
pub struct Score {
    pub value: u32,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Event)]
pub struct PopTiles(pub Vec<(Entity, Position)>);

fn cleanup_board(mut commands: Commands, cleanup: Query<Entity, With<CleanUp>>) {
    for e in cleanup.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn init_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    // text
    let font = asset_server.load("fonts/font.ttf");
    let font = TextStyle {
        font,
        font_size: 30.0,
        ..default()
    };

    let offset_x = -((COLS - 1) as f32) * (TILE_SIZE + TILE_GAP) / 2.;
    let offset_y = -((ROWS - 1) as f32) * (TILE_SIZE + TILE_GAP) / 2.;

    let mut rng_generator = thread_rng();

    for row in 0..ROWS {
        for col in 0..COLS {
            let pos = Vec2::new(
                offset_x + col as f32 * (TILE_SIZE + TILE_GAP),
                offset_y + row as f32 * (TILE_SIZE + TILE_GAP),
            );

            let val = rng_generator.gen_range(1..10);
            commands
                .spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.20, 0.3, 0.70),
                            custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_translation(pos.extend(0.0)),
                        ..default()
                    },
                    Tile {
                        value: val,
                        position: Position {
                            row: row as u8,
                            col: col as u8,
                        },
                    },
                    CleanUp,
                ))
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text::from_section(format!("{}", val), font.clone()),
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
        }
    }

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
        CleanUp,
    ));

    commands.insert_resource(Score { value: 0 });
    commands.spawn((
        CleanUp,
        ScoreBoard,
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font_size: 50.,
                    color: Color::srgb(1., 0.7, 0.8),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 50.,
                color: Color::srgb(1., 0.7, 0.8),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(50.0),
            left: Val::Px(50.0),
            ..default()
        }),
    ));
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
        if let Ok((mut visibility, mut transform)) = rectangle.get_single_mut() {
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
                        sprite.color = Color::srgb(0.20, 0.3, 0.70)
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
) {
    if let Ok((mut visibility, transform)) = rectangle.get_single_mut() {
        *visibility = Visibility::Hidden;

        let bounds = RectBounds::new(&transform);

        let mut tiles_selected: Vec<_> = tiles
            .iter_mut()
            .filter(|(_, tile_transform, _, _)| bounds.contains(tile_transform))
            .collect();

        if tiles_selected.len() < 10
            && tiles_selected
                .iter()
                .map(|(_, _, _, t)| t.value)
                .sum::<u8>()
                == 10
        {
            commands.trigger(PopTiles(
                tiles_selected
                    .iter()
                    .map(|(e, _, _, tile)| (*e, tile.position.clone()))
                    .collect(),
            ));
        } else {
            for (_, _, ref mut s, _) in &mut tiles_selected {
                s.color = Color::srgb(0.20, 0.3, 0.70);
            }
        }
    }
}

fn pop_tiles(
    trigger: Trigger<PopTiles>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
) {
    for (entity, pos) in trigger.event().0.iter() {
        // commands.entity(*entity).despawn_recursive(); // animation will despawn the tile
        score.value += 1;
    }

    // println!("tile pop: + {}", trigger.event().0.len());

    if let Ok(mut text) = score_board.get_single_mut() {
        text.sections[1].value = score.value.to_string();

        text.sections.push(TextSection {
            value: format!("\ntiles popped: + {}", trigger.event().0.len()),
            style: TextStyle {
                font_size: 20.,
                color: Color::srgb(1., 0.7, 0.8),
                ..default()
            },
        });
    }
}

fn refresh_scoreboard(mut score_board: Query<&mut Text, With<ScoreBoard>>) {
    if let Ok(mut text) = score_board.get_single_mut() {
        if text.sections.len() > 10 {
            text.sections.remove(2);
        }
    }
}
