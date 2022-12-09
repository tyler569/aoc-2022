use std::{str::FromStr, collections::HashSet};

use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 9)?;
    let instructions = parse(&input);

    println!("part1: {}", part1(&instructions));
    println!("part2: {}", part2(&instructions));

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn add(self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Self::Up => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Down => (x, y - 1),
            Self::Right => (x + 1, y),
        }
    }
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::Right),
            "L" => Ok(Self::Left),
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Instruction(Direction, i32);

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, count) = s.split_once(' ').unwrap();
        Ok(Self(direction.parse()?, count.parse().map_err(|_| ())?))
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split('\n')
        .map(|l| l.parse().unwrap())
        .collect()
}

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();

    dx.max(dy)
}

fn follow(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
    if distance(head, tail) < 2 {
        return tail
    }

    let dx = (head.0 - tail.0).clamp(-1, 1);
    let dy = (head.1 - tail.1).clamp(-1, 1);
    (tail.0 + dx, tail.1 + dy)
}

fn sample_print(field: &HashSet<(i64, i64)>) {
    for x in (-4..5).rev() {
        for y in (-4..5).rev() {
            if field.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();

    visited.insert(tail);

    for instruction in instructions {
        for _ in 0..instruction.1 {
            head = instruction.0.add(head);
            tail = follow(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut rope = vec![(0, 0); 10];
    let mut visited = HashSet::new();

    visited.insert((0, 0));

    for instruction in instructions {
        for _ in 0..instruction.1 {
            rope[0] = instruction.0.add(rope[0]);
            for i in 0..(rope.len()-1) {
                rope[i+1] = follow(rope[i], rope[i+1]);
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str =
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const SAMPLE2: &'static str =
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        let instructions = parse(&SAMPLE);
        assert_eq!(part1(&instructions), 13);
    }

    #[test]
    fn test_part2() {
        let instructions = parse(&SAMPLE);
        assert_eq!(part2(&instructions), 1);
        let instructions2 = parse(&SAMPLE2);
        assert_eq!(part2(&instructions2), 36);
    }
}
