use raylib::prelude::*;
use raylib::consts::KeyboardKey;
use rand::Rng;

const WIDTH: i32 = 750;
const HEIGHT: i32 = 750;
const ROWS: i32 = 25;
const COLS: i32 = 25;
const SPEED: i32 = 5;
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

    fn colliding(&self, other: Block) -> bool {
        other.x > self.x && other.x < (self.x + WIDTH/COLS) && other.y > self.y && other.y < (self.y + HEIGHT/ROWS)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

impl Dir {
    fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }
}

fn push_dir(dir: Dir, blocks: &Vec<Block>, q: &mut Vec<Dir>) {
    if q.len() > 0 {
        if (*q.last().unwrap()).opposite() == dir {
            return;
        }
    } else if blocks[0].dir.opposite() == dir {
        return;
    }
    q.push(dir);
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

    let mut gameover: bool = false;

    while !rl.window_should_close() {
        if gameover {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);
            continue;
        }

        // Add to end if needed
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
        
        // Apply movement
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

        // Collision checks
        if blocks.len() > 1 {
            for i in 1..blocks.len() {
                if blocks[i].dir == blocks[i-1].dir && blocks[i].colliding(blocks[0]) {
                    gameover = true;
                }
            }
        }   

        // Get input
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            push_dir(Dir::Down, &blocks, &mut queue);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            push_dir(Dir::Up, &blocks, &mut queue);
        }
        if rl.is_key_released(KeyboardKey::KEY_LEFT) {
            push_dir(Dir::Left, &blocks, &mut queue);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            push_dir(Dir::Right, &blocks, &mut queue);
        }

        // Check if ready for input
        if blocks[0].x % (WIDTH/COLS) < SNAP_RANGE && blocks[0].y % (HEIGHT/ROWS) < SNAP_RANGE && queue.len() > 0 && (blocks.len() == 1 || blocks[0].dir == blocks[1].dir)  {
            let first = blocks[0];
            for block in blocks.iter_mut() {
                block.x -= first.x % (WIDTH/COLS);
                block.y -= first.y % (HEIGHT/ROWS);
            }
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