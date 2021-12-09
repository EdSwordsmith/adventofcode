use std::collections::{VecDeque, HashSet};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);

    let heightmap: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut sum = 0;
    let mut basin_sizes: (u32, u32, u32) = (0, 0, 0);
    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            if check_low_point(&heightmap, x, y) {
                let risk = heightmap[y][x] + 1;
                sum += risk;

                let basin_size: u32 = check_basin_size(&heightmap, x, y);
                if basin_size > basin_sizes.2 {
                    basin_sizes.0 = basin_sizes.1;
                    basin_sizes.1 = basin_sizes.2;
                    basin_sizes.2 = basin_size;
                } else if basin_size > basin_sizes.1 {
                    basin_sizes.0 = basin_sizes.1;
                    basin_sizes.1 = basin_size;
                } else if basin_size > basin_sizes.0 {
                    basin_sizes.0 = basin_size;
                }
            }
        }
    }

    println!("Part 1: {}", sum);
    println!("Part 2: {}", basin_sizes.0 * basin_sizes.1 * basin_sizes.2);
}

fn check_low_point(heightmap: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    let up = y != 0 && heightmap[y - 1][x] <= heightmap[y][x];
    let down = y != heightmap.len() - 1 && heightmap[y + 1][x] <= heightmap[y][x];
    let left = x != 0 && heightmap[y][x - 1] <= heightmap[y][x];
    let right = x != heightmap[y].len() - 1 && heightmap[y][x + 1] <= heightmap[y][x];

    !up && !down && !left && !right
}

fn check_basin_size(heightmap: &Vec<Vec<u32>>, x: usize, y: usize) -> u32 {
    let mut queue = VecDeque::new();
    let mut checked = HashSet::new();
    let mut basin_size: u32 = 0;

    queue.push_back((x, y));

    while queue.len() > 0 {
        let (x, y) = queue.pop_front().unwrap();

        if y != 0 && heightmap[y - 1][x] != 9 && heightmap[y][x] <= heightmap[y - 1][x] {
            let up = (x, y - 1);
            if !checked.contains(&up) && !queue.contains(&up) {
                queue.push_back(up);
            }
        }

        if y != heightmap.len() - 1 && heightmap[y + 1][x] != 9 && heightmap[y][x] <= heightmap[y + 1][x] {
            let down = (x, y + 1);
            if !checked.contains(&down) && !queue.contains(&down) {
                queue.push_back(down);
            }
        }
        if x != 0 && heightmap[y][x - 1] != 9 && heightmap[y][x] <= heightmap[y][x - 1] {
            let left = (x - 1, y);
            if !checked.contains(&left) && !queue.contains(&left) {
                queue.push_back(left);
            }
        }
        if x != heightmap[y].len() - 1 && heightmap[y][x + 1] != 9 && heightmap[y][x] <= heightmap[y][x + 1] {
            let right = (x + 1, y);
            if !checked.contains(&right) && !queue.contains(&right) {
                queue.push_back(right);
            }
        }

        checked.insert((x, y));
        basin_size += 1;
    }

    basin_size
}
