use rand::{Rng, thread_rng};
use std::io;

const GAME_SIZE: usize = 12;

type Coord = (i8, i8);
type Snake = Vec<Coord>;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {Up, Down, Left, Right}
#[derive(Debug)]
struct Game {
    snake: Snake,
    direction: Direction,
    apple_c: Coord,
    score: u8,
    lost: bool
}
impl Direction {
    fn opposite(&self) -> Direction { // return true if its the opposite direction
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn rand_dir() -> Direction { // Random direction for the game start
        let dirs = vec![Self::Up,Self::Down,Self::Left,Self::Right];
        dirs[thread_rng().gen_range(0..3)]
    }
}

impl Game {
    fn new() -> Self { // New instance of the game
        Game {
            snake : vec![(7,6),(6,6),(5,6)],
            direction : Direction::rand_dir(),
            apple_c : Self::new_apple(),
            score : 0,
            lost : false
        }
    }

    fn update(&mut self, dir: Direction) { // Update game: score, movement etc
        if dir.opposite() != self.direction {self.direction = dir} // avoid opposite direction
        self.mv(self.direction); // Move

        if self.snake[0] == self.apple_c {
            self.score += 1;
            self.apple_c = Game::new_apple()
        }

        if in_arr(&self.snake, self.snake[0], 1) // Loss detection
            || self.snake[0].1 < 0 // y
            || self.snake[0].1 == GAME_SIZE as i8
            || self.snake[0].0 < 0 // x
            || self.snake[0].0 == GAME_SIZE as i8
        {self.lost = true}
        
        println!("{:?}", self); // Debug
    }

    fn mv(&mut self, dir: Direction) { // Move snake // FIXME Only functions with a lenght of 3
        let mut old_tail: Coord = self.snake[0];
        match dir {
            Direction::Up => self.snake[0].1 -= 1,
            Direction::Down => self.snake[0].1 += 1,
            Direction::Left => self.snake[0].0 -= 1,
            Direction::Right => self.snake[0].0 += 1, 
        }
        for p in 0..self.snake.len() {
            if p == 0 {self.snake[1] = old_tail; old_tail = self.snake[1]; continue}
            if self.snake[p] == old_tail {self.snake[p] = self.snake[p+1]; continue}
            self.snake[p] = old_tail;
            old_tail = self.snake[p];
        }
    }

    fn new_apple() -> Coord { // New apple
        (thread_rng().gen_range(0..GAME_SIZE as i8 - 1), thread_rng().gen_range(0..GAME_SIZE as i8 - 1))
    }

    fn print_field(&self) { // Print a new field
        println!();
        for py in 0..GAME_SIZE as i8 {
            let mut line = "".to_string(); 
            for px in 0..GAME_SIZE as i8 {
                line += if in_arr(&self.snake, (px, py), 0) {"O "}
                else if self.apple_c == (px, py) {"A "} else {". "}
            }
            println!("{}", line);
        }
    }
}

fn input() -> Direction { // Get user input
    // Direction::Up
    loop {
        let mut u_in = String::new();
        io::stdin().read_line(&mut u_in).expect("Failed to read line");
        match u_in.trim() {
            "w" | "W" => return Direction::Up,
            "a" | "A" => return Direction::Left,
            "s" | "S" => return Direction::Down,
            "d" | "D" => return Direction::Right,
            _ => {println!("Only use: w, a, s and d."); continue}
        }
    }
}

fn in_arr(arr: &Snake, val: Coord, sp: usize) -> bool { // Check if a variable is in array
    for i in sp..arr.len() {if arr[i] == val {return true}}
    false
}

fn main() { // entry point
    let mut game = Game::new();

    while !game.lost {
        game.print_field();
        game.update(input());
    }
    println!("Game over!\nYou finished with a score of: {}", game.score);
}
