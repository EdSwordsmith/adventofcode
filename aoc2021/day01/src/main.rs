fn main() {
    let input = include_str!("../input.in");

    let depths: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let num_increases = depths
        .iter()
        .zip(depths.iter().skip(1))
        .filter(|(c1, c2)| c1 < c2)
        .count();
    println!("Part 1: {}", num_increases);

    let sums: Vec<u32> = depths
        .iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((c1, c2), c3)| c1 + c2 + c3)
        .collect();
    let sum_increases = sums
        .iter()
        .zip(sums.iter().skip(1))
        .filter(|(c1, c2)| c1 < c2)
        .count();
    println!("Part 2: {}", sum_increases);
}
