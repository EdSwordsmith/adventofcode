use std::{collections::HashSet, io};

fn main() {
    let mut points: HashSet<(u32, u32)> = io::stdin()
        .lines()
        .flat_map(|line| parse_line(&line.unwrap()))
        .collect();
    let initial_state = points.clone();
    let max_y = points.iter().map(|point| point.1).max().unwrap();
    let mut count = 0;

    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if !points.contains(&(x, y + 1)) {
                y += 1;
            } else if !points.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !points.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }

            if y >= max_y {
                break;
            }
        }

        if y >= max_y {
            break;
        }
        points.insert((x, y));
        count += 1;
    }

    println!("Part 1: {}", count);

    count = 0;
    points = initial_state;
    let floor = max_y + 2;

    loop {
        let mut x = 500;
        let mut y = 0;

        loop {
            if y + 1 == floor {
                break;
            }

            if !points.contains(&(x, y + 1)) {
                y += 1;
            } else if !points.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !points.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                break;
            }
        }

        points.insert((x, y));
        count += 1;
        if x == 500 && y == 0 {
            break;
        }
    }

    println!("Part 2: {}", count);
}

fn parse_line(line: &impl AsRef<str>) -> HashSet<(u32, u32)> {
    let vertices: Vec<(u32, u32)> = line
        .as_ref()
        .split(" -> ")
        .map(|coords| coords.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut points = HashSet::new();

    for (prev, curr) in vertices.iter().zip(vertices.iter().skip(1)) {
        let max_x = std::cmp::max(prev.0, curr.0);
        let max_y = std::cmp::max(prev.1, curr.1);
        let min_x = std::cmp::min(prev.0, curr.0);
        let min_y = std::cmp::min(prev.1, curr.1);

        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                points.insert((x, y));
            }
        }
    }

    points
}
