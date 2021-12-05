use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type DiagramMap = HashMap<i32, HashMap<i32, i32>>;
type Point = (i32, i32);

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input_file = File::open(input_path).unwrap();
    let reader = BufReader::new(input_file);
    let lines: Vec<(Point, Point)> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut split = line.split(" -> ").map(|v| {
                let mut split = v.split(',').map(|n| n.parse::<i32>().unwrap());
                (split.next().unwrap(), split.next().unwrap())
            });

            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    let part_one: usize = lines
        .iter()
        .fold(DiagramMap::new(), |mut map, (start, end)| {
            add_line_to_map(&mut map, *start, *end, true);
            map
        })
        .iter()
        .map(|(_, y_map)| y_map.iter().filter(|(_, y)| (**y) > 1).count())
        .sum();

    let part_two: usize = lines
        .iter()
        .fold(DiagramMap::new(), |mut map, (start, end)| {
            add_line_to_map(&mut map, *start, *end, false);
            map
        })
        .iter()
        .map(|(_, y_map)| y_map.iter().filter(|(_, y)| (**y) > 1).count())
        .sum();

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", part_two);
}

fn get_increment(start: Point, end: Point) -> Point {
    let d0 = end.0 as i32 - start.0;
    let d1 = end.1 as i32 - start.1;
    (d0.signum(), d1.signum())
}

fn add_points(point: Point, increment: Point) -> Point {
    let mut res = point;
    res.0 = res.0 + increment.0;
    res.1 = res.1 + increment.1;
    res
}

fn add_line_to_map(map: &mut DiagramMap, start: Point, end: Point, part_one: bool) {
    if start.0 != end.0 && start.1 != end.1 && part_one {
        return;
    }

    let mut current = start;
    let increment = get_increment(start, end);
    let end = add_points(end, increment);

    while current != end {
        let (x, y) = current;

        if !map.contains_key(&x) {
            map.insert(x, HashMap::new());
        }

        if let Some(y_map) = map.get_mut(&x) {
            let zero = 0;
            let current = *y_map.get(&y).unwrap_or(&zero);
            y_map.insert(y, current + 1);
        }

        current = add_points(current, increment);
    }
}
