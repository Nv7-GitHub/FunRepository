use enum_map::Enum;

pub const TARGET_SCORE: usize = 15;
pub const COLS: usize = 4;
pub const ROWS: usize = 3;

#[derive(Enum, Clone, Copy, Debug)]
pub enum Gem {
  Ruby,
  Emerald,
  Onyx,
  Sapphire,
  Diamond,
} // Cards can't be gold so ignoring that

#[derive(Clone, Debug)]
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

  // For editing
  highlight: Vec<Vec<bool>>,
}

impl Default for Board {
  fn default() -> Self {
    Self { cards: vec![vec![None; COLS]; ROWS], highlight: vec![vec![false; COLS]; ROWS] }
  }
}

impl Board {
  fn reset_highlight(&mut self) {
    self.highlight = vec![vec![false; COLS]; ROWS];
  }
}

mod solve;
mod edit;
