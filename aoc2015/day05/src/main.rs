use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let input = fs::read_to_string(input_file).unwrap();
    let nice_strings_count1 = input.lines().filter(|line| is_nice_string1(line)).count();
    let nice_strings_count2 = input.lines().filter(|line| is_nice_string2(line)).count();

    println!("Part 1: {}", nice_strings_count1);
    println!("Part 2: {}", nice_strings_count2);
}

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const FORBIDDEN: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn is_nice_string1(string: &str) -> bool {
    let has_vowels = string.chars().filter(|c| VOWELS.contains(c)).count() >= 3;

    let has_consecutive = string
        .chars()
        .zip(string.chars().skip(1))
        .filter(|(c1, c2)| c1 == c2)
        .count()
        > 0;

    let doesnt_have_forbidden = FORBIDDEN.iter().filter(|p| string.contains(*p)).count() == 0;

    has_vowels && has_consecutive && doesnt_have_forbidden
}

fn is_nice_string2(string: &str) -> bool {
    let pairs: Vec<String> = string
        .chars()
        .zip(string.chars().skip(1))
        .map(|(c1, c2)| {
            let mut s = String::from(c1);
            s.push(c2);
            s
        })
        .collect();

    let mut has_repeating_pair = false;
    for (i, pair) in pairs.iter().enumerate() {
        if pairs
            .iter()
            .skip(i + 2)
            .filter(|p| pair.eq(p.clone()))
            .count()
            > 0
        {
            has_repeating_pair = true;
            break;
        }
    }

    let has_repeating_letter = string
        .chars()
        .zip(string.chars().skip(2))
        .filter(|(c1, c2)| c1 == c2)
        .count()
        > 0;

    has_repeating_letter && has_repeating_pair
}
