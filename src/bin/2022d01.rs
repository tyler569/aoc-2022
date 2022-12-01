use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 1)?;
    let elf_foods = parse(&input);

    println!("part1: {}", part1(&elf_foods)?);
    println!("part2: {}", part2(&elf_foods)?);

    Ok(())
}

fn parse(input: &str) -> Vec<Option<i64>> {
    input
        .trim()
        .split('\n')
        .map(|l| l.parse::<i64>().ok())
        .collect()
}

fn part1(elf_foods: &[Option<i64>]) -> anyhow::Result<i64> {
    Ok(elf_foods
        .split(|v| v.is_none())
        .map(|s| s.iter().map(|v| v.unwrap()).sum())
        .max()
        .unwrap())
}

fn part2(elf_foods: &[Option<i64>]) -> anyhow::Result<i64> {
    let mut ordered = elf_foods
        .split(|v| v.is_none())
        .map(|s| s.iter().map(|v| v.unwrap()).sum())
        .collect::<Vec<_>>();
    
    ordered.sort();

    Ok(ordered
        .iter()
        .rev()
        .take(3)
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn test_part1() -> anyhow::Result<()> {
        let elf_foods = parse(SAMPLE);
        assert_eq!(part1(&elf_foods)?, 24000);

        Ok(())
    }

    #[test]
    fn test_part2() -> anyhow::Result<()> {
        let elf_foods = parse(SAMPLE);
        assert_eq!(part2(&elf_foods)?, 45000);

        Ok(())
    }
}