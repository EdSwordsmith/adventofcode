use std::{collections::HashSet, io};

enum Line {
    Row(usize),
    Col(usize),
}

impl Line {
    fn height_map_index(&self, rows: usize) -> usize {
        match self {
            Line::Row(row) => *row,
            Line::Col(col) => rows + *col,
        }
    }

    fn from_index(index: usize, rows: usize) -> Line {
        if index < rows {
            Line::Row(index)
        } else {
            Line::Col(index - rows)
        }
    }
}

fn main() {
    let trees: Vec<Vec<u32>> = io::stdin()
        .lines()
        .map(|line| line.expect("Error reading line"))
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = trees.len();
    let cols = trees[0].len();
    let mut height_map: Vec<(u32, u32)> = (0..rows + cols)
        .map(|index| match Line::from_index(index, rows) {
            Line::Row(row) => (trees[row][0], trees[row][cols - 1]),
            Line::Col(col) => (trees[0][col], trees[rows - 1][col]),
        })
        .collect();

    let mut visible_trees = HashSet::new();
    for row in 0..rows {
        let row_heights = height_map
            .get_mut(Line::Row(row).height_map_index(rows))
            .unwrap();

        for col in 0..cols {
            let opposite_col = cols - col - 1;
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                visible_trees.insert((row, col));
                continue;
            }

            if trees[row][col] > row_heights.0 {
                row_heights.0 = trees[row][col];
                visible_trees.insert((row, col));
            }

            if trees[row][opposite_col] > row_heights.1 {
                row_heights.1 = trees[row][opposite_col];
                visible_trees.insert((row, opposite_col));
            }
        }
    }

    for col in 0..cols {
        let col_heights = height_map
            .get_mut(Line::Col(col).height_map_index(rows))
            .unwrap();

        for row in 0..rows {
            let opposite_row = rows - row - 1;
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                continue;
            }

            if trees[row][col] > col_heights.0 {
                col_heights.0 = trees[row][col];
                visible_trees.insert((row, col));
            }

            if trees[opposite_row][col] > col_heights.1 {
                col_heights.1 = trees[opposite_row][col];
                visible_trees.insert((opposite_row, col));
            }
        }
    }

    println!("Part 1: {}", visible_trees.len());

    let mut part_two = 0;
    for row in 0..rows {
        for col in 0..cols {
            let height = trees[row][col];

            // Top
            let mut top_dist = row;
            for r in (0..row).rev() {
                if trees[r][col] >= height {
                    top_dist -= r;
                    break;
                }
            };

            // Left
            let mut left_dist = col;
            for c in (0..col).rev() {
                if trees[row][c] >= height {
                    left_dist -= c;
                    break;
                }
            };

            // Bottom
            let mut bottom_dist = rows - row - 1;
            for r in row+1..rows {
                if trees[r][col] >= height {
                    bottom_dist = r - row;
                    break;
                }
            };

            // Right
            let mut right_dist = cols - col - 1;
            for c in col+1..cols {
                if trees[row][c] >= height {
                    right_dist = c - col;
                    break;
                }
            };

            let value = top_dist * bottom_dist * left_dist * right_dist;
            //println!("({}, {}) Value: {} (T{} B{} L{} R{})", row, col, value, top_dist, bottom_dist, left_dist, right_dist);
            part_two = std::cmp::max(part_two, value);
        }
    }

    println!("Part Two: {}", part_two);
}
