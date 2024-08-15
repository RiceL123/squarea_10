use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{BreakLineOn, Text2dBounds},
};
use rand::random;
mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_board)
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

#[derive(Component)]
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
                .spawn(SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(0.20, 0.3, 0.70),
                        custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_translation(pos.extend(0.0)),
                    ..default()
                })
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
