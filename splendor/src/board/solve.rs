use enum_map::EnumMap;

use super::*;

impl Board {
  fn distance(&self, card: Card, cards: Vec<Card>) -> usize{
    // Check how much left
    let mut reqs: EnumMap<Gem, usize> = EnumMap::default();
    for req in card.requirements {
      reqs[req.gem] = req.count;
    }
    for card in cards.iter() {
      if reqs[card.gem] > 0 {
        reqs[card.gem] = reqs[card.gem] - 1;
      }
    }

    // Calculate turns
    let mut count = 0;
    for (_, cnt) in reqs {
      count += cnt;
    }
    let mut turns = count / 3; // Can get 3 at a time in the strt
    count %= 3;
    turns += count/2; // Have to get 2 at a time since then

    // Return
    turns
  }

  fn iddfs(&self, cards: Vec<Card>) {

  }

  pub fn solve(&self, cards: Vec<Card>) {

  }
}