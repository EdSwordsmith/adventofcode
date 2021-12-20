use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let file_path = env::args().nth(1).expect("Input file needed!");
    let input_file = File::open(file_path).unwrap();
    let reader = BufReader::new(input_file);

    let grid: Vec<Vec<u64>> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .collect()
        })
        .collect();

    let result = dijkstra(&grid);
    println!("Result: {}", result);
}

fn get_adjacent(pos: (usize, usize), width: usize, height: usize) -> Vec<(usize, usize)> {
    let mut v = vec![];
    if pos.0 > 0 {
        v.push((pos.0 - 1, pos.1));
    }

    if pos.1 > 0 {
        v.push((pos.0, pos.1 - 1));
    }

    if pos.0 < width - 1 {
        v.push((pos.0 + 1, pos.1));
    }

    if pos.1 < height - 1 {
        v.push((pos.0, pos.1 + 1));
    }

    v
}

fn dijkstra(grid: &Vec<Vec<u64>>) -> u64 {
    let height = grid.len();
    let width = grid[0].len();
    let goal = (height - 1, width - 1);

    let mut dist: Vec<u64> = Vec::new();
    let mut heap = BinaryHeap::new();

    for _ in 0..width * height {
        dist.push(u64::MAX);
    }
    dist[0] = 0;

    heap.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = heap.pop() {
        let idx = position.0 * width + position.1;
        if position == goal {
            return cost;
        }

        if cost > dist[idx] {
            continue;
        }

        for adj in get_adjacent(position, width, height).iter() {
            let adj_idx = adj.0 * width + adj.1;
            let next = State {
                cost: cost + grid[adj.0][adj.1],
                position: *adj,
            };

            if next.cost < dist[adj_idx] {
                heap.push(next);
                dist[adj_idx] = next.cost;
            }
        }
    }

    unreachable!("There should always be a way");
}
