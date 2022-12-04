use std::io;

struct SectionRange {
    from: u32,
    to: u32,
}

impl SectionRange {
    fn from_str(s: &str) -> SectionRange {
        let (from, to) = s.split_once('-').unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        SectionRange { from, to }
    }

    fn contains(&self, other: &SectionRange) -> bool {
        other.from >= self.from && other.to <= self.to
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        (other.from >= self.from && other.from <= self.to)
            || (other.to <= self.to && other.to >= self.from)
    }
}

fn parse_line(line: String) -> (SectionRange, SectionRange) {
    let (first_range, second_range) = line.trim().split_once(',').unwrap();
    (
        SectionRange::from_str(first_range),
        SectionRange::from_str(second_range),
    )
}

fn main() {
    let pairs: Vec<(SectionRange, SectionRange)> = io::stdin()
        .lines()
        .map(|line| parse_line(line.unwrap()))
        .collect();

    let first_part = pairs
        .iter()
        .filter(|(r1, r2)| r1.contains(r2) || r2.contains(r1))
        .count();

    println!("Part 1: {}", first_part);

    let second_part = pairs
        .iter()
        .filter(|(r1, r2)| r1.overlaps(r2) || r2.overlaps(r1))
        .count();

    println!("Part 1: {}", second_part);
}
