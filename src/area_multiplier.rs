use bevy::prelude::{
    default, App, Color, Commands, Plugin, Query, ResMut, Sprite, SpriteBundle, Startup, Text,
    TextSection, TextStyle, Transform, Trigger, Vec3, Visibility, With,
};

use crate::conversions::{IntBounds, PrevArea};
use crate::squarea_core::{PopTiles, Score, ScoreBoard, TILE_GAP, TILE_SIZE};
pub struct AreaMultiplier;

impl Plugin for AreaMultiplier {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_area);
        app.observe(apply_area_multiplier);
    }
}

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
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
    mut prev_area: Query<(&mut Transform, &mut Visibility, &mut PrevArea)>,
) {
    let bounds = IntBounds::from_positions(trigger.event().0.iter().map(|(_, p)| p).collect());

    let width = bounds.right - bounds.left + 1;
    let height = bounds.upper - bounds.lower + 1;
    let area = height * width;

    println!("area multiplier: + {area}");
    score.value += area as u32;

    let (mut prev_area_transform, mut visibility, mut prev_area) =
        prev_area.get_single_mut().expect("no prev area ggs");

    prev_area_transform.translation = bounds.to_translation().extend(1.);
    prev_area_transform.scale = bounds.to_scale().extend(1.);

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
