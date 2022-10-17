use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = aoc_rs::get_input(2018, 1)?;
    let numbers: Vec<i64> = input
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    part1(&numbers);
    part2(&numbers);

    Ok(())
}

fn part1(input: &[i64]) {
    println!("part1: {}", input.iter().sum::<i64>());
}

fn part2(input: &[i64]) {
    let mut reached = HashSet::new();
    let mut sum = 0;
    loop {
        for v in input {
            sum += v;
            if reached.contains(&sum) {
                println!("part2: {}", sum);
                return;
            }
            reached.insert(sum);
        }
    }
}