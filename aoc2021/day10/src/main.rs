use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let lines_data: Vec<(u32, u64)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            check_line(&line)
        })
        .collect();

    println!(
        "Part 1: {}",
        lines_data.iter().map(|(x, _)| *x).sum::<u32>()
    );
    let mut scores: Vec<u64> = lines_data.iter().map(|(_, x)| *x).filter(|x| *x != 0).collect();
    scores.sort();
    println!("Part 2: {}", scores[scores.len() / 2]);
}

fn check_line(line: &impl AsRef<str>) -> (u32, u64) {
    // (), [], {}, <>
    let mut stack = Vec::new();

    for c in line.as_ref().chars() {
        match c {
            '(' | '[' | '{' | '<' => {
                stack.push(c);
            }
            ')' | ']' | '}' | '>' => {
                if let Some(last) = stack.pop() {
                    if get_matching_closure(last) != c {
                        return (get_wrong_points(c), 0);
                    }
                }
            }
            _ => {}
        }
    }

    let score = stack
        .iter()
        .rev()
        .map(|c| get_matching_closure(*c))
        .fold(0, |acc, c| 5 * acc + get_complete_points(c));

    (0, score)
}

fn get_matching_closure(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("Invalid closure char"),
    }
}

fn get_wrong_points(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid closure char"),
    }
}

fn get_complete_points(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid closure char"),
    }
}
