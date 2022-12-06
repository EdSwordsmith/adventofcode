use std::{io, collections::HashSet};

fn check_for_marker(chars: &[char]) -> bool {
    let set: HashSet<char> = chars.iter().map(|c| *c).collect();
    set.len() == chars.len()
}

fn solve(iter: &[char], window_size: usize) -> usize {
    for (i, chars) in iter.windows(window_size).enumerate() {
        if check_for_marker(chars) {
            return i + window_size;
        }
    }

    unreachable!("Trust me, bro")
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line from stdin");

    let iter: Vec<char> = input
        .chars()
        .collect();
    
    println!("Part 1: {}", solve(&iter, 4));
    println!("Part 2: {}", solve(&iter, 14));
}
