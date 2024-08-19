use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
};

use crate::area_multiplier::{IntBounds, PrevArea};
use crate::squarea_core::{PopTiles, Rectangle, Score, ScoreBoard, TILE_GAP, TILE_SIZE};

pub struct ComboMultiplier;

impl Plugin for ComboMultiplier {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_combo);
        app.observe(apply_combo_multiplier);
    }
}

#[derive(Resource)]
pub struct Combo(u32);

fn init_combo(mut commands: Commands) {
    commands.insert_resource(Combo(0));
}

fn apply_combo_multiplier(
    trigger: Trigger<PopTiles>,
    prev_area: Query<&PrevArea>,
    mut combo: ResMut<Combo>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
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

    let prev_area = prev_area.get_single().expect("no prev area");

    let intersect = prev_area.0.intersect(&bounds);

    if intersect {
        combo.0 += 1;
        score.value += combo.0;

        println!("combo: + {:?}", combo.0);

        let mut text = score_board.single_mut();
        text.sections.push(TextSection {
            value: format!("\ncombo: + {}", combo.0),
            style: TextStyle {
                font_size: 20.,
                color: Color::srgb(1., 0.7, 0.8),
                ..default()
            },
        });
    } else {
        combo.0 = 0;
    }
}
