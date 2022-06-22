mod board;
use board::*;
use std::io::{self, stdout, Write};

fn print(msg: &str) {
    print!("{}", msg);
    stdout().flush().unwrap();
}

fn clear() {
    print("\x1b[H\x1b[2J");
}
  

fn main() {
    let mut board = Board::new();
    clear();
    println!("{}", board);

    let mut player = Player::Min;
    let stdin = io::stdin();
    loop {
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
        player = match player {
            Player::Min => Player::Max,
            Player::Max => Player::Min,
            Player::Neutral => Player::Neutral,
        };
        clear();
        println!("{}", board);
    }
}
