use enum_map::Enum;

#[derive(Enum, Clone, Copy, Debug)]
pub enum Gem {
  Ruby ,
  Emerald,
  Onyx,
  Sapphire,
  Diamond,
  Gold,
}

pub const COLS: usize = 4;
pub const ROWS: usize = 3;

#[derive(Clone, Debug)]
pub struct Card {
  pub points: usize,
  pub gem: Gem,
  pub requirements: Vec<Requirement>,
}

#[derive(Clone, Debug)]
pub struct Requirement {
  pub gem: Gem,
  pub count: usize,
}

pub struct Board {
  pub cards: Vec<Vec<Option<Card>>>,
}

mod solve;
