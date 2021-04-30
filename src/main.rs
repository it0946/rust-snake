use rand::{Rng, thread_rng};
use std::{io, thread, time};

const BOARD_SIZE: usize = 12;

fn in_arr(nums: (u8, u8), snake_parts: &Vec<(u8, u8)>) -> bool {
    for i in 0..snake_parts.len() { if snake_parts[i] == nums {return true} }
    false
}

enum Direction {Up, Down, Left, Right}

struct Board {
    //field: [[char;BOARD_SIZE];BOARD_SIZE],
    snake_parts: Vec<(u8, u8)>,
    parts: i8,
    apple_location: (u8, u8),
    lost: bool,
    score: i8,
    direction: Direction
}

impl Board {
    fn new() -> Board {
        Board {
            //field : [['.';BOARD_SIZE];BOARD_SIZE],
            snake_parts : vec![(3,3);3],
            parts: 3,
            apple_location : Board::new_apple(),
            lost : false,
            score: 0,
            direction : Direction::Up
        }
    }
    fn new_apple() -> (u8, u8) {
        let mut rng = thread_rng();
        (rng.gen_range(0..8), rng.gen_range(0..8))
    }
    fn update_snake(&mut self, dir: Direction) {
        for part in &self.snake_parts {
            match dir {
                Direction::Up => {
                    if matches!(self.direction, Direction::Down) {break}
                    println!("test");
                    break;
                },
                Direction::Down => {},
                Direction::Left => {},
                Direction::Right => {}
            }
        }
        if self.snake_parts[0] == self.apple_location {
            self.apple_location = Board::new_apple();
            self.parts += 1;
            self.score += 1;
        }
    }
    fn print_field(&self) {
        println!("Current score: {}", self.score);
        for py in 0..BOARD_SIZE as u8 {
            let mut line = "".to_string();
            for px in 0..BOARD_SIZE as u8 {
                line += if in_arr((px, py), &self.snake_parts) {"O "}
                else if (px, py) == self.apple_location {"A "} else {". "}
            }
            println!("{}", line);
        }
    }
}

fn input() -> Direction {
    let direction: Direction;
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim() {
            "w" | "W" => {direction = Direction::Up; break}, 
            "a" | "A" => {direction = Direction::Left; break}, 
            "s" | "S" => {direction = Direction::Down; break}, 
            "d" | "D" => {direction = Direction::Right; break},
            _ => { println!("Only use: w, a, s and d"); continue }
        }
    }
    direction
}
fn main() {
    let mut game = Board::new();
    while !game.lost {
        game.print_field();
        game.update_snake(input());
        // thread::sleep(time::Duration::from_millis(100)); commented out until I'm no longer bad
    }
}
