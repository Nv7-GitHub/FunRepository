mod board;
use board::*;
mod minimax;
use minimax::*;
use std::io::{self, stdout, Write};

fn print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}

fn clear() {
    print("\x1b[H\x1b[2J");
}
 

const MAX_DEPTH: usize = 5;

fn main() {
    let mut board = Board::new();
    clear();
    println!("{}", board);

    let mut player = Player::Min;
    let stdin = io::stdin();
    loop {
        // Get most optimal move
        let mut best_move = None;
        let opp = player.opposite();
        let mut best_move_score = match opp {
            Player::Min => NEGATIVE_INF,
            Player::Max => POSITIVE_INF,
            _ => 0,
        };
        for m in board.possible_moves(player) {
            let mut new = board;
            new.turn(player, m);
            let val = minimax(new, MAX_DEPTH, NEGATIVE_INF, POSITIVE_INF, player);
            match player {
                Player::Min => {
                    if val < best_move_score {
                        best_move_score = val;
                        best_move = Some(m);
                    }
                }
                Player::Max => {
                    if val > best_move_score {
                        best_move_score = val;
                        best_move = Some(m);
                    }
                }
                _ => {}
            }
        }
        if let Some(best) = best_move {
            println!("The most optimal move is {best:?}.");
        }

        // Get color
        let mut inp = String::new();
        print(format!("Enter a color ({player:?}): ").as_str());
        stdin.read_line(&mut inp).unwrap();
        let inp = inp.trim();
        let color = match inp {
            "quit" => break,
            "black" => Color::Black,
            "green" => Color::Green,
            "red" => Color::Red,
            "blue" => Color::Blue,
            "yellow" => Color::Yellow,
            "purple" => Color::Purple,
            _ => continue,
        };

        // Do turn
        board.turn(player, color);
        player = player.opposite();
        clear();
        println!("{}", board);

        // Check for winner
        if board.winner() != Player::Neutral {
            println!("{:?} won!", board.winner());
            return;
        }
    }
}
