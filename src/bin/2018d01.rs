use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = aoc_rs::get_input(2018, 1)?;
    let numbers = parse(&input);

    println!("part1: {}", part1(&numbers));
    println!("part2: {}", part2(&numbers));

    Ok(())
}

fn parse(input: &str) -> Vec<i64> {
    input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn part1(input: &[i64]) -> i64 {
    input.iter().sum::<i64>()
}

fn part2(input: &[i64]) -> i64 {
    let mut reached = HashSet::new();
    let mut sum = 0;
    loop {
        for v in input {
            sum += v;
            if reached.contains(&sum) {
                return sum;
            }
            reached.insert(sum);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
    +1
    -2
    +3
    +1
    ";

    #[test]
    fn test_part1() {
        let numbers = parse(SAMPLE);
        assert_eq!(part1(&numbers), 3);
    }

    #[test]
    fn test_part2() {
        let numbers = parse(SAMPLE);
        assert_eq!(part2(&numbers), 2);
    }
}