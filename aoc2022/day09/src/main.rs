use std::{io, collections::HashSet, ops::{AddAssign, Sub}};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Vec2(i64, i64);

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Vec2 {
    fn largest_abs_component(&self) -> i64 {
        std::cmp::max(self.0.abs(), self.1.abs())
    }

    fn fake_normalized(&self) -> Vec2 {
        Vec2(self.0.signum(), self.1.signum())
    }
}

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl Movement {
    fn parse(s: &impl AsRef<str>) -> Movement {
        let (dir, step) = s.as_ref().split_once(' ').unwrap();
        let step = step.parse().unwrap();
        match dir.chars().next().unwrap() {
            'U' => Self::Up(step),
            'D' => Self::Down(step),
            'R' => Self::Right(step),
            'L' => Self::Left(step),
            _ => unreachable!("Bro"),
        }
    }

    fn delta(&self) -> (Vec2, u32) {
        match self {
            Movement::Up(v) => (Vec2(0, 1), *v),
            Movement::Down(v) => (Vec2(0, -1), *v),
            Movement::Left(v) => (Vec2(-1, 0), *v),
            Movement::Right(v) => (Vec2(1, 0), *v),
        }
    }
}

fn move_rope(rope: &mut [Vec2], movements: &[Movement]) -> usize {
    let mut tail_positions = HashSet::new();
    tail_positions.insert(*rope.last().unwrap());

    for movement in movements.iter() {
        let (delta, distance) = movement.delta();

        for _ in 0..distance {
            for i in 0..rope.len() {
                if i == 0 {
                    rope[0] += delta;
                    continue;
                }

                let tail_to_head = rope[i - 1] - rope[i];
                if tail_to_head.largest_abs_component() <= 1 {
                    continue;
                }

                rope[i] += tail_to_head.fake_normalized();
            }

            tail_positions.insert(*rope.last().unwrap());
        }
    }

    tail_positions.len()
}

fn main() {
    let movements: Vec<Movement> = io::stdin()
        .lines()
        .map(|line| Movement::parse(&line.unwrap()))
        .collect();

    let mut rope = [Vec2(0, 0); 2];
    println!("Part 1: {}", move_rope(&mut rope, &movements));

    let mut rope = [Vec2(0, 0); 10];
    println!("Part 2: {}", move_rope(&mut rope, &movements));
}
