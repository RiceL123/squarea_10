use bevy::{math::Vec2, prelude::Component};

use crate::squarea_core::{Position, COLS, ROWS, TILE_GAP, TILE_SIZE};

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

    pub fn from_positions(positions: Vec<&Position>) -> Self {
        let mut bounds = IntBounds::default();

        for pos in positions {
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

        bounds
    }

    pub fn to_translation(&self) -> Vec2 {
        Vec2::new(
            (0.5 + (self.right + self.left) as f32 / 2. - (COLS as f32 / 2.))
                * (TILE_SIZE + TILE_GAP),
            (0.5 + (self.upper + self.lower) as f32 / 2. - (ROWS as f32 / 2.))
                * (TILE_SIZE + TILE_GAP),
        )
    }

    pub fn to_scale(&self) -> Vec2 {
        Vec2::new(
            (self.right - self.left + 1) as f32 * (TILE_SIZE + TILE_GAP),
            (self.upper - self.lower + 1) as f32 * (TILE_SIZE + TILE_GAP),
        )
    }
}

#[derive(Component)]
pub struct PrevArea(pub IntBounds);
