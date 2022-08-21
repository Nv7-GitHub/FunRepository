use std::fmt::Display;

use super::*;

impl Display for Gem {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Self::Onyx => write!(f, "\x1b[30;1mO\x1b[0m"),
      Self::Ruby => write!(f, "\x1b[31;1mR\x1b[0m"),
      Self::Emerald => write!(f, "\x1b[32;1mE\x1b[0m"),
      Self::Sapphire => write!(f, "\x1b[34;1mS\x1b[0m"),
      Self::Diamond => write!(f, "\x1b[37;1mD\x1b[0m"),
    }
  }
}

impl Display for Card {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({}, {}) - {}{}", self.pos.0 + 1, self.pos.1 + 1, self.points, self.gem) // (row, col) - <points><gem>
  }
}

impl Board {
  pub fn loadfile(&mut self, file: &str) -> Vec<Card> {
    let mut inv = Vec::new();
    let res = std::fs::read_to_string(file).unwrap();
    let mut row = 0;
    let mut col = 0;
    for (ind, line) in res.split("\n").enumerate() {
      // Parse card
      let mut card = Card::default();
      for i in 0..line.len()/2 {
        let cnt: usize = (line.as_bytes()[i*2] as char).to_string().parse().unwrap();
        let gem = match line.as_bytes()[i*2+1] as char {
          'r' => Gem::Ruby,
          'e' => Gem::Emerald,
          'o' => Gem::Onyx,
          's' => Gem::Sapphire,
          'd' => Gem::Diamond,
          _ => panic!("invalid gem letter: {} at line {} column {}", line.as_bytes()[i] as char, ind + 1, i*2 + 2),
        };

        if i == 0 {
          card.gem = gem;
          card.points = cnt;
        } else {
          card.requirements.push(Requirement{gem, count: cnt});
        }
      }

      // Save
      if row <= ROWS {
        card.pos = (row, col);
        self.cards[row][col] = Some(card);
  
        col += 1;
        if col >= COLS {
          row += 1;
          col = 0;
        }
      } else {
        card.pos = (ROWS, COLS); // Out of bounds
        inv.push(card);
      }
    }
    
    inv
  }
}