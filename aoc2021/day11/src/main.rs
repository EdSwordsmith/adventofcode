use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, hash};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);

    let mut grid: [[u32; 10]; 10] = [[0; 10]; 10];
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (j, c) in line.chars().enumerate() {
            grid[i][j] = c.to_digit(10).unwrap();
        }
    }

    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut grid);
    }

    println!("Part 1: {}", flashes);

    let mut i = 100;
    loop {
        i += 1;
        let dflashes = step(&mut grid);

        if dflashes == 100 {
            break;
        }
    }

    println!("Part 2: {}", i);
}

fn step(grid: &mut [[u32; 10]; 10]) -> u32 {
    let mut has_flashed = [[false; 10]; 10];

    // Increase energy levels
    for i in 0..10 {
        for j in 0..10 {
            grid[i][j] += 1;
        }
    }

    loop {
        let mut flashed = false;

        for i in 0..10 {
            for j in 0..10 {
                if grid[i][j] > 9 && !has_flashed[i][j] {
                    flashed = true;
                    handle_flash(grid, i, j);
                    has_flashed[i][j] = true;
                }
            }
        }

        if !flashed {
            break;
        }
    }

    

    // Reset flashes to 0
    for i in 0..10 {
        for j in 0..10 {
            if grid[i][j] > 9 {
                grid[i][j] = 0;
            }
        }
    }

    has_flashed
        .iter()
        .map(|row| row.iter().filter(|flashed| **flashed).count() as u32)
        .sum()
}

fn handle_flash(grid: &mut [[u32; 10]; 10], i: usize, j: usize) {
    if i > 0 {
        grid[i - 1][j] += 1;
    }

    if i < 9 {
        grid[i + 1][j] += 1;
    }

    if j > 0 {
        grid[i][j - 1] += 1;
    }

    if j < 9 {
        grid[i][j + 1] += 1;
    }

    if i > 0 && j > 0 {
        grid[i - 1][j - 1] += 1;
    }

    if i > 0 && j < 9 {
        grid[i - 1][j + 1] += 1;
    }

    if i < 9 && j > 0 {
        grid[i + 1][j - 1] += 1;
    }

    if i < 9 && j < 9 {
        grid[i + 1][j + 1] += 1;
    }
}
