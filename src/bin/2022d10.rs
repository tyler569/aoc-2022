use std::{str::FromStr, collections::HashSet};

use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 10)?;
    let instructions = parse(&input);

    println!("part1: {}", part1(&instructions));

    println!("sample 2:");
    part2(&parse(SAMPLE));

    println!("part2:");
    part2(&instructions);

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        match parts.next() {
            Some("addx") => Ok(Self::Addx(parts.next().unwrap().parse().unwrap())),
            Some("noop") => Ok(Self::Noop),
            _ => Err(()),
        }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split('\n')
        .map(|l| l.parse().unwrap())
        .collect()
}

fn part1(instructions: &[Instruction]) -> i64 {
    let mut cycle = 1;
    let mut x = 1;
    let mut twenties = vec![];

    instructions.iter().for_each(|instr| {
        match instr {
            Instruction::Addx(v) => {
                if cycle % 40 == 20 {
                    twenties.push(x * cycle);
                }
                cycle += 1;
                if cycle % 40 == 20 {
                    twenties.push(x * cycle);
                }
                x += v;
                cycle += 1;
            }
            Instruction::Noop => {
                if cycle % 40 == 20 {
                    twenties.push(x * cycle);
                }
                cycle += 1;
            },
        }
    });

    twenties.iter().sum()
}

fn position(cycle: i64) -> (i64, i64) {
    (cycle / 40, cycle % 40)
}

fn print_screen(screen: HashSet<(i64, i64)>) {
    for r in 0..6 {
        for c in 0..40 {
            if screen.contains(&(r, c)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn set_screen(cycle: i64, x: i64, screen: &mut HashSet<(i64, i64)>) {
    let sprite = (x-1)..=(x+1);
    let position = position(cycle);
    if sprite.contains(&position.1) {
        screen.insert(position);
    }
}

fn part2(instructions: &[Instruction]) {
    let mut cycle = 0;
    let mut x = 1;
    let mut screen = HashSet::new();

    instructions.iter().for_each(|instr| {
        match instr {
            Instruction::Addx(v) => {
                set_screen(cycle, x, &mut screen);
                cycle += 1;
                set_screen(cycle, x, &mut screen);
                cycle += 1;
                x += v;
            }
            Instruction::Noop => {
                set_screen(cycle, x, &mut screen);
                cycle += 1;
            },
        }
    });

    print_screen(screen);
}


const SAMPLE: &'static str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let instructions = parse(&SAMPLE);
        assert_eq!(part1(&instructions), 13140);
    }
}
