use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

/* Thanks to Diogo Correia, for giving me an hint when I was stuck in part 2 */
fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let mut reader = BufReader::new(input_file);
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let mut fishes: [u64; 9] = input
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .fold([0; 9], |mut fishes, fish| {
            fishes[fish as usize] += 1;
            fishes
        });

    println!("Part 1: {}", run_cycles(&mut fishes, 0..80));
    println!("Part 2: {}", run_cycles(&mut fishes, 80..256));
}

fn run_cycles(fishes: &mut [u64; 9], range: Range<i32>) -> u64 {
    for _ in range {
        let new_fishes = fishes[0];

        for i in 0..8 {
            fishes[i] = fishes[i + 1];
        }

        fishes[6] += new_fishes;
        fishes[8] = new_fishes;
    }

    fishes.iter().sum()
}
