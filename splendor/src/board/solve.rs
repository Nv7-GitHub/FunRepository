use super::*;

impl Board {
  pub fn distance(&self, card: &Card, cards: &Vec<Card>) -> Option<usize> {
    // Calculate count
    let mut count = 0;
    for req in card.requirements.iter() {
      count += req.count;
    }
    for crd in cards.iter() {
      if card.requirements.iter().any(|req| req.gem == crd.gem) {
        if count > 0 {
          count -= 1;
        }
      }
    }
    if count >= MAX_GEMS {
      return None; // Impossible to get
    }

    // Calculate turns
    let mut turns = count / 3; // Can get 3 at a time in the start
    count %= 3;
    turns += (count + 1) / 2; // Have to get 2 at a time since then

    // Return
    Some(turns)
  }

  fn iddfs(&mut self, cards: &mut Vec<Card>, cardturns: usize, depth: usize) -> Option<Vec<Card>> {
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
    for r in 0..ROWS {
      for c in 0..COLS {
        // Add card to inv and remove from table
        if let Some(card) = &self.cards[r][c] {
          if let Some(dist) = self.distance(card, cards) { // Able to get card
            if dist + cardturns > depth { // Incorrect depth
              continue;
            }
  
            // Buy card
            let card = self.cards[r][c].take();
            cards.push(card.as_ref().unwrap().clone());
  
            // Calculate
            let res = self.iddfs(cards, cardturns + dist, depth - 1);
  
            // Return to original state
            self.cards[r][c] = card;
  
            // Check
            if let Some(out) = res {
              return Some(out); // If it works return
            } else {
              cards.pop(); // Incorrect, remove from strategy
            }
          }
        }
      }
    }
    
    // None of the cards work, return
    None
  }

  pub fn solve(&mut self, cards: &mut Vec<Card>) -> Vec<Card> {
    let mut depth = 1;
    loop {
      let res = self.iddfs(cards, 0, depth);
      if let Some(val) = res {
        return val;
      }
      println!("Checking depth {depth}...");
      depth += 1;
    }
  }
}