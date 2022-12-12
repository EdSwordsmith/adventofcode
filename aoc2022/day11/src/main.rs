use std::{collections::VecDeque, io};

#[derive(Debug, Clone, Copy)]
enum Value {
    Old,
    Num(u64),
}

impl Value {
    fn get(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Num(value) => *value,
        }
    }

    fn parse(s: &impl AsRef<str>) -> Self {
        let s = s.as_ref();
        if s == "old" {
            Value::Old
        } else {
            Value::Num(s.parse().unwrap())
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Value, Value),
    Mult(Value, Value),
}

impl Operation {
    fn parse(s: &impl AsRef<str>) -> Self {
        let mut split = s.as_ref().split(' ');
        let v1 = Value::parse(&split.next().unwrap());
        let op = split.next().unwrap();
        let v2 = Value::parse(&split.next().unwrap());

        if op == "+" {
            Operation::Add(v1, v2)
        } else {
            Operation::Mult(v1, v2)
        }
    }

    fn evaluate(&self, old: u64) -> u64 {
        match self {
            Operation::Add(v1, v2) => v1.get(old) + v2.get(old),
            Operation::Mult(v1, v2) => v1.get(old) * v2.get(old),
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    count: usize,
}

fn main() {
    let mut monkeys = parse_input();
    let monkeys_clone: Vec<Monkey> = monkeys.iter().map(|m| {
        Monkey { items: m.items.clone(), operation: m.operation, test: m.test, if_true: m.if_true, if_false: m.if_false, count: 0 }
    }).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkeys[i].count += 1;
                let mut item = monkeys[i].items.pop_front().unwrap();
                item = monkeys[i].operation.evaluate(item);
                item /= 3;
                let target = if item % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    let [first, second] = get_most_count(&monkeys);

    println!("Part 1: {}", first * second);

    let mut monkeys = monkeys_clone;
    let magic_number: u64 = monkeys.iter().map(|monkey| monkey.test).product();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                monkeys[i].count += 1;
                let mut item = monkeys[i].items.pop_front().unwrap();
                let res = monkeys[i].operation.evaluate(item);
                item = res % magic_number;
                let target = if res % monkeys[i].test == 0 {
                    monkeys[i].if_true
                } else {
                    monkeys[i].if_false
                };
                monkeys[target].items.push_back(item);
            }
        }
    }

    let [first, second] = get_most_count(&monkeys);
    println!("Part 2: {}", first * second);
    
}

fn get_most_count(monkeys: &[Monkey]) -> [usize; 2] {
    monkeys
        .iter()
        .map(|monkey| monkey.count)
        .fold([0; 2], |mut acc, item| {
            if item > acc[0] {
                acc[1] = acc[0];
                acc[0] = item;
            } else if item > acc[1] {
                acc[1] = item;
            }

            acc
        })
}

fn parse_input() -> Vec<Monkey> {
    let lines: Vec<String> = io::stdin()
        .lines()
        .map(|line| line.unwrap().trim().to_owned())
        .filter(|line| !line.is_empty())
        .collect();
    let mut monkeys = Vec::new();

    for monkey_lines in lines.windows(6).step_by(6) {
        let items = &monkey_lines[1][16..];
        let items: VecDeque<u64> = items
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();

        let operation = Operation::parse(&&monkey_lines[2][17..]);

        let test = &monkey_lines[3][19..];
        let test = test.parse::<u64>().unwrap();

        let if_true = &monkey_lines[4][25..];
        let if_true = if_true.parse::<usize>().unwrap();

        let if_false = &monkey_lines[5][26..];
        let if_false = if_false.parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            count: 0,
        });
    }

    monkeys
}
