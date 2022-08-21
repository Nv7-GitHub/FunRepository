use std::{io::{self, Write}, fmt::Display, str::FromStr, string::ParseError};

use super::*;

fn clear() {
  print!("\x1b[H\x1b[2J");
  io::stdout().flush().unwrap();
}

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

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    for (r, line) in self.cards.iter().enumerate() {
      for (c, card) in line.into_iter().enumerate() {
        if self.highlight[r][c] {
          print!("\x1b[47m");
        }

        match card {
          None => write!(f, "\x1b[35mX\x1b[0m"),
          Some(v) => write!(f, "{}", v.gem),
        }?;

        if self.highlight[r][c] {
          print!("\x1b[0m");
        }
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

impl FromStr for Gem {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "ruby" => Self::Ruby,
      "emerald" => Self::Emerald,
      "onyx" => Self::Onyx,
      "sapphire" => Self::Sapphire,
      "diamond" => Self::Diamond,
      _ => panic!("invalid input"),
    })
  }
}

impl Board {
  fn inp(&self, prompt: &str) -> String {
    clear();
    println!("{}", self);

    // Print
    print!("{}", prompt);
    io::stdout().flush().unwrap();
  
    // Read
    let mut out = String::new();
    io::stdin().read_line(&mut out).unwrap();
  
    // Remove trailing newline
    if let Some('\n') = out.chars().next_back() {
      out.pop().unwrap();
    }
  
    out
  }

  fn edit_iter(&mut self, inv: &mut Vec<Card>) {
    loop {
      self.reset_highlight();
      
      let op = self.inp("Operation (edit, remove, addinv, done): ");
      match op.as_str() {
        "remove" => {
          let row: usize = self.inp("Row: ").parse().unwrap();
          let col: usize = self.inp("Column: ").parse().unwrap();
          self.cards[row][col] = None;
        }
        "edit" => {
          // Get pos
          let row: usize = self.inp("Row: ").parse().unwrap();
          let col: usize = self.inp("Column: ").parse().unwrap();
          self.highlight[row][col] = true;

          // Get card color
          let gem: Gem = self.inp("Gem: ").parse().unwrap();
          let points: usize = self.inp("Points: ").parse().unwrap();

          // Get requirements
          let mut reqs = Vec::new();
          loop {
            if self.inp("Done inputting requirements? (y/n): ") == "y" { // Done editing?
              break;
            }

            // Get params
            let gem: Gem = self.inp("Gem: ").parse().unwrap();
            let cnt: usize = self.inp("Count: ").parse().unwrap();
            reqs.push(Requirement { gem, count: cnt });
          }

          // Make card
          self.cards[row][col] = Some(Card{
            points,
            gem,
            requirements: reqs,
            pos: (row, col),
          });
          self.reset_highlight();
        }
        "done" => break,
        "addinv" => {
          let row: usize = self.inp("Row: ").parse().unwrap();
          let col: usize = self.inp("Column: ").parse().unwrap();
          let card = self.cards[row].remove(col).unwrap();
          self.cards[row].insert(col, None);
          inv.push(card);
        }
        _ => panic!("invalid input"),
      }
    }
  }

  pub fn edit(&mut self) {
    let mut inv = Vec::new();
    loop {
      // Let editing happen
      self.edit_iter(&mut inv);

      // Solve
      clear();
      let res = self.solve(&mut inv);

      // Display
      self.reset_highlight();
      for c in res.iter() {
        self.highlight[c.pos.0][c.pos.1] = true;
      }
      self.inp("Press ENTER to continue editing...");
    }
  }
}