use std::str::FromStr;

use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 3)?;
    let ruksacks = parse(&input);

    println!("part1: {}", part1(&ruksacks));
    println!("part2: {}", part2(&ruksacks));

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Item(u8);

impl Item {
    fn priority(self) -> u8 {
        match self.0 {
            v @ b'a'..=b'z' => v - b'a' + 1,
            v @ b'A'..=b'Z' => v - b'A' + 27,
            _ => panic!("Impossible!"),
        }
    }

    fn from_char(c: char) -> Result<Self, ()> {
        match c {
            v @ 'a'..='z' => Ok(Self(v as u8)),
            v @ 'A'..='Z' => Ok(Self(v as u8)),
            _ => Err(()),
        }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(())
        }

        Self::from_char(s.chars().next().unwrap())
    }
}

struct Ruksack(Vec<Item>);

impl Ruksack {
    fn pockets(&self) -> (&[Item], &[Item]) {
        self.0.split_at(self.0.len() / 2)
    }
}

impl FromStr for Ruksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().map(|c| Item::from_char(c)).collect::<Result<Vec<_>, ()>>()?))
    }
}

fn parse(input: &str) -> Vec<Ruksack> {
    input
        .trim()
        .split('\n')
        .map(|v| v.parse().unwrap())
        .collect()
}

fn common_item(p1: &[Item], p2: &[Item]) -> Item {
    for i in p1 {
        if p2.contains(i) {
            return *i
        }
    }

    panic!("No mismatched item")
}

fn part1(ruksacks: &[Ruksack]) -> i64 {
    let mut total = 0_i64;

    for sack in ruksacks {
        let (p1, p2) = sack.pockets();
        let common_item = common_item(p1, p2);
        total += common_item.priority() as i64;
    }

    total
}

fn group_badge(sacks: &[Ruksack]) -> Item {
    for i in &sacks[0].0 {
        if sacks[1].0.contains(i) && sacks[2].0.contains(i) {
            return *i;
        }
    }

    panic!("No badge found")
}

fn part2(ruksacks: &[Ruksack]) -> i64 {
    let mut total = 0_i64;

    for sacks in ruksacks.chunks(3) {
        let badge_item = group_badge(sacks);
        total += badge_item.priority() as i64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

    #[test]
    fn test_part1() {
        let ruksacks = parse(SAMPLE);
        assert_eq!(part1(&ruksacks), 157);
    }

    #[test]
    fn test_part2() {
        let ruksacks = parse(SAMPLE);
        assert_eq!(part2(&ruksacks), 70);
    }
}