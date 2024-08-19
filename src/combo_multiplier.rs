use bevy::prelude::{
    default, App, Color, Commands, Plugin, Query, ResMut, Resource, Startup, Text, TextSection,
    TextStyle, Trigger, With,
};

use crate::conversions::{IntBounds, PrevArea};
use crate::squarea_core::{PopTiles, Score, ScoreBoard};

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
    mut score: ResMut<Score>,
    mut score_board: Query<&mut Text, With<ScoreBoard>>,
) {
    let bounds = IntBounds::from_positions(trigger.event().0.iter().map(|(_, p)| p).collect());

    let prev_area = prev_area.get_single().expect("no prev area");

    let intersect = prev_area.0.intersect(&bounds);

    if intersect {
        combo.0 += 1;
        let combo_bonus = match combo.0 {
            0..6 => (combo.0 as f32).powf(1.5 as f32) as u32,
            _ => combo.0 * 2,
        };
        // let combo_bonus = (combo.0 as f32).powf(1.5 as f32) as u32;
        score.value += combo_bonus;

        println!("combo: x{} (+{})", combo.0, combo_bonus);

        let mut text = score_board.single_mut();
        text.sections.push(TextSection {
            value: format!("\ncombo: x{} (+{})", combo.0, combo_bonus),
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
