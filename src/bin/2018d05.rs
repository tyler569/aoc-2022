use aoc_rs::get_input;

fn main() -> anyhow::Result<()> {
    let input = get_input(2018, 5)?;
    let polymer = input.trim();

    println!("part1: {}", part1(polymer));
    println!("part2: {}", part2(polymer));

    Ok(())
}

fn react(polymer: &mut dyn Iterator<Item = char>) -> String {
    fn can_react(a: char, b: char) -> bool {
        a != b && a.to_ascii_lowercase() == b.to_ascii_lowercase()
    }

    let mut output = String::new();
    for c in polymer {
        if !output.is_empty() && can_react(output.chars().last().unwrap(), c) {
            output.pop();
        } else {
            output.push(c);
        }
    }

    output
}

fn part1(polymer: &str) -> usize {
    react(&mut polymer.chars()).len()
}

fn part2(polymer: &str) -> usize {
    ('a'..'z')
        .map(|l| polymer.chars().filter(move |v| v.to_ascii_lowercase() != l))
        .map(|mut p| react(&mut p).len())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 10);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 4);
    }
}