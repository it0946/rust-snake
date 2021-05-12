use rand::{Rng, thread_rng};
use std::io;

const GAME_SIZE: usize  = 12;

type Coord = (i8, i8);
type Snake = Vec<Coord>;

#[derive(PartialEq)]
enum Dir {Up, Down, Left, Right}

struct Game {
    lost: bool,
    snake: Snake,
    apple: Coord,
    dir: Dir
}

impl Dir {
    fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left
        }
    }
}

impl Game {
    fn update(&mut self, dir: Dir) {
        if dir.opposite() != self.dir {self.dir = dir}
        self.mv();        

        if self.snake[0] == self.apple {
            self.snake.push(self.snake[self.snake.len() - 1]); 
            self.apple = self.new_apple()
        }

        if in_arr(&self.snake, self.snake[0], 1)
            || self.snake[0].1 < 0 || self.snake[0].1 == GAME_SIZE as i8 // y
            || self.snake[0].0 < 0 || self.snake[0].0 == GAME_SIZE as i8 // x
        {self.lost = true}
    }
    
    fn mv(&mut self) {
        let (mut o, mut o2) = (self.snake[0], self.snake[0]);

        match self.dir {
            Dir::Up => self.snake[0].1 -= 1,
            Dir::Down => self.snake[0].1 += 1,
            Dir::Left => self.snake[0].0 -= 1,
            Dir::Right => self.snake[0].0 += 1, 
        }

        for p in 1..self.snake.len() { // Move rest of the snake
            if p == 1 {o2 = self.snake[1]; self.snake[1] = o; continue}
            if p % 2 == 0 {o = self.snake[p]; self.snake[p] = o2; continue}
            o2 = self.snake[p];
            self.snake[p] = o;
        }
    }

    fn print(&self) {
        println!();
        for y in 0..GAME_SIZE as i8 {
            let mut line = "".to_string();
            for x in 0..GAME_SIZE as i8 {
                line += if in_arr(&self.snake, (x, y), 0) {"O "}
                else if self.apple == (x, y) {"A "} else {". "}
            }
            println!("{}", line);
        }
    }

    fn new_apple(&self) -> Coord {
        loop { // lazy but works
            let (x, y) = (thread_rng().gen_range(0..GAME_SIZE as i8 - 1), thread_rng().gen_range(0..GAME_SIZE as i8 - 1));
            if !in_arr(&self.snake, (x, y), 0) {return (x, y)}
        }
    }
}

fn input() -> Dir {
    loop {
        let mut u_in = String::new();
        io::stdin().read_line(&mut u_in).expect("Failed to read line");
        return match u_in.trim() {
            "w" | "W" => Dir::Up,
            "a" | "A" => Dir::Left,
            "s" | "S" => Dir::Down,
            "d" | "D" => Dir::Right,
            _ => {println!("Only use: w, a, s and d."); continue}
        }
    }
}

fn in_arr(arr: &Snake, val: Coord, sp: usize) -> bool {
    for i in sp..arr.len() { if arr[i] == val {return true}}
    false
}

fn main() {
    println!("Use w, a, s, d and then enter to move the snake.");

    let mut game = Game {
        lost: false,
        snake: vec![(6,6);3],
        apple: ( 
            // Technically an apple can spawn inside the snake (6, 6), but I wont bother for now (don't yell at me cag)
            thread_rng().gen_range(0..GAME_SIZE as i8 - 1),
            thread_rng().gen_range(0..GAME_SIZE as i8 - 1)
        ),
        dir: Dir::Up
    };

    while !game.lost {
        game.print();
        game.update(input());
    }

    println!("Game over.\nYou finished with a score of: {}", game.snake.len() - 3);
}
