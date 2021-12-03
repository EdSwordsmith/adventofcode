use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = env::args().nth(1).expect("Input file needed");
    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let numbers: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect();
    let start = numbers.get(0).unwrap();
    let mut end = numbers
        .iter()
        .fold(start.to_vec(), |acc, v| add_vectors(acc, v.to_vec()));

    let count = end.pop().unwrap();
    let end: Vec<u32> = end
        .iter()
        .map(|n| if *n > count / 2 { 1 } else { 0 })
        .collect();

    let (gamma, epsilon): (u32, u32) = end.iter().fold((0, 0), |(gamma, epsilon), n| {
        (gamma * 2 + n, epsilon * 2 + (1 - n))
    });

    let numbers: Vec<Vec<u32>> = numbers
        .iter()
        .map(|v| {
            let mut v = v.clone();
            v.pop();
            v
        })
        .collect();
    let mut oxygen = numbers.clone();
    let mut co2 = numbers.clone();

    let mut i = 0;
    while oxygen.len() > 1 {
        let ones: usize = oxygen.iter().filter(|v| *v.get(i).unwrap() == 1).count();
        let zeros: usize = oxygen.iter().filter(|v| *v.get(i).unwrap() == 0).count();
        let n: u32 = if ones >= zeros { 1 } else { 0 };
        oxygen.retain(|v| *v.get(i).unwrap() == n);
        i += 1;
    }

    let mut i = 0;
    while co2.len() > 1 {
        let ones: usize = co2.iter().filter(|v| *v.get(i).unwrap() == 1).count();
        let zeros: usize = co2.iter().filter(|v| *v.get(i).unwrap() == 0).count();
        let n: u32 = if ones >= zeros { 1 } else { 0 };
        co2.retain(|v| *v.get(i).unwrap() != n);
        i += 1;
    }

    let oxygen = oxygen[0].iter().fold(0, |acc, n| 2 * acc + n);
    let co2 = co2[0].iter().fold(0, |acc, n| 2 * acc + n);
    println!("Part 1: {}", gamma * epsilon);
    println!("Part 2: {}", oxygen * co2);
}

fn add_vectors(v1: Vec<u32>, v2: Vec<u32>) -> Vec<u32> {
    v1.iter().zip(v2.iter()).map(|(n1, n2)| n1 + n2).collect()
}

fn parse_line(line: impl AsRef<str>) -> Vec<u32> {
    let mut v: Vec<u32> = line
        .as_ref()
        .chars()
        .map(|c| c.to_digit(2).unwrap())
        .collect();
    v.push(1);
    v
}
