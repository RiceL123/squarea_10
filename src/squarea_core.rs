use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use rand::{thread_rng, Rng};

pub struct SquareaCore;

impl Plugin for SquareaCore {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_board);
        app.add_systems(
            Update,
            (
                (
                    open_rectangle.run_if(input_just_pressed(MouseButton::Left)),
                    extend_rectangle.run_if(input_pressed(MouseButton::Left)),
                    close_rectangle.run_if(input_just_released(MouseButton::Left)),
                )
                    .chain(),
                handle_reset.run_if(input_just_pressed(KeyCode::KeyR)),
            ),
        );
        app.add_event::<PopTiles>();
        app.observe(pop_tiles);
        app.insert_resource(Time::<Fixed>::from_seconds(2.));
        app.add_systems(FixedUpdate, refresh_scoreboard);
    }
}

const ROWS: usize = 10;
const COLS: usize = 10;

pub const TILE_SIZE: f32 = 50.;
pub const TILE_GAP: f32 = 10.;

#[derive(Component, Debug)]
pub struct Tile(u8);

#[derive(Component)]
pub struct Rectangle;

#[derive(Resource, Debug)]
pub struct Score {
    pub value: u32,
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Event)]
pub struct PopTiles(pub Vec<(Entity, Position)>);

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

    commands.insert_resource(Score { value: 0 });
    commands.spawn((
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
        let (mut visibility, mut transform) =
            rectangle.get_single_mut().expect("ggs no input rect lmao");

        transform.translation = position.extend(1.0);
        *visibility = Visibility::Visible;
    }
}

struct Bounds {
    upper: f32,
    lower: f32,
    left: f32,
    right: f32,
}

fn get_input_bounds(rect_transform: Transform) -> Bounds {
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

    return Bounds {
        upper: upper_bound,
        lower: lower_bound,
        left: left_bound,
        right: right_bound,
    };
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
        let (_, mut rect_transform) = rectangle.get_single_mut().expect("ggs no input rect lmao");
        rect_transform.scale = Vec3::new(
            rect_transform.translation.x - position.x,
            rect_transform.translation.y - position.y,
            1.0,
        );

        // input_rectangle is anchored to the topRight, so offset it by
        let bounds = get_input_bounds(*rect_transform);

        tiles
            .iter_mut()
            .for_each(|(_, tile_transform, mut sprite)| {
                if tile_transform.translation.x <= bounds.left
                    && tile_transform.translation.x >= bounds.right
                    && tile_transform.translation.y <= bounds.lower
                    && tile_transform.translation.y >= bounds.upper
                {
                    sprite.color = Color::srgb(0.20, 0.8, 0.70)
                } else {
                    sprite.color = Color::srgb(0.20, 0.3, 0.70)
                }
            });

        // *visibility = Visibility::Visible;
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
    let bounds = get_input_bounds(*rect_transform);

    let mut tiles_selected: Vec<_> = tiles
        .iter_mut()
        .filter(|(_, tile_transform, _, _)| {
            tile_transform.translation.x <= bounds.left
                && tile_transform.translation.x >= bounds.right
                && tile_transform.translation.y <= bounds.lower
                && tile_transform.translation.y >= bounds.upper
        })
        .collect();

    if tiles_selected.iter().map(|(_, _, _, t)| t.0).sum::<u8>() == 10 {
        commands.trigger(PopTiles(
            tiles_selected
                .iter()
                .map(|(e, t, _, _)| {
                    (
                        *e,
                        Position {
                            x: t.translation.x,
                            y: t.translation.y,
                        },
                    )
                })
                .collect(),
        ));
    } else {
        for (_, _, ref mut s, _) in &mut tiles_selected {
            s.color = Color::srgb(0.20, 0.3, 0.70);
        }
    }
}

fn handle_reset(mut tiles: Query<(Entity, &Transform, &mut Sprite), With<Tile>>) {
    for (_, _, mut sprite) in &mut tiles {
        sprite.color = Color::srgb(0.20, 0.3, 0.70);
    }
}

fn pop_tiles(
    trigger: Trigger<PopTiles>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
) {
    for (entity, pos) in trigger.event().0.iter() {
        println!("{:?}", pos);
        commands.entity(*entity).despawn_recursive();
        score.value += 1;
    }

    let mut text = score_board.single_mut();

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

fn refresh_scoreboard(mut score_board: Query<&mut Text, With<ScoreBoard>>) {
    let mut text = score_board.single_mut();
    if text.sections.len() > 4 {
        text.sections.remove(2);
    }
}
