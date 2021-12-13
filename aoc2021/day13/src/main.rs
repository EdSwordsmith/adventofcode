use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use rltk::{GameState, Rltk};

struct State {
    dots: HashSet<(u32, u32)>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
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

            if first_part {
                first_part = false;
                println!("Part 1: {}", dots.len());
            }
        }
    }

    // Visual stuff go BRRRRR
    use rltk::RltkBuilder;

    let ctx = RltkBuilder::simple80x50()
        .with_title("Advent of Code 2021 - Day 13")
        .build()
        .expect("Please work");

    let gs = State { dots };

    rltk::main_loop(ctx, gs).unwrap();
}
