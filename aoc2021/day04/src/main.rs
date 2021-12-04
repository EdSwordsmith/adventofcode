use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

struct BingoBoard {
    board: [u32; 25],
    marked: [bool; 25],
    won: bool,
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            board: [0; 25],
            marked: [false; 25],
            won: false,
        }
    }

    fn parse_line(&mut self, line: &impl AsRef<str>, i: usize) {
        let mut numbers = line
            .as_ref()
            .split(' ')
            .map(|n| n.parse::<i32>().unwrap_or(-1))
            .filter(|n| *n != -1)
            .map(|n| n as u32);

        for j in 0..5 {
            self.board[i * 5 + j] = numbers.next().unwrap();
        }
    }

    fn check_row(&self, row: usize) -> bool {
        let mut ret = true;
        for col in 0..5 {
            if !self.marked[row * 5 + col] {
                ret = false;
                break;
            }
        }
        ret
    }

    fn check_col(&self, col: usize) -> bool {
        let mut ret = true;
        for row in 0..5 {
            if !self.marked[row * 5 + col] {
                ret = false;
                break;
            }
        }
        ret
    }

    fn mark(&mut self, num: u32) -> bool {
        for (i, n) in self.board.iter().enumerate() {
            if *n == num {
                self.marked[i] = true;
                let row = i / 5;
                let col = i % 5;
                self.won = self.check_row(row) || self.check_col(col)
            }
        }

        self.won
    }

    fn score(&self) -> u32 {
        self.board
            .iter()
            .zip(self.marked.iter())
            .filter(|(_, is_marked)| !**is_marked)
            .map(|(num, _)| num)
            .sum()
    }
}

fn main() -> Result<()> {
    // Open file and get reader
    let file_path = env::args().nth(1).expect("Input file needed!");
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    // Get the numbers
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()?
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    // Get the bingo boards
    let mut boards: Vec<BingoBoard> = Vec::new();
    for mut chunk in &lines.chunks(6) {
        // ignore the first line
        chunk.next().unwrap()?;
        let board = chunk.map(|line| line.unwrap()).enumerate().fold(
            BingoBoard::new(),
            |mut board, (i, line)| {
                board.parse_line(&line, i);
                board
            },
        );
        boards.push(board);
    }

    // Iterate over numbers and mark boards
    let mut first = true;
    for number in numbers.iter() {
        let last = boards.len() == 1;
        
        for board in boards.iter_mut() {
            let won = board.mark(*number);
            
            if won && first {
                println!("Part 1: {}", *number * board.score());
                first = false;
            } else if won && last {
                println!("Part 2: {}", *number * board.score());
            }
        }

        boards.retain(|board| !board.won);
    }

    Ok(())
}
