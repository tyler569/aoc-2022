use std::{str::FromStr, collections::{HashMap, hash_map::Entry}};
use aoc_rs::{get_input, parser::Parser};

#[derive(Debug)]
struct Claim {
    number: u64,
    point: (u64, u64),
    size: (u64, u64),
}

impl Claim {
    fn top(&self) -> u64 {
        self.point.0
    }

    fn bottom(&self) -> u64 {
        self.point.0 + self.size.0
    }

    fn left(&self) -> u64 {
        self.point.1
    }

    fn right(&self) -> u64 {
        self.point.1 + self.size.1
    }

    fn points(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        (self.left()..self.right())
            .flat_map(move |px| (self.top()..self.bottom())
                .map(move |py| (px, py)))
    }
}

impl FromStr for Claim {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(str);
        parser.eat('#');
        let number = parser.u64();
        parser.eat_str(" @ ");
        let ox = parser.u64();
        parser.eat(',');
        let oy = parser.u64();
        parser.eat_str(": ");
        let sx = parser.u64();
        parser.eat('x');
        let sy = parser.u64();

        Ok(Self { number, point: (ox, oy), size: (sx, sy) })
    }
}

fn parse(input: &str) -> Vec<Claim> {
    input
        .trim()
        .split('\n')
        .map(|i| i.parse().unwrap())
        .collect()
}

fn main() -> anyhow::Result<()> {
    let input = get_input(2018, 3)?;
    let claims = parse(&input);

    println!("part1: {}", part1(&claims));
    println!("part2: {}", part2(&claims).expect("No solution found!"));

    Ok(())
}

fn overlap_counts(claims: &[Claim]) -> HashMap<(u64, u64), i32> {
    let mut points = HashMap::new();

    for claim in claims {
        for point in claim.points() {
            match points.entry(point) {
                Entry::Occupied(o) => { *o.into_mut() += 1 }
                Entry::Vacant(v) => { v.insert(1); }
            }
        }
    }

    points
}

fn part1(claims: &[Claim]) -> usize {
    overlap_counts(claims).values().filter(|&&v| v > 1).count()
}

fn part2(claims: &[Claim]) -> Option<u64> {
    let overlap_counts = overlap_counts(claims);

    'next_claim: for claim in claims {
        for point in claim.points() {
            if overlap_counts[&point] != 1 {
                continue 'next_claim
            }
        }
        return Some(claim.number)
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = " \
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

    #[test]
    fn test_part1() {
        let claims = parse(SAMPLE);
        assert_eq!(part1(&claims), 4);
    }

    #[test]
    fn test_part2() {
        let claims = parse(SAMPLE);
        assert_eq!(part2(&claims), Some(3));
    }
}