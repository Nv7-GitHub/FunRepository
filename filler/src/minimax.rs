use super::*;

pub const NEGATIVE_INF: i32 = i32::MIN;
pub const POSITIVE_INF: i32 = i32::MAX;

pub fn minimax(board: Board, depth: usize, mut alpha: i32, mut beta: i32, player: Player) -> i32 {
  if depth == 0 || board.winner() != Player::Neutral {
    return board.count();
  }

  match player {
    Player::Max => {
      let mut max = NEGATIVE_INF;
      for col in board.possible_moves(player) {
        let mut new = board;
        new.turn(player, col);
        let res = minimax(new, depth - 1, alpha, beta, Player::Min);
        if res > max {
          max = res;
        }
        if res > alpha {
          alpha = res;
        }
        if beta <= alpha {
          break;
        }
      }

      max
    }
    Player::Min => {
      let mut min = POSITIVE_INF;
      for col in board.possible_moves(player) {
        let mut new = board;
        new.turn(player, col);
        let res = minimax(new, depth - 1, alpha, beta, Player::Max);
        if res < min {
          min = res;
        }
        if res < beta {
          beta = res;
        }
        if beta <= alpha {
          break;
        }
      }

      min
    }
    Player::Neutral => 0,
  }
}