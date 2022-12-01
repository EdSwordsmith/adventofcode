use std::io;
use std::io::BufRead;

fn main() {
    let calories = io::stdin()
        .lock()
        .lines()
        .map(|s| s.unwrap().parse::<u32>().unwrap_or(0))
        .fold(Vec::new(), |mut acc, value| {
            let mut current_size = acc.len();
            
            if value == 0 || current_size == 0 {
                acc.push(0);
                current_size += 1
            }

            if value != 0 {
                acc[current_size - 1] += value;
            }

            acc
        });
    
    if let Some(maximum_calories) = calories.iter().max() {
        println!("Part 1: {}", maximum_calories);
    }

    let top_three: u32 = calories
        .iter()
        .fold([0; 3], |acc, value| {
            let value = *value;
            if value > acc[0] {
                [value, acc[0], acc[1]]
            } else if value > acc[1] {
                [acc[0], value, acc[1]]
            } else if value > acc[2] {
                [acc[0], acc[1], value]
            } else {
                [acc[0], acc[1], acc[2]]
            }
        })
        .iter()
        .sum();

    println!("Part 2: {}", top_three);
}
