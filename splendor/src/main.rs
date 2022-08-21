mod board;
use board::*;

fn main() {
    let mut b = Board::default();
    let mut inv = b.loadfile("board.txt");
    let res = b.solve(&mut inv);
    for card in res {
        println!("{}", card);
    }
}
