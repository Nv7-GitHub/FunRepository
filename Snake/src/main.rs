use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use rand::Rng;

const WIDTH: i32 = 750;
const HEIGHT: i32 = 750;
const ROWS: i32 = 25;
const COLS: i32 = 25;
const SPEED: i32 = 3;
const SNAP_RANGE: i32 = 2;

#[derive(Clone, Copy, Debug)]
struct Block {
    x: i32,
    y: i32,
    dir: Dir,
    dirchangex: i32,
    dirchangey: i32,
}

impl Block {
    fn new(x: i32, y: i32, dir: Dir) -> Self {
        Self { x, y, dir, dirchangex: -1, dirchangey: -1 }
    }

    fn apply_movement(&mut self, dir: Dir, val: i32) {
        match dir {
            Dir::Up => self.y -= val,
            Dir::Down => self.y += val,
            Dir::Left => self.x -= val,
            Dir::Right => self.x += val,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

    rl.set_target_fps(60);
    
    let mut blocks = vec![Block::new(0, 0, Dir::Down)];
    let mut queue: Vec<Dir> = Vec::new();
    let mut rng = rand::thread_rng();

    let mut apple_x = rng.gen_range(0..COLS) * HEIGHT/COLS;
    let mut apple_y = rng.gen_range(0..ROWS) * WIDTH/ROWS;
    let mut append = false;

    while !rl.window_should_close() {
        // Apply movement
        if append {
            let last = blocks[blocks.len()-1];
            match last.dir {
                Dir::Up => blocks.push(Block::new(last.x, last.y + HEIGHT/ROWS, last.dir)),
                Dir::Down => blocks.push(Block::new(last.x, last.y - HEIGHT/ROWS, last.dir)),
                Dir::Left => blocks.push(Block::new(last.x + WIDTH/COLS, last.y, last.dir)),
                Dir::Right => blocks.push(Block::new(last.x - WIDTH/COLS, last.y, last.dir)),
            }
            
            append = false;
        }

        for block in blocks.iter_mut() {
            block.apply_movement(block.dir, SPEED);
        }
        let mut i = blocks.len()-1;
        while i > 0 {
            if blocks[i].dir != blocks[i-1].dir && blocks[i].x == blocks[i-1].dirchangex && blocks[i].y == blocks[i-1].dirchangey {
                blocks[i].dir = blocks[i-1].dir;
                blocks[i].dirchangex = blocks[i-1].dirchangex;
                blocks[i].dirchangey = blocks[i-1].dirchangey;
            }
            i -= 1;
        }

        // Get input
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            queue.push(Dir::Down);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            queue.push(Dir::Up);
        }
        if rl.is_key_released(KeyboardKey::KEY_LEFT) {
            queue.push(Dir::Left);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            queue.push(Dir::Right);
        }

        // Check if ready for input
        if blocks[0].x % (WIDTH/COLS) < SNAP_RANGE && blocks[0].y % (HEIGHT/ROWS) < SNAP_RANGE && queue.len() > 0  {
            blocks[0].x -= blocks[0].x % (WIDTH/COLS);
            blocks[0].y -= blocks[0].y % (HEIGHT/ROWS);
            blocks[0].dir = queue.remove(0);
            blocks[0].dirchangex = blocks[0].x;
            blocks[0].dirchangey = blocks[0].y;
        }

        // Check if in apple
        if (blocks[0].y - apple_y).abs() < (HEIGHT/ROWS) && (blocks[0].x - apple_x).abs() < (WIDTH/COLS) {
            apple_x = rng.gen_range(0..COLS) * HEIGHT/ROWS;
            apple_y = rng.gen_range(0..ROWS) * WIDTH/COLS;
            append = true;
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.draw_fps(10, 10);
        d.clear_background(Color::WHITE);
        for block in blocks.iter() {
            d.draw_rectangle(block.x, block.y, WIDTH/COLS, HEIGHT/ROWS, Color::BLACK);
        }
        d.draw_circle(apple_x + (WIDTH/COLS/2), apple_y + (HEIGHT/ROWS/2), (HEIGHT/ROWS) as f32/2.0, Color::RED);
    }
}