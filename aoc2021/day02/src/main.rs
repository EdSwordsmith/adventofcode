use std::env;
use std::fs;

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Command {
    fn new(command: &str) -> Command {
        let mut split = command.split(' ');
        let dir = split.next().unwrap();
        let amount: u32 = split.next().unwrap().parse().unwrap();

        match dir {
            "forward" => Command::Forward(amount),
            "up" => Command::Up(amount),
            "down" => Command::Down(amount),
            _ => panic!("Invalid command"),
        }
    }
}

fn main() {
    let input_file = env::args().nth(1).expect("You need to pass an input file!");
    let input = fs::read_to_string(input_file).unwrap();

    let commands: Vec<Command> = input.lines().map(|line| Command::new(line)).collect();

    let (pos1, depth1) = commands
        .iter()
        .fold((0, 0), |(pos, depth), command| match command {
            Command::Forward(amount) => (pos + amount, depth),
            Command::Up(amount) => (pos, depth - amount),
            Command::Down(amount) => (pos, depth + amount),
        });

    let (pos2, depth2, _) = commands
        .iter()
        .fold((0, 0, 0), |(pos, depth, aim), command| match command {
            Command::Forward(amount) => (pos + amount, depth + amount * aim, aim),
            Command::Up(amount) => (pos, depth, aim - amount),
            Command::Down(amount) => (pos, depth, aim + amount),
        });

    println!("Part 1: {}", pos1 * depth1);
    println!("Part 2: {}", pos2 * depth2);
}
