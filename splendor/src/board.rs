pub enum Gem {
  Ruby,
  Emerald,
  Onyx,
  Sapphire,
  Diamond,
  Gold,
}

pub const COLS: usize = 4;
pub const ROWS: usize = 3;

pub struct Card {
  pub points: usize,
  pub gem: Gem,
  pub requirements: Vec<Requirement>,
}

pub struct Requirement {
  pub gem: Gem,
  pub count: usize,
}

pub struct Board {
  pub cards: [[Card; COLS]; ROWS],
}

