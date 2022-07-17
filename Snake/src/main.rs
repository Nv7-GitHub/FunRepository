use raylib::prelude::*;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 500;
const ROWS: i32 = 25;
const COLS: i32 = 25;
const SPEED: i32 = 1;

struct Block {
    x: i32,
    y: i32,
}

impl Block {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Snake")
        .build();
    
    let blocks = vec![Block::new(0, 0)];
    let dir = Dir::Down;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        for block in blocks.iter() {
            d.draw_rectangle(block.x, block.y, WIDTH/COLS, HEIGHT/ROWS, Color::BLACK);
        }
    }
}