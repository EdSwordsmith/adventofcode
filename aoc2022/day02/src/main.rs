use std::io::{self, BufRead};

#[derive(Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn opponent(c: char) -> Play {
        match c {
            'A' => Play::Rock,
            'B' => Play::Paper,
            'C' => Play::Scissors,
            _ => unreachable!(),
        }
    }

    fn player(c: char) -> Play {
        match c {
            'X' => Play::Rock,
            'Y' => Play::Paper,
            'Z' => Play::Scissors,
            _ => unreachable!(),
        }
    }

    fn player_points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_result(opponent: Play, result: GameResult) -> Play {
        match (opponent, result) {
            (Play::Rock, GameResult::Win) => Self::Paper,
            (Play::Rock, GameResult::Draw) => Self::Rock,
            (Play::Rock, GameResult::Defeat) => Self::Scissors,
            (Play::Paper, GameResult::Win) => Self::Scissors,
            (Play::Paper, GameResult::Draw) => Self::Paper,
            (Play::Paper, GameResult::Defeat) => Self::Rock,
            (Play::Scissors, GameResult::Win) => Self::Rock,
            (Play::Scissors, GameResult::Draw) => Self::Scissors,
            (Play::Scissors, GameResult::Defeat) => Self::Paper,
        }
    }
}

enum GameResult {
    Win,
    Draw,
    Defeat,
}

impl GameResult {
    fn points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Defeat => 0,
        }
    }

    fn from_game(opponent: Play, me: Play) -> GameResult {
        match (opponent, me) {
            (Play::Rock, Play::Rock)
            | (Play::Paper, Play::Paper)
            | (Play::Scissors, Play::Scissors) => Self::Draw,
            (Play::Rock, Play::Scissors)
            | (Play::Paper, Play::Rock)
            | (Play::Scissors, Play::Paper) => Self::Defeat,
            (Play::Rock, Play::Paper)
            | (Play::Paper, Play::Scissors)
            | (Play::Scissors, Play::Rock) => Self::Win,
        }
    }

    fn from_char(c: char) -> GameResult {
        match c {
            'X' => Self::Defeat,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input: Vec<(Play, char)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (f, s) = line.split_once(' ').unwrap();
            let f = f.chars().next().unwrap();
            let s = s.chars().next().unwrap();
            (f, s)
        })
        .map(|(op, me)| (Play::opponent(op), me))
        .collect();

    let first_score: u32 = input
        .iter()
        .map(|(opponent, me)| (opponent, Play::player(*me)))
        .map(|(opponent, me)| GameResult::from_game(*opponent, me).points() + me.player_points())
        .sum();

    println!("Part 1: {}", first_score);

    let second_score: u32 = input
        .iter()
        .map(|(opponent, res)| (opponent, GameResult::from_char(*res)))
        .map(|(opponent, res)| res.points() + Play::from_result(*opponent, res).player_points())
        .sum();

    println!("Part 2: {}", second_score);
}
