use core::f32;

use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use crate::squarea_core::{PopTiles, Score, ScoreBoard, TILE_GAP, TILE_SIZE};
pub struct AreaMultiplier;

impl Plugin for AreaMultiplier {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_area);
        app.observe(apply_area_multiplier);
    }
}

#[derive(Component)]
pub struct PrevArea;

fn setup_area(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.).with_scale(Vec3::new(
                TILE_SIZE + TILE_GAP,
                TILE_SIZE + TILE_GAP,
                1.,
            )),
            sprite: Sprite {
                color: Color::srgba(0.9, 0.8, 0.2, 0.2),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        PrevArea,
    ));
}

fn apply_area_multiplier(
    trigger: Trigger<PopTiles>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
    mut prev_area: Query<(&mut Transform, &mut Visibility), With<PrevArea>>,
) {
    let mut left_bound = f32::MAX;
    let mut right_bound = f32::MIN;
    let mut lower_bound = f32::MAX;
    let mut upper_bound = f32::MIN;

    for (entity, pos) in trigger.event().0.iter() {
        if pos.x < left_bound {
            left_bound = pos.x
        }

        if pos.x > right_bound {
            right_bound = pos.x
        }

        if pos.y < lower_bound {
            lower_bound = pos.y
        }

        if pos.y > upper_bound {
            upper_bound = pos.y
        }
    }

    let height = if upper_bound - lower_bound == 0. {
        1.
    } else {
        upper_bound - lower_bound
    };

    let width = if right_bound - left_bound == 0. {
        1.
    } else {
        right_bound - left_bound
    };

    let area = height * width / (TILE_SIZE + TILE_GAP) + 1.;
    println!("area multiplier {area}",);
    score.value += area as u32;

    let (mut prev_area_transform, mut visibility) =
        prev_area.get_single_mut().expect("no prev area ggs");
    prev_area_transform.translation = Vec3::new(
        (left_bound + right_bound) / 2.,
        (lower_bound + upper_bound) / 2.,
        1.,
    );

    prev_area_transform.scale = Vec3::new(
        if width == 1. {
            TILE_SIZE + TILE_GAP
        } else {
            width + (TILE_SIZE + TILE_GAP)
        },
        if height == 1. {
            TILE_SIZE + TILE_GAP
        } else {
            height + (TILE_SIZE + TILE_GAP)
        },
        1.,
    );

    *visibility = Visibility::Visible;

    let mut text = score_board.single_mut();
    text.sections[1].value = score.value.to_string();

    text.sections.push(TextSection {
        value: format!("\narea multiplier: + {area}"),
        style: TextStyle {
            font_size: 20.,
            color: Color::srgb(1., 0.7, 0.8),
            ..default()
        },
    })
}
