use std::{io, thread, time};
use rand::{Rng, thread_rng};

const BOARD_SIZE: usize = 12;

fn in_arr(nums: (i8, i8), snake_parts: &Vec<(i8, i8)>, s_p: usize) -> bool {
    for i in s_p..snake_parts.len() { if snake_parts[i] == nums {return true} }
    false
}
#[derive(Copy, Clone)]
enum Direction {Up, Down, Left, Right}

impl Direction {
    fn opposite(&self, dir: &Direction) -> bool {
        match dir {
            Direction::Up => return matches!(dir, Direction::Down),
            Direction::Down => return matches!(dir, Direction::Up ),
            Direction::Left => return matches!(dir, Direction::Right),
            Direction::Right => return matches!(dir, Direction::Left)
        } 
    }
}

struct Board {
    //field: [[char;BOARD_SIZE];BOARD_SIZE],
    snake_parts: Vec<(i8, i8)>,
    parts: i8,
    apple_location: (i8, i8),
    lost: bool,
    score: i8,
    direction: Direction
}

impl Board {
    fn new() -> Board {
        Board {
            //field : [['.';BOARD_SIZE];BOARD_SIZE], turns out this is not needed
            snake_parts : vec![(7,6),(6,6),(5,6)],
            parts: 3,
            apple_location : Board::new_apple(),
            lost : false,
            score: 0,
            direction : Direction::Up
        }
    }
    fn new_apple() -> (i8, i8) {(thread_rng().gen_range(0..8), thread_rng().gen_range(0..8))}

    fn update_snake(&mut self, dir: Direction) {
        self.move_snake(dir);
        if self.snake_parts[0] == self.apple_location {
            self.apple_location = Board::new_apple();
            self.snake_parts.push(self.snake_parts[self.snake_parts.len() - 1]); // worst line of code in this program. coming back to this later I was wrong
            self.parts += 1;
            self.score += 1;
        }
    }
    fn print_field(&self) {
        println!("Current score: {}", self.score);
        for py in 0..BOARD_SIZE as i8 {
            let mut line = "".to_string();
            for px in 0..BOARD_SIZE as i8 {
                line += if in_arr((px, py), &self.snake_parts, 0) {"O "}
                else if (px, py) == self.apple_location {"A "} else {". "}
            }
            println!("{}", line);
        }
    }

    fn move_snake(&mut self, dir: Direction) { // (x, y)
        if !dir.opposite(&self.direction) {self.direction = dir}
        match dir { // TODO: move rest of the body and temporary stuff
            Direction::Up => {
                for coord in self.snake_parts.clone() {
                    for i in 1..self.snake_parts.len() {
                        self.snake_parts[i] = self.snake_parts[i - 1];
                    }
                }
                self.snake_parts[0].1 -= 1;
            }
            Direction::Down => {
                for coord in self.snake_parts.clone() {
                    for i in 1..self.snake_parts.len() {
                     self.snake_parts[i] = self.snake_parts[i - 1];
                    }
                }
                self.snake_parts[0].1 += 1;
            }
            Direction::Left => {
                for coord in self.snake_parts.clone() {
                    for i in 1..self.snake_parts.len() {
                     self.snake_parts[i] = self.snake_parts[i - 1];
                    }
                }
                self.snake_parts[0].0 -= 1;
            }
            Direction::Right => {
                for coord in self.snake_parts.clone() {
                    for i in 1..self.snake_parts.len() {
                     self.snake_parts[i] = self.snake_parts[i - 1];
                    }
                }
                self.snake_parts[0].0 += 1;
                println!("{:?}", self.snake_parts);
            }
        }

        if in_arr(self.snake_parts[0], &self.snake_parts, 1)  // loss detection: works
            || self.snake_parts[0].1 < 0
            || self.snake_parts[0].1 >= 12
            || self.snake_parts[0].0 < 0
            || self.snake_parts[0].0 >= 12
        {self.lost = true}
    }
}

fn input() -> Direction { // temporary solution TODO: non blocking input
    let direction: Direction;
    loop {
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim() {
            "w" | "W" => {direction = Direction::Up; break}, 
            "a" | "A" => {direction = Direction::Left; break}, 
            "s" | "S" => {direction = Direction::Down; break}, 
            "d" | "D" => {direction = Direction::Right; break},
            _ => {println!("Only use: w, a, s and d"); continue}
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
    println!("Game over.\nYou finished with a score of: {}", game.score);
}
