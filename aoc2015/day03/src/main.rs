use std::env;
use std::fs;
use std::hash::Hash;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pos(i32, i32);

fn main() {
    // Get input filename
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let input = fs::read_to_string(input_file).unwrap();
    println!("Part 1: {}", visit_houses(&input));
    println!("Part 2: {}", visit_houses_with_robo_santa(&input));
}

fn visit_houses(input: &String) -> u32 {
    let mut positions = HashSet::new();
    let mut pos = Pos(0, 0);
    positions.insert(pos);

    for c in input.chars() {
        match c {
            '>' => { pos.0 = pos.0 + 1 },
            '<' => { pos.0 = pos.0 - 1 },
            '^' => { pos.1 = pos.1 + 1 },
            'v' => { pos.1 = pos.1 - 1 },
            _ => {}
        }

        positions.insert(pos);
    }

    positions.len() as u32
}

fn visit_houses_with_robo_santa(input: &String) -> u32 {
    let mut positions = HashSet::new();
    let mut santa = Pos(0, 0);
    let mut robo_santa = Pos(0, 0);
    let mut is_santa = true;
    positions.insert(santa);

    for c in input.chars() { 
        let mut pos = if is_santa { santa } else { robo_santa };

        match c {
            '>' => { pos.0 = pos.0 + 1 },
            '<' => { pos.0 = pos.0 - 1 },
            '^' => { pos.1 = pos.1 + 1 },
            'v' => { pos.1 = pos.1 - 1 },
            _ => {}
        }

        positions.insert(pos);
        if is_santa {
            santa = pos;
        } else {
            robo_santa = pos;
        }
        is_santa = !is_santa;
    }

    positions.len() as u32
}
