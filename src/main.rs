use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
    text::{BreakLineOn, Text2dBounds},
    window::PrimaryWindow,
};
use rand::{thread_rng, Rng};
mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init_board)
        .add_systems(
            Update,
            (
                (
                    open_rectangle.run_if(input_just_pressed(MouseButton::Left)),
                    extend_rectangle.run_if(input_pressed(MouseButton::Left)),
                    close_rectangle.run_if(input_just_released(MouseButton::Left)),
                )
                    .chain(),
                // handle_drag.run_if(input_pressed(MouseButton::Left)),
                handle_reset.run_if(input_just_pressed(KeyCode::KeyR)),
            ),
        )
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
pub struct Tile(u8);

#[derive(Component)]
pub struct Rectangle;

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
                    Tile(val),
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
        let (mut visibility, mut transform) =
            rectangle.get_single_mut().expect("ggs no input rect lmao");

        transform.translation = position.extend(1.0);
        *visibility = Visibility::Visible;
    }
}

fn extend_rectangle(
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
        let (mut visibility, mut transform) =
            rectangle.get_single_mut().expect("ggs no input rect lmao");
        transform.scale = Vec3::new(
            transform.translation.x - position.x,
            transform.translation.y - position.y,
            1.0,
        );

        *visibility = Visibility::Visible;
    }
}

fn close_rectangle(
    mut commands: Commands,
    mut rectangle: Query<(&mut Visibility, &mut Transform), With<Rectangle>>,
    mut tiles: Query<(Entity, &Transform, &mut Sprite, &Tile), Without<Rectangle>>,
) {
    let (mut visibility, rect_transform) =
        rectangle.get_single_mut().expect("ggs no input rect lmao");

    *visibility = Visibility::Hidden;

    // input_rectangle is anchored to the topRight, so offset it by
    let (left_bound, right_bound) = match (
        rect_transform.translation.x,
        rect_transform.translation.x - rect_transform.scale.x,
    ) {
        (a, b) if a < b => (b, a),
        (a, b) => (a, b),
    };

    let (lower_bound, upper_bound) = match (
        rect_transform.translation.y,
        rect_transform.translation.y - rect_transform.scale.y,
    ) {
        (a, b) if a < b => (b, a),
        (a, b) => (a, b),
    };

    let tiles_selected: Vec<_> = tiles
        .iter()
        .filter(|(_, tile_transform, _, _)| {
            tile_transform.translation.x <= left_bound
                && tile_transform.translation.x >= right_bound
                && tile_transform.translation.y <= lower_bound
                && tile_transform.translation.y >= upper_bound
        })
        .collect();

    // tiles_selected.for_each(|((_, _, mut sprite, _))| sprite.color = Color::srgb(0.2, 1.0, 0.4));

    if tiles_selected.iter().map(|(_, _, _, t)| t.0).sum::<u8>() == 10 {
        for (e, _, _, _) in tiles_selected {
            commands.entity(e).despawn_recursive();
        }
    }
}

// fn handle_drag(
//     mut commands: Commands,
//     mut tiles: Query<(Entity, &Transform, &mut Sprite), With<Tile>>,
//     windows: Query<&Window, With<PrimaryWindow>>,
//     camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
// ) {
//     let (camera, camera_transform) = camera_q.single();

//     if let Some(position) = windows
//         .single()
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
//     {
//         // println!("Cursor is inside the primary window, at {:?}", position);

//         for (entity, transform, mut sprite) in &mut tiles {
//             if position.y >= transform.translation.y - (TILE_SIZE / 2.)
//                 && position.y <= transform.translation.y + (TILE_SIZE / 2.)
//                 && position.x >= transform.translation.x - (TILE_SIZE / 2.)
//                 && position.x <= transform.translation.x + (TILE_SIZE / 2.)
//             {
//                 // println!(
//                 //     "{} >= {} >= {} && {} >= {} >= {}",
//                 //     transform.translation.x,
//                 //     position.x,
//                 //     transform.translation.x + TILE_SIZE,
//                 //     transform.translation.y,
//                 //     position.y,
//                 //     transform.translation.y + TILE_SIZE,
//                 // );
//                 // println!("dragging deez nuts {:?} {:?}", entity, transform);
//                 sprite.color = Color::srgb(1.0, 0.2, 0.7);
//                 // commands.entity(entity).despawn_recursive();
//             }
//         }
//     }
// }

fn handle_reset(mut tiles: Query<(Entity, &Transform, &mut Sprite), With<Tile>>) {
    for (entity, transform, mut sprite) in &mut tiles {
        sprite.color = Color::srgb(0.20, 0.3, 0.70);
    }
}
