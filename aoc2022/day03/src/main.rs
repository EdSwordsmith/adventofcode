use std::io;

fn common_item(rucksack: &String) -> char {
    let half = rucksack.len() / 2;
    let first = &rucksack[..half];
    let second = &rucksack[half..];

    first
        .chars()
        .filter(|c| second.contains(*c))
        .next()
        .unwrap()
}

fn item_priority(item: char) -> u8 {
    let mut item_bytes = [0; 2];
    item.encode_utf8(&mut item_bytes);

    if item.is_ascii_lowercase() {
        item_bytes[0] - 96
    } else {
        item_bytes[0] - 38
    }
}

fn common_item_group(one: &String, two: &String, three: &String) -> char {
    one
        .chars()
        .filter(|c| two.contains(*c))
        .filter(|c| three.contains(*c))
        .next()
        .unwrap()
}

fn main() {
    let rucksacks: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .collect();

    let first_part: u32 = rucksacks
        .iter()
        .map(|rucksack| common_item(rucksack))
        .map(|item| item_priority(item) as u32)
        .sum();

    println!("Part 1: {}", first_part);

    let rucksacks_after_first = &rucksacks[1..];
    let rucksacks_after_second = &rucksacks[2..];
    let second_part: u32 = rucksacks
        .iter()
        .zip(rucksacks_after_first)
        .zip(rucksacks_after_second)
        .enumerate()
        .filter(|(i, _)| *i % 3 == 0)
        .map(|(_, ((one, two), three))| common_item_group(one, two, three))
        .map(|item| item_priority(item) as u32)
        .sum();

    println!("Part 2: {}", second_part);
}
