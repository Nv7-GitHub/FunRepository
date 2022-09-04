pub const TARGET_SCORE: usize = 15;
const MAX_GEMS: usize = 10;
pub const COLS: usize = 4;
pub const ROWS: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Gem {
  Ruby,
  Emerald,
  Onyx,
  Sapphire,
  Diamond,
} // Cards can't be gold so ignoring that

impl Default for Gem {
  fn default() -> Self {
    Self::Ruby
  }
}

#[derive(Clone, Debug, Default)]
pub struct Card {
  pub points: usize,
  pub gem: Gem,
  pub requirements: Vec<Requirement>,
  pub pos: (usize, usize),
}

#[derive(Clone, Debug)]
pub struct Requirement {
  pub gem: Gem,
  pub count: usize,
}

pub struct Board {
  pub cards: Vec<Vec<Option<Card>>>,
}

impl Default for Board {
  fn default() -> Self {
    Self { cards: vec![vec![None; COLS]; ROWS] }
  }
}

mod solve;
mod display;
mod dist;
