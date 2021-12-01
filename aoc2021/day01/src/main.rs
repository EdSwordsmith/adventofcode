use std::env;
use std::fs;

fn main() {
    let input_file = env::args().nth(1).expect("You need to pass an input file!");
    let input = fs::read_to_string(input_file).unwrap();

    let depths: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    
    println!("Part 1: {}", count_increases(&depths, 1));
    println!("Part 2: {}", count_increases(&depths, 3));
}

fn count_increases(depths: &Vec<u32>, skip: usize) -> usize {
    depths
        .iter()
        .zip(depths.iter().skip(skip))
        .filter(|(c1, c2)| c1 < c2)
        .count()
}
