use std::str::FromStr;

use aoc_rs::{get_input, reparse};

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 2)?;

    let strategy1 = parse1(&input);
    println!("part1: {}", part1(&strategy1));

    let strategy2 = parse2(&input);
    println!("part2: {}", part2(&strategy2));

    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play {
    fn value(self) -> i64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn beats(self, other: Self) -> GameResult {
        match (self, other) {
            (a, b) if a == b => GameResult::Draw,
            (Self::Rock, Self::Scissors) => GameResult::Win,
            (Self::Scissors, Self::Paper) => GameResult::Win,
            (Self::Paper, Self::Rock) => GameResult::Win,
            _ => GameResult::Lose,
        }
    }

    fn match_result(self, result: GameResult) -> Self {
        match (self, result) {
            (v, GameResult::Draw) => v,
            (Self::Rock, GameResult::Win) => Self::Paper,
            (Self::Rock, GameResult::Lose) => Self::Scissors,
            (Self::Paper, GameResult::Win) => Self::Scissors,
            (Self::Paper, GameResult::Lose) => Self::Rock,
            (Self::Scissors, GameResult::Win) => Self::Rock,
            (Self::Scissors, GameResult::Lose) => Self::Paper,
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),
            _ => Err(())
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    fn value(self) -> i64 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(()),
        }
    }
}

fn parse1(input: &str) -> Vec<(Play, Play)> {
    input
        .trim()
        .split('\n')
        .map(|line| reparse!((Play, Play), "(.) (.)", line))
        .collect()
}

fn value1(&(other, my): &(Play, Play)) -> i64 {
    my.value() + my.beats(other).value()
}

fn part1(strategy: &[(Play, Play)]) -> i64 {
    strategy.iter().map(value1).sum()
}

fn parse2(input: &str) -> Vec<(Play, GameResult)> {
    input
        .trim()
        .split('\n')
        .map(|line| reparse!((Play, GameResult), "(.) (.)", line))
        .collect()
}

fn value2(&(other, result): &(Play, GameResult)) -> i64 {
    let my = other.match_result(result);

    my.value() + my.beats(other).value()
}

fn part2(strategy: &[(Play, GameResult)]) -> i64 {
    strategy.iter().map(value2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
A Y
B X
C Z
";

    #[test]
    fn test_part1() {
        let strategy = parse1(SAMPLE);
        assert_eq!(part1(&strategy), 15);
    }

    #[test]
    fn test_part2() {
        let strategy = parse2(SAMPLE);
        assert_eq!(part2(&strategy), 12);
    }
}