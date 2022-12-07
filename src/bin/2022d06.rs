use aoc_rs::get_input;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 6)?;

    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));

    Ok(())
}

fn all_distinct<T: PartialEq>(slice: &[T]) -> bool {
    for (i, a) in slice.iter().enumerate() {
        for b in &slice[i+1..] {
            if a == b {
                return false
            }
        }
    }

    true
}

fn part1(signal: &str) -> usize {
    signal
        .as_bytes()
        .windows(4)
        .find_position(|w| all_distinct(w))
        .unwrap()
        .0 + 4
}

fn part2(signal: &str) -> usize {
    signal
        .as_bytes()
        .windows(14)
        .find_position(|w| all_distinct(w))
        .unwrap()
        .0 + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}