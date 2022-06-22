use std::fmt::{Error, Formatter, Display};

use rand::Rng;
use rand::distributions::{Distribution, Standard};

const WIDTH: usize = 8;
const HEIGHT: usize = 7;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
  Red,
  Green,
  Yellow,
  Blue,
  Purple,
  Black,
}

const COLORS: [Color; 6] = [Color::Red, Color::Green, Color::Yellow, Color::Blue, Color::Purple, Color::Black];

impl Color {
  pub fn char(self) -> char {
    match self {
      Color::Red => '🟥',
      Color::Green => '🟩',
      Color::Yellow => '🟨',
      Color::Blue => '🟦',
      Color::Purple => '🟪',
      Color::Black => '⬛',
    }
  }
}

impl Distribution<Color> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
      match rng.gen_range(0..6) {
          0 => Color::Red,
          1 => Color::Green,
          2 => Color::Yellow,
          3 => Color::Blue,
          4 => Color::Purple,
          _ => Color::Black
      }
  }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Player {
  Max,
  Min,
  Neutral,
}

impl Player {
  pub fn opposite(self) -> Player {
    match self {
      Player::Max => Player::Min,
      Player::Min => Player::Max,
      Player::Neutral => Player::Neutral,
    }
  }
}

#[derive(Copy, Clone)]
pub struct Cell {
  pub color: Color,
  pub player: Player,
}

#[derive(Copy, Clone)]
pub struct Board {
  pub board: [[Cell; WIDTH]; HEIGHT],
}

struct Offset(i32, i32); // (row, col)

const OFFSETS: [Offset; 4] = [
  Offset(-1, 0),
  Offset(0, 1),
  Offset(1, 0),
  Offset(0, -1),
];

impl Board {
  pub fn new() -> Self {
    let mut board = [[Cell {
      color: Color::Black,
      player: Player::Neutral,
    }; WIDTH]; HEIGHT];
    
    let mut rng = rand::thread_rng();
    
    for i in 0..HEIGHT {
      for j in 0..WIDTH {
        loop {
          let color = rng.gen();
          // Check above
          if i > 0 {
            if board[i - 1][j].color == color {
              continue;
            }
          }
          // Check left
          if j > 0 {
            if board[i][j - 1].color == color {
              continue;
            }
          }
          // Update
          board[i][j].color = color;
          break;
        }
      }
    }
    
    // Make players
    board[HEIGHT-1][0].player = Player::Max;
    board[0][WIDTH-1].player = Player::Min;

    Self{ board }
  }

  fn eval_players(&mut self) {
    let mut new = *self;
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        if self.board[row][col].player == Player::Max || self.board[row][col].player == Player::Min {
          continue;
        }

        for off in OFFSETS {
          // Go through offsets
          let row_cur = row as i32 + off.0;
          let col_cur = col as i32 + off.1;
          if (row_cur < 0) || (row_cur >= HEIGHT as i32) || (col_cur < 0) || (col_cur >= WIDTH as i32) {
            continue;
          }

          // Check cell
          let cell = self.board[row_cur as usize][col_cur as usize];
          if cell.player != Player::Neutral && cell.color == self.board[row][col].color {
            new.board[row][col].player = cell.player; // Update
            break;
          }
        }
      }
    }
    *self = new;
  }

  pub fn turn(&mut self, player: Player, color: Color) {
    // Update color of all your squares
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        if self.board[row][col].player == player {
          self.board[row][col].color = color;
        }
      }
    }
    // Eval
    self.eval_players();
  }

  pub fn winner(&self) -> Player {
    // Check if there is a winner
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        if self.board[row][col].player == Player::Neutral {
          return Player::Neutral;
        }
      }
    }
    if self.count() > 0 {
      return Player::Max;
    }
    return Player::Min;
  }

  pub fn count(&self) -> i32 {
    let mut out = 0;
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        match self.board[row][col].player {
          Player::Max => out += 1,
          Player::Min => out -= 1,
          _ => {}
        }
      }
    }
    return out;
  }

  pub fn possible_moves(&self, player: Player) -> Vec<Color> {
    let opposite = player.opposite();
    let mut opp_color = Color::Black;
    let mut your_color = Color::Black;
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        if self.board[row][col].player == opposite {
          opp_color = self.board[row][col].color;
        }
        if self.board[row][col].player == player {
          your_color = self.board[row][col].color;
        }
      }
    }
    let mut out = Vec::with_capacity(5);
    for col in COLORS {
      if col != opp_color && col != your_color {
        out.push(col);
      }
    }
    out
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
    for row in 0..HEIGHT {
      for col in 0..WIDTH {
        write!(f, "{}", self.board[row][col].color.char())?;
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}