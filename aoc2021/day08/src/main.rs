use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);

    let (part1, part2): (u32, u32) = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (digits, output) = line.split_once(" | ").unwrap();

            // Part 1
            let easy_digits = output
                .split(' ')
                .map(|digit| digit.trim().len())
                .filter(|segments| {
                    *segments == 2 || *segments == 4 || *segments == 3 || *segments == 7
                })
                .count();

            // Part 2
            let digits = parse_into_bits(&digits);
            let translation = match_signals_to_digits(digits);
            let output = parse_into_bits(&output);
            let value = output
                .iter()
                .map(|digit| translation.get(digit).unwrap())
                .fold(0, |acc, digit| acc * 10 + digit);

            (easy_digits as u32, value)
        })
        .fold((0, 0), |(a1, a2), (e1, e2)| (a1 + e1, a2 + e2));

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_into_bits(signals: &impl AsRef<str>) -> Vec<u32> {
    signals
        .as_ref()
        .split(' ')
        .map(|digit| {
            digit.chars().fold(0, |acc, c| match c {
                'a' => acc | 1,
                'b' => acc | 2,
                'c' => acc | 4,
                'd' => acc | 8,
                'e' => acc | 16,
                'f' => acc | 32,
                'g' => acc | 64,
                _ => unreachable!("error in parsing the input"),
            })
        })
        .collect()
}

fn match_signals_to_digits(signals: Vec<u32>) -> HashMap<u32, u32> {
    let mut map: [u32; 10] = [0; 10];

    for signal in signals.iter() {
        let segments = signal.count_ones();
        match segments {
            2 => { map[1] = *signal },
            4 => { map[4] = *signal },
            3 => { map[7] = *signal },
            7 => { map[8] = *signal },
            _ => { },
        };
    }

    for signal in signals.iter() {
        let segments = signal.count_ones();
        match segments {
            5 => {
                // 2, 3, 5
                if *signal & map[1] == map[1] {
                    map[3] = *signal;
                } else if (*signal & map[4]).count_ones() == 2 {
                    map[2] = *signal;
                } else {
                    map[5] = *signal;
                }
            }
            6 => {
                // 0, 6, 9
                if *signal & map[1] != map[1] {
                    map[6] = *signal;
                } else if *signal & map[4] == map[4] {
                    map[9] = *signal;
                } else {
                    map[0] = *signal;
                }
            }
            _ => {}
        };
    }

    let mut res = HashMap::new();
    for (i, v) in map.iter().enumerate() {
        res.insert(*v, i as u32);
    }
    res
}
