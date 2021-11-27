use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Present(u32, u32, u32);

impl Present {
    fn from(present: String) -> Self {
        let mut dimensions = present.split("x").map(|s| s.parse::<u32>().unwrap());
        let w = dimensions.next().unwrap();
        let h = dimensions.next().unwrap();
        let l = dimensions.next().unwrap();
        return Present(w, h, l)
    }

    fn areas(&self) -> Vec<u32> {
        vec![self.0 * self.1, self.0 * self.2, self.1 * self.2]
    }

    fn perimeters(&self) -> Vec<u32> {
        vec![self.0 * 2 + self.1 * 2, self.0 * 2 + self.2 * 2, self.1 * 2 + self.2 * 2]
    }

    fn wrapping_paper(&self) -> u32 {
        let areas = self.areas();
        let smallest = areas.iter().min().unwrap();
        let total = areas.iter().fold(0, |acc, x| acc + 2 * x);
        smallest + total
    }

    fn ribbon(&self) -> u32 {
        let perimeters = self.perimeters();
        let smallest = perimeters.iter().min().unwrap();
        let volume = self.0 * self.1 * self.2;
        smallest + volume
    }
}

fn main() {
    // Get input filename
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];

    let file = File::open(input_file).unwrap();
    let reader = BufReader::new(file);

    let (wrapping, ribbon) = reader
        .lines()
        .map(|line| Present::from(line.unwrap()))
        .map(|present| (present.wrapping_paper(), present.ribbon()))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("Part 1: {}", wrapping);
    println!("Part 2: {}", ribbon);
}
