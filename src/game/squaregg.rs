use core::fmt;

use bevy::prelude::*;

use rand::{thread_rng, Rng};

use super::playing::TilesPoppedEvent;

pub const ROWS: usize = 11;
pub const COLS: usize = 18;
const DURATION: f32 = 200.;

#[derive(Debug, Clone)]
pub struct Area {
    pub upper: i32,
    pub lower: i32,
    pub left: i32,
    pub right: i32,
}

impl Default for Area {
    fn default() -> Self {
        Self {
            upper: i32::MIN,
            lower: i32::MAX,
            left: i32::MAX,
            right: i32::MIN,
        }
    }
}

impl Area {
    pub fn intersect(&self, other: &Area) -> bool {
        if self.right < other.left || other.right < self.left {
            return false;
        }

        if self.upper < other.lower || other.upper < self.lower {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

pub struct Squaregg {
    pub board: Vec<Vec<Option<i32>>>,
    pub score: i32,
    pub combo: i32,
    pub timer: Timer,
    pub prev_area: Area,
}

impl fmt::Debug for Squaregg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|tile| match tile {
                        Some(val) => val.to_string(),
                        None => " ".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .iter()
            .rev() // the way I spawned the tiles in board/mod.rs makes it upside down lmao so flip
            .for_each(|row| println!("{:?}", row));

        f.debug_struct("Point")
            // .field("board", &&self, board)
            .field("score", &self.score)
            .field("timer", &self.timer)
            .field("prev_area", &self.prev_area)
            .finish()
    }
}

impl Squaregg {
    pub fn new() -> Self {
        let mut rng_generator = thread_rng();

        Squaregg {
            board: (0..ROWS)
                .map(|_| {
                    (0..COLS)
                        .map(|_| Some(rng_generator.gen_range(1..10)))
                        .collect()
                })
                .collect(),
            score: 0,
            combo: 0,
            timer: Timer::from_seconds(DURATION, TimerMode::Repeating),
            prev_area: Area::default(),
        }
    }

    pub fn reset(&mut self) {
        let mut rng_generator = thread_rng();

        self.board = (0..ROWS)
            .map(|_| {
                (0..COLS)
                    .map(|_| Some(rng_generator.gen_range(1..10)))
                    .collect()
            })
            .collect();
        self.score = 0;
        self.timer = Timer::from_seconds(DURATION, TimerMode::Repeating);
        self.prev_area = Area::default();
    }

    // on success get the previous area
    pub fn try_pop_area(area: Area) -> Result<Area, ()> {
        Ok(Area::default())
    }

    pub fn try_pop_tiles(
        &mut self,
        tiles: &Vec<Position>,
        // mut ev: EventWriter<TilesPoppedEvent>,
        // mut commands: Commands
    ) -> bool {
        let mut bounds = Area::default();

        for pos in tiles {
            if (pos.row as i32) < bounds.lower {
                bounds.lower = pos.row as i32
            }

            if (pos.row as i32) > bounds.upper {
                bounds.upper = pos.row as i32
            }

            if (pos.col as i32) < bounds.left {
                bounds.left = pos.col as i32
            }

            if (pos.col as i32) > bounds.right {
                bounds.right = pos.col as i32
            }
        }

        if let Some(sum) = tiles.iter().try_fold(0i32, |acc, tile| {
            acc.checked_add(self.board[tile.row][tile.col].unwrap())
        }) {
            if sum == 10 {
                // also some event writer or something
                tiles.iter().for_each(|pos| {
                    self.board[pos.row][pos.col] = None;
                });

                let combo_multiplier = match combo_multiplier(&bounds, &self.prev_area, self.combo)
                {
                    Some(combo_multiplier) => {
                        self.combo += 1;
                        combo_multiplier
                    }
                    None => {
                        self.combo = 0;
                        0
                    }
                };

                self.score += tiles.len() as i32 + area_multiplier(&bounds) + combo_multiplier;

                self.prev_area = bounds.clone();

                // ev.send(TilesPoppedEvent {
                //     tiles: tiles.to_vec()
                // });

                return true
            }
        }

        false
    }
}

fn area_multiplier(area: &Area) -> i32 {
    let width = area.right - area.left + 1;
    let height = area.upper - area.lower + 1;
    height * width
}

fn combo_multiplier(area: &Area, prev_area: &Area, curr_combo: i32) -> Option<i32> {
    match area.intersect(prev_area) {
        true => {
            let n_combo = curr_combo + 1;
            let combo_bonus = match n_combo {
                0..6 => (n_combo as f32).powf(1.5 as f32) as i32,
                _ => n_combo * 2,
            };
            Some(combo_bonus)
        }
        false => None,
    }
}
