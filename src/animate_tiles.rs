use crate::squarea_core::{PopTiles, Score, ScoreBoard};
use bevy::{
    animation::AnimationTarget,
    ecs::observer::TriggerTargets,
    input::common_conditions::{input_just_pressed, input_just_released, input_pressed},
    prelude::*,
    sprite::Anchor,
    window::PrimaryWindow,
};
pub struct AnimateTiles;

impl Plugin for AnimateTiles {
    fn build(&self, app: &mut App) {
        app.observe(animate_tiles);
    }
}

fn animate_tiles(trigger: Trigger<PopTiles>) {
    trigger.event().0.iter().for_each(|(e, p)| {})
}
