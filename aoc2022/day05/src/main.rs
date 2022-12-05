use std::{env, fs};

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_str(instruction: &str) -> Instruction {
        let mut instruction_args = instruction.trim().split(' ');
        let amount = instruction_args.nth(1).unwrap().parse().unwrap();
        let from = instruction_args.nth(1).unwrap().parse().unwrap();
        let to = instruction_args.nth(1).unwrap().parse().unwrap();

        Instruction { amount, from, to }
    }
}

fn parse_initial_state(state: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let mut stacks = Vec::new();
    let mut stacks_copy = Vec::new();

    // Parse the initial state
    for line in state.lines() {
        for (i, c) in line.chars().enumerate() {
            if i % 4 != 1 {
                continue;
            }

            let stack_index = i / 4;
            if stack_index + 1 > stacks.len() {
                stacks.push(Vec::new());
                stacks_copy.push(Vec::new());
            }

            if c.is_alphabetic() {
                stacks[stack_index].insert(0, c);
                stacks_copy[stack_index].insert(0, c);
            }
        }
    }

    (stacks, stacks_copy)
}

fn main() {
    let input_path = env::args().nth(1).unwrap();
    let input = fs::read_to_string(input_path).unwrap();
    let (state, instructions) = input.split_once("\n\n").unwrap();

    let (mut stacks, stacks_copy) = parse_initial_state(state);
    let instructions: Vec<Instruction> = instructions.lines().map(|instruction| Instruction::from_str(instruction)).collect();

    for instruction in instructions.iter() {
        for _ in 0..instruction.amount {
            let v = stacks[instruction.from - 1].pop().unwrap();
            stacks[instruction.to - 1].push(v);
        }
    }

    let part_one: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("Part 1: {}", part_one);

    let mut stacks = stacks_copy;

    for instruction in instructions.iter() {
        let mut values = Vec::new();
        for _ in 0..instruction.amount {
            let v = stacks[instruction.from - 1].pop().unwrap();
            values.push(v);
        }

        for v in values.iter().rev() {
            stacks[instruction.to - 1].push(*v);
        }
    }

    let part_two: String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    println!("Part 2: {}", part_two);
}
