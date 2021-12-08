use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let mut reader = BufReader::new(input_file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut positions: Vec<i32> = line.split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    positions.sort();
    let mut pos = positions[positions.len() / 2];
    let cost: i32 = positions.iter().map(|crab| (pos - *crab).abs()).sum();

    println!("Part 1: {:?}", cost);

    let mut cost: i32 = positions
        .iter()
        .map(|crab| (pos- *crab).abs())
        .map(|n| n * (n + 1) / 2)
        .sum();

    loop {
        let bi_cost: i32 = positions
            .iter()
            .map(|crab| (pos - 1 - *crab).abs())
            .map(|n| n * (n + 1) / 2)
            .sum();
        let ai_cost: i32 = positions
            .iter()
            .map(|crab| (pos + 1 - *crab).abs())
            .map(|n| n * (n + 1) / 2)
            .sum();

        if cost < bi_cost && cost < ai_cost {
            break;
        } else if bi_cost < ai_cost {
            pos -= 1;
            cost = bi_cost;
        } else {
            pos += 1;
            cost = ai_cost;
        }
    }

    println!("Part 2: {:?}", cost);
}
