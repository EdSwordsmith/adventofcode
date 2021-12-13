use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rltk::{GameState, Rltk};

struct State {
    dots: HashSet<(u32, u32)>,
    fold_commands: VecDeque<(char, u32)>,
    part1: usize,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if let Some(fold) = self.fold_commands.pop_front() {
            fold_dots(&mut self.dots, fold.0, fold.1);
            match fold.0 {
                'x' => {
                    for y in 0..ctx.height_pixels {
                        ctx.print(fold.1, y, String::from("|"));
                    }
                }
                'y' => {
                    for x in 0..ctx.width_pixels {
                        ctx.print(x, fold.1, String::from("-"));
                    }
                }
                _ => {
                    unreachable!("please work too");
                }
            }
        } else {
            let p1 = format!("Part 1: {}", self.part1);
            ctx.print(10, 40, p1);
        }

        // Draw
        for (x, y) in self.dots.iter() {
            ctx.print(*x, *y, "#");
        }
    }
}

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let mut parsing_dots = true;
    let mut first_part = true;

    let mut dots = HashSet::new();
    let mut fold_commands = VecDeque::new();
    let mut part1 = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if line.trim().is_empty() {
            parsing_dots = false;
            continue;
        }

        if parsing_dots {
            let (x, y) = line.split_once(',').unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();
            dots.insert((x, y));
        } else {
            let (fold, value) = line.split_once('=').unwrap();
            let value: u32 = value.parse().unwrap();
            let var = fold.chars().last().unwrap();
            if !first_part {
                fold_commands.push_back((var, value));
            }

            if first_part {
                first_part = false;
                fold_dots(&mut dots, var, value);
                println!("Part 1: {}", dots.len());
                part1 = dots.len();
            }
        }
    }

    // Visual stuff go BRRRRR
    use rltk::RltkBuilder;

    let ctx = RltkBuilder::simple80x50()
        .with_title("Advent of Code 2021 - Day 13")
        .with_fps_cap(4.0)
        .build()
        .expect("Please work");

    let gs = State {
        dots,
        fold_commands,
        part1,
    };

    rltk::main_loop(ctx, gs).unwrap();
}

fn fold_dots(dots: &mut HashSet<(u32, u32)>, var: char, value: u32) {
    match var {
        'x' => {
            let mut folded = Vec::new();
            for (x, y) in dots.iter() {
                if *x > value {
                    folded.push((*x, *y));
                }
            }

            for dot in folded.iter() {
                dots.remove(dot);
                let mut new = *dot;
                new.0 = 2 * value - new.0;
                dots.insert(new);
            }
        }
        'y' => {
            let mut folded = Vec::new();
            for (x, y) in dots.iter() {
                if *y > value {
                    folded.push((*x, *y));
                }
            }

            for dot in folded.iter() {
                dots.remove(dot);
                let mut new = *dot;
                new.1 = 2 * value - new.1;
                dots.insert(new);
            }
        }
        _ => unreachable!("you shouldn't be seeing this"),
    }
}
