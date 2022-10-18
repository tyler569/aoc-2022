use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2019, 1)?;
    let numbers = parse(&input);

    println!("part1: {}", part1(&numbers));
    println!("part2: {}", part2(&numbers));

    Ok(())
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().expect("Malformed input"))
        .collect()
}

fn part1(modules: &[i64]) -> i64 {
    modules.iter()
        .map(|m| m / 3 - 2)
        .sum()
}

fn module_fuel(module: i64) -> i64 {
    if module <= 0 { 
        return 0
    }

    let fuel = std::cmp::max(module / 3 - 2, 0);

    module + module_fuel(fuel)
}

fn part2(modules: &[i64]) -> i64 {
    modules.iter().map(|&v| module_fuel(v) - v).sum()
}