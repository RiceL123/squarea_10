use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
};

use crate::area_multiplier::PrevArea;
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
    area: Query<&Transform, With<Rectangle>>,
    prev_area: Query<&Transform, With<PrevArea>>,
    mut combo: ResMut<Combo>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
) {
    let area_transform = area.get_single().expect("no selected area");
    let prev_area_transofrm = prev_area.get_single().expect("no prev area");

    println!("area: {:?}", area_transform.translation);
    println!("prev area: {:?}", prev_area_transofrm.translation);

    let intersect = true;

    if intersect {
        score.value += combo.0;

        println!("combo: {:?}", combo.0);

        if combo.0 > 0 {
            let mut text = score_board.single_mut();
            text.sections.push(TextSection {
                value: format!("\ncombo: + {}", combo.0),
                style: TextStyle {
                    font_size: 20.,
                    color: Color::srgb(1., 0.7, 0.8),
                    ..default()
                },
            });
        }

        combo.0 += 1;
    } else {
        combo.0 = 0;
    }
}
