use rand::{Rng, thread_rng};
use std::io;

const GAME_SIZE: usize = 12;

type Coord = (i8, i8);
type Snake = Vec<Coord>;

#[derive(PartialEq, Clone, Copy)]
enum Direction {Up, Down, Left, Right}
struct Game {
    snake: Snake,
    direction: Direction,
    apple_c: Coord,
    add_new: bool,
    lost: bool
}

impl Direction {
    fn opposite(&self) -> Self { // returns the opposite direction
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}

impl Game {
    fn new() -> Self { // returns new instance of the game
        Game {
            snake : vec![(7,6),(6,6),(5,6)],
            direction : Direction::Up,
            apple_c : Self::new_apple(),
            add_new : false,
            lost : false,
        }
    }
    fn update(&mut self, dir: Direction) { // Update game: score, movement etc
        if dir.opposite() != self.direction {self.direction = dir} // avoid opposite direction
        self.mv(); // Move
        if self.snake[0] == self.apple_c { // Apple eating
            self.add_new = true;
            self.apple_c = Game::new_apple()
        }
        if in_arr(&self.snake, self.snake[0], 1) // Loss detection
            || self.snake[0].1 < 0 // y
            || self.snake[0].1 == GAME_SIZE as i8
            || self.snake[0].0 < 0 // x
            || self.snake[0].0 == GAME_SIZE as i8
        {self.lost = true}
    }
    fn mv(&mut self) { // Move snake
        let mut tail_old = self.snake[0];
        let mut tail_old_2 = self.snake[0];
        let tail_last = self.snake[self.snake.len() - 1];
        match self.direction {
            Direction::Up => self.snake[0].1 -= 1,
            Direction::Down => self.snake[0].1 += 1,
            Direction::Left => self.snake[0].0 -= 1,
            Direction::Right => self.snake[0].0 += 1, 
        }

        for p in 1..self.snake.len() { // Move rest of the snake
            // spaghetti code time
            if p == 1 {tail_old_2 = self.snake[1]; self.snake[1] = tail_old; continue}
            if p % 2 == 0 {tail_old = self.snake[p]; self.snake[p] = tail_old_2; continue}
            tail_old_2 = self.snake[p];
            self.snake[p] = tail_old;
        }

        if self.add_new {self.snake.push(tail_last); self.add_new = false}
    }
    fn new_apple() -> Coord { // New apple
        (thread_rng().gen_range(0..GAME_SIZE as i8 - 1), thread_rng().gen_range(0..GAME_SIZE as i8 - 1))
    }
    fn print_field(&self) { // Print field
        println!();
        for y in 0..GAME_SIZE as i8 {
            let mut line = "".to_string();
            for x in 0..GAME_SIZE as i8 {
                line += if in_arr(&self.snake, (x, y), 0) {"O "}
                else if self.apple_c == (x, y) {"A "} else {". "}
            }
            println!("{}", line);
        }
    }
}

fn input() -> Direction { // Get user input
    loop {
        let mut u_in = String::new();
        io::stdin().read_line(&mut u_in).expect("Failed to read line");
        return match u_in.trim() {
            "w" | "W" => Direction::Up,
            "a" | "A" => Direction::Left,
            "s" | "S" => Direction::Down,
            "d" | "D" => Direction::Right,
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
    println!("This is a bad version of snake so use w, a, s, d and press enter to move");
    while !game.lost { // Game loop
        game.print_field();
        game.update(input());
    }
    println!("Game over!\nYou finished with a score of: {}", game.snake.len() - 3);
}
