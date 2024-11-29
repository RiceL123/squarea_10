use bevy::prelude::*;

use crate::{game::squaregg::{Area, COLS, ROWS}, menu::settings::GameConfig};

pub struct RectBounds {
    upper: f32,
    lower: f32,
    left: f32,
    right: f32,
}

impl RectBounds {
    pub fn new(rect_transform: &Transform) -> Self {
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

        return RectBounds {
            upper: upper_bound,
            lower: lower_bound,
            left: left_bound,
            right: right_bound,
        };
    }

    pub fn contains(&self, transform_object: &Transform) -> bool {
        transform_object.translation.x <= self.left
            && transform_object.translation.x >= self.right
            && transform_object.translation.y <= self.lower
            && transform_object.translation.y >= self.upper
    }

    pub fn area_to_rectbounds(area: Area, config: Res<GameConfig>) {
        let translation = Vec2::new(
            (0.5 + (area.right + area.left) as f32 / 2. - (COLS as f32 / 2.))
                * (config.tile_size + config.tile_gap),
            (0.5 + (area.upper + area.lower) as f32 / 2. - (ROWS as f32 / 2.))
                * (config.tile_size + config.tile_gap),
        );
        
        let scale = Vec2::new(
            (area.right - area.left + 1) as f32 * (config.tile_size + config.tile_gap),
            (area.upper - area.lower + 1) as f32 * (config.tile_size + config.tile_gap),
        );

        }
}

