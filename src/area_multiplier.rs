use core::f32;
use std::u8;

use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};

use crate::squarea_core::{
    Bounds, PopTiles, Position, Score, ScoreBoard, COLS, ROWS, TILE_GAP, TILE_SIZE,
};
pub struct AreaMultiplier;

impl Plugin for AreaMultiplier {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_area);
        app.observe(apply_area_multiplier);
    }
}

#[derive(Clone, Debug)]
pub struct IntBounds {
    pub upper: u8,
    pub lower: u8,
    pub left: u8,
    pub right: u8,
}

impl Default for IntBounds {
    fn default() -> Self {
        IntBounds {
            upper: u8::MIN,
            lower: u8::MAX,
            left: u8::MAX,
            right: u8::MIN,
        }
    }
}

impl IntBounds {
    pub fn intersect(&self, other: &IntBounds) -> bool {
        if self.right < other.left || other.right < self.left {
            return false;
        }

        if self.upper < other.lower || other.upper < self.lower {
            return false;
        }

        true
    }
}

#[derive(Component)]
pub struct PrevArea(pub IntBounds);

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
        PrevArea(IntBounds::default()),
    ));
}

fn apply_area_multiplier(
    trigger: Trigger<PopTiles>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
    mut prev_area: Query<(&mut Transform, &mut Visibility, &mut PrevArea)>,
) {
    let mut bounds = IntBounds::default();

    for (entity, pos) in trigger.event().0.iter() {
        if pos.row < bounds.lower {
            bounds.lower = pos.row
        }

        if pos.row > bounds.upper {
            bounds.upper = pos.row
        }

        if pos.col < bounds.left {
            bounds.left = pos.col
        }

        if pos.col > bounds.right {
            bounds.right = pos.col
        }
    }

    let height = bounds.right - bounds.left + 1;
    let width = bounds.upper - bounds.lower + 1;
    let area = height * width;

    println!("area multiplier: + {area}");
    score.value += area as u32;

    let (mut prev_area_transform, mut visibility, mut prev_area) =
        prev_area.get_single_mut().expect("no prev area ggs");

    prev_area_transform.translation = Vec3::new(
        (0.5 + (bounds.right + bounds.left) as f32 / 2. - (COLS as f32 / 2.))
            * (TILE_SIZE + TILE_GAP),
        (0.5 + (bounds.upper + bounds.lower) as f32 / 2. - (ROWS as f32 / 2.))
            * (TILE_SIZE + TILE_GAP),
        1.,
    );

    prev_area_transform.scale = Vec3::new(
        height as f32 * (TILE_SIZE + TILE_GAP),
        width as f32 * (TILE_SIZE + TILE_GAP),
        1.,
    );

    *visibility = Visibility::Visible;

    prev_area.0 = bounds;

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
