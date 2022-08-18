use enum_map::EnumMap;

use super::*;

const TARGET_SCORE: usize = 15;

impl Board {
  fn distance(&self, card: &Card, cards: &Vec<Card>) -> usize{
    // Check how much left
    let mut reqs: EnumMap<Gem, usize> = EnumMap::default();
    for req in card.requirements.iter() {
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

  fn iddfs(&mut self, cards: &mut Vec<Card>, depth: usize) -> Option<Vec<Card>> {
    // Check if done
    let mut score = 0;
    for card in cards.iter() {
      score += card.points;
    }
    if score >= TARGET_SCORE {
      return Some(cards.into_iter().map(|x| x.clone()).collect());
    }

    // Check if past max depth
    if depth == 0 {
      return None;
    }

    // Go through possible cards
    let mut vals: Vec<(usize, usize, usize)> = Vec::with_capacity(ROWS * COLS);
    for r in 0..ROWS {
      for c in 0..COLS {
        // Add card to inv and remove from table
        if let Some(card) = &self.cards[r][c] {
          vals.push((r, c, self.distance(card, cards)));
        }
      }
    }
    vals.sort_by(|a, b| b.2.cmp(&a.2));

    for (r, c, _) in vals {
      // Buy card
      let card = self.cards[r].remove(c);
      self.cards[r].insert(c, None);
      cards.push(card.as_ref().unwrap().clone());

      // Calculate
      let res = self.iddfs(cards, depth - 1);
      if let Some(out) = res {
        return Some(out); // If it works return
      }
      
      // Return to original state
      self.cards[r][c] = card;
      cards.pop().unwrap();
    }

    None
  }

  pub fn solve(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
    let mut depth = 1;
    loop {
      let res = self.iddfs(cards, depth);
      if let Some(val) = res {
        return val;
      }
      println!("Checking depth {depth}...");
      depth += 1;
    }
  }
}