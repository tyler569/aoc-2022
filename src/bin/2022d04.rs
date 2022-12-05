use std::ops::RangeInclusive;

use aoc_rs::{get_input, reparse};

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 4)?;
    let pairs = parse(&input);

    println!("part1: {}", part1(&pairs));
    println!("part2: {}", part2(&pairs));

    Ok(())
}

fn parse(input: &str) -> Vec<(RangeInclusive<i64>, RangeInclusive<i64>)> {
    input
        .trim()
        .split('\n')
        .map(|v| reparse!((i64, i64, i64, i64), r"^(\d+)-(\d+),(\d+)-(\d+)$", v))
        .map(|(a, b, c, d)| ((a..=b), (c..=d)))
        .collect()
}

fn fully_contains(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    (a.contains(&b.start()) && a.contains(&b.end())) ||
    (b.contains(&a.start()) && b.contains(&a.end()))
}

fn partially_contains(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> bool {
    (a.contains(&b.start()) || a.contains(&b.end())) ||
    (b.contains(&a.start()) || b.contains(&a.end()))
}

fn part1(pairs: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    pairs.iter().filter(|(a, b)| fully_contains(a, b)).count()
}

fn part2(pairs: &[(RangeInclusive<i64>, RangeInclusive<i64>)]) -> usize {
    pairs.iter().filter(|(a, b)| partially_contains(a, b)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn test_part1() {
        let pairs = parse(SAMPLE);
        assert_eq!(part1(&pairs), 2);
    }

    #[test]
    fn test_part2() {
        let pairs = parse(SAMPLE);
        assert_eq!(part2(&pairs), 4);
    }
}