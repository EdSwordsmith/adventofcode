use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let mut parsing_rules = false;

    let mut pairs = HashMap::new();
    let mut letters = HashMap::new();
    let mut rules = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if !parsing_rules {
            for pair in line.chars().zip(line.chars().skip(1)) {
                let zero: u64 = 0;
                let count = pairs.get(&pair).unwrap_or(&zero) + 1;
                pairs.insert(pair, count);
            }

            for letter in line.chars() {
                let zero: u64 = 0;
                let count = letters.get(&letter).unwrap_or(&zero) + 1;
                letters.insert(letter, count);
            }

            parsing_rules = true;
        } else if let Some((pair, c)) = line.split_once(" -> ") {
            let c = c.chars().next().unwrap();
            let pair = {
                let mut chars = pair.chars();
                let first = chars.next().unwrap();
                let second = chars.next().unwrap();
                (first, second)
            };
            rules.insert(pair, c);
        }
    }

    for _ in 0..10 {
        tick(&mut letters, &mut pairs, &rules);
    }

    println!("Part 1: {}", letters.values().max().unwrap() - letters.values().min().unwrap());

    for _ in 10..40 {
        tick(&mut letters, &mut pairs, &rules);
    }

    println!("Part 2: {}", letters.values().max().unwrap() - letters.values().min().unwrap());
}

fn tick(
    letters: &mut HashMap<char, u64>,
    pairs: &mut HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) {
    let mut update = HashMap::new();

    for (pair, count) in pairs.iter_mut() {
        if let Some(c) = rules.get(pair) {
            let mut pair1 = pair.clone();
            pair1.1 = c.clone();
            let mut pair2 = pair.clone();
            pair2.0 = c.clone();

            let zero: u64 = 0;
            let ncount = update.get(&pair1).unwrap_or(&zero) + *count;
            update.insert(pair1, ncount);
            let ncount = update.get(&pair2).unwrap_or(&zero) + *count;
            update.insert(pair2, ncount);
            let ncount = letters.get(c).unwrap_or(&zero) + *count;
            letters.insert(c.clone(), ncount);

            *count = 0;
        }
    }

    for (pair, count) in update.iter() {
        let zero: u64 = 0;
        let new_count = pairs.get(pair).unwrap_or(&zero) + count;
        pairs.insert(*pair, new_count);
    }
}
