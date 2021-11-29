use std::env;
use std::fs;

struct Pos {
    x: u32,
    y: u32,
}

impl From<&str> for Pos {
    fn from(string: &str) -> Self {
        let mut nums = string.split(',').map(|s| s.parse::<u32>().unwrap());
        let x = nums.next().unwrap();
        let y = nums.next().unwrap();
        Pos { x, y }
    }
}

enum Instruction {
    TurnOn(Pos, Pos),
    TurnOff(Pos, Pos),
    Toggle(Pos, Pos),
}

impl From<&str> for Instruction {
    fn from(string: &str) -> Self {
        let words: Vec<&str> = string.split(' ').collect();
        match words[0] {
            "toggle" => {
                let start = Pos::from(words[1]);
                let end = Pos::from(words[3]);
                Instruction::Toggle(start, end)
            }
            "turn" => {
                let start = Pos::from(words[2]);
                let end = Pos::from(words[4]);

                match words[1] {
                    "on" => Instruction::TurnOn(start, end),
                    "off" => Instruction::TurnOff(start, end),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }
}

struct Grid<T: Clone> {
    grid: Vec<T>,
}

impl<T: Clone> Grid<T> {
    fn new(v: T) -> Self {
        Grid {
            grid: vec![v; 1000 * 1000],
        }
    }

    fn get(&self, x: u32, y: u32) -> &T {
        let x = x as usize;
        let y = y as usize;
        return &self.grid[x + y * 1000];
    }

    fn set(&mut self, x: u32, y: u32, value: T) {
        let x = x as usize;
        let y = y as usize;
        self.grid[x + y * 1000] = value;
    }
}

impl Grid<bool> {
    fn turn_on(&mut self, x: u32, y: u32) {
        self.set(x, y, true);
    }

    fn turn_off(&mut self, x: u32, y: u32) {
        self.set(x, y, false);
    }

    fn toggle(&mut self, x: u32, y: u32) {
        self.set(x, y, !self.get(x, y));
    }
}

impl Grid<u32> {
    fn turn_on(&mut self, x: u32, y: u32) {
        self.set(x, y, *self.get(x, y) + 1);
    }

    fn turn_off(&mut self, x: u32, y: u32) {
        let value = *self.get(x, y);
        if value > 0 {
            self.set(x, y, value - 1);
        }
    }

    fn toggle(&mut self, x: u32, y: u32) {
        self.set(x, y, *self.get(x, y) + 2);
    }
}

fn main() {
    let input_file = env::args().skip(1).next().unwrap();
    let input = fs::read_to_string(input_file).unwrap();
    let mut bool_grid = Grid::new(false);
    let mut num_grid: Grid<u32> = Grid::new(0);

    let instructions: Vec<Instruction> =
        input.lines().map(|line| Instruction::from(line)).collect();
    for instruction in instructions.iter() {
        match instruction {
            Instruction::TurnOn(start, end) => {
                for x in start.x..end.x + 1 {
                    for y in start.y..end.y + 1 {
                        bool_grid.turn_on(x, y);
                        num_grid.turn_on(x, y);
                    }
                }
            }
            Instruction::TurnOff(start, end) => {
                for x in start.x..end.x + 1 {
                    for y in start.y..end.y + 1 {
                        bool_grid.turn_off(x, y);
                        num_grid.turn_off(x, y);
                    }
                }
            }
            Instruction::Toggle(start, end) => {
                for x in start.x..end.x + 1 {
                    for y in start.y..end.y + 1 {
                        bool_grid.toggle(x, y);
                        num_grid.toggle(x, y);
                    }
                }
            }
        }
    }

    let part1 = bool_grid.grid.iter().filter(|v| **v).count();
    let part2: u32 = num_grid.grid.iter().sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
