mod board;
use board::*;

fn main() {
    let mut b = Board::default();
    let mut inv = b.loadfile("board.txt");
    let res = b.solve(&mut inv);
    for card in res {
        if card.pos.0 < ROWS && card.pos.1 < COLS {
            println!("{}", card);
        }
    }
    /*for row in b.cards.iter() {
        for card in row {
            if let Some(c) = card {
                println!("{:?} {:?}", c, b.distance(c, &inv));
            }
        }
    }*/
}
