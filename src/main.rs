use bevy::{
    input::common_conditions::{input_just_pressed, input_pressed},
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
    window::PrimaryWindow,
};
use rand::random;
mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_board)
        .add_systems(Update, handle_drag.run_if(input_pressed(MouseButton::Left)))
        .insert_resource(ClearColor(Color::srgb(1., 0.3, 0.4)))
        .run();
}

// x coordinates
const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;
// y coordinates
const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;

const ROWS: usize = 10;
const COLS: usize = 10;

const TILE_SIZE: f32 = 50.;
const TILE_GAP: f32 = 10.;

#[derive(Component)]
pub struct Block;

#[derive(Component)]
pub struct ActiveBlock;

#[derive(Component, Debug)]
pub struct Tile;

fn init_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    //camera
    commands.spawn(Camera2dBundle::default());

    // text
    let font = asset_server.load("fonts/font.ttf");
    let font = TextStyle {
        font,
        font_size: 30.0,
        ..default()
    };

    // let mut board = Board { grid: [[]] };

    // activeBlock
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(500., 500., 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::srgb(0.9, 0.8, 0.7),
                ..default()
            },
            ..default()
        },
        ActiveBlock,
    ));

    let offset_x = -((COLS - 1) as f32) * (TILE_SIZE + TILE_GAP) / 2.;
    let offset_y = -((ROWS - 1) as f32) * (TILE_SIZE + TILE_GAP) / 2.;

    for row in 0..ROWS {
        for col in 0..COLS {
            let pos = Vec2::new(
                offset_x + col as f32 * (TILE_SIZE + TILE_GAP),
                offset_y + row as f32 * (TILE_SIZE + TILE_GAP),
            );

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
                    Tile,
                ))
                .with_children(|builder| {
                    builder.spawn(Text2dBundle {
                        text: Text::from_section(format!("{},{}", row, col), font.clone()),
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    });
                });
        }
    }
}

fn handle_drag(
    mut commands: Commands,
    mut tiles: Query<(Entity, &Transform, &mut Sprite), With<Tile>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera_q.single();

    if let Some(position) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        println!("Cursor is inside the primary window, at {:?}", position);

        for (entity, transform, mut sprite) in &mut tiles {
            if (position.x >= transform.translation.x - (TILE_SIZE / 2.)
                && position.x <= transform.translation.x + (TILE_SIZE / 2.))
                && (position.y >= transform.translation.y - (TILE_SIZE / 2.)
                    && position.y <= transform.translation.y + (TILE_SIZE / 2.))
            {
                println!(
                    "{} >= {} >= {} && {} >= {} >= {}",
                    transform.translation.x,
                    position.x,
                    transform.translation.x + TILE_SIZE,
                    transform.translation.y,
                    position.y,
                    transform.translation.y + TILE_SIZE,
                );
                // println!("dragging deez nuts {:?} {:?}", entity, transform);
                sprite.color = Color::srgb(1.0, 0.2, 0.7);
                // commands.entity(entity).despawn_recursive();
            }
        }
    }
}
