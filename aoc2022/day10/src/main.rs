use std::{io};

#[derive(Debug)]
enum Instruction {
    AddX(i64),
    NoOp
}

impl Instruction {
    fn execute(&self, register: &mut i64) {
        match self {
            Instruction::AddX(v) => *register += v,
            Instruction::NoOp => {},
        }
    }

    fn duration(&self) -> i64 {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }

    fn parse(s: String) -> Instruction {
        if s.starts_with('n') {
            return Instruction::NoOp;
        }

        let value = &s[5..];
        let value = value.parse().unwrap();
        Instruction::AddX(value)
    }
}

fn main() {
    let instructions: Vec<Instruction> = io::stdin()
        .lines()
        .map(|line| Instruction::parse(line.unwrap()))
        .collect();

    
    let mut cycle = 1;
    let mut register = 1;
    let mut part_one = 0;
    let mut screen = [['.'; 40]; 6];

    for instruction in instructions.iter() {
        for _ in 0..instruction.duration() {
            let row = (cycle - 1) / 40;
            let col = (cycle - 1) % 40;
            let dist = register - (col as i64);
            if dist.abs() <= 1 {
                screen[row][col] = '#';
            }

            if (cycle - 20) % 40 == 0 {
                part_one += (cycle as i64) * register;
            }

            cycle += 1;
        }

        instruction.execute(&mut register);
    }

    println!("Part 1: {}", part_one);

    for line in screen.iter() {
        let line: String = line.iter().collect();
        println!("{}", line);
    }
}
