use aoc_rs::get_input;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 8)?;
    let field = parse(&input);

    println!("part1: {}", part1(&field));
    println!("part2: {}", part2(&field));

    Ok(())
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split("\n")
        .map(|s| s.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn split_exclusive<T>(slice: &[T], index: usize) -> (&[T], &[T]) {
    let (s1, s2) = slice.split_at(index);
    let (_, s3) = s2.split_at(1);
    (s1, s3)
}

fn visibility_lines(field: &[Vec<u8>], x: usize, y: usize) -> Vec<Vec<u8>> {
    let ys = &field[x];
    let xs = field.iter().map(|l| l[y]).collect::<Vec<_>>();

    let (y1, y2) = split_exclusive(ys, y);
    let (x1, x2) = split_exclusive(&xs, x);

    vec![
        x1.iter().rev().map(|c| *c).collect(),
        x2.iter().map(|c| *c).collect(),
        y1.iter().rev().map(|c| *c).collect(),
        y2.iter().map(|c| *c).collect(),
    ]
}

fn is_visible(height: u8, lines: &[Vec<u8>]) -> bool {
    !lines.iter().all(|l| l.iter().any(|&v| v >= height))
}

fn part1(field: &[Vec<u8>]) -> usize {
    let mut visible = 0;

    for x in 0..field.len() {
        for y in 0..field[0].len() {
            visible += is_visible(field[x][y], &visibility_lines(field, x, y)) as usize;
        }
    }

    visible
}

fn visibility_score(height: u8, lines: &[Vec<u8>]) -> usize {
    lines.iter().map(|l| l.iter().find_position(|&&v| v >= height).map(|(i, _)| i + 1).unwrap_or(l.len())).product()
}

fn part2(field: &[Vec<u8>]) -> usize {
    (0..field.len())
        .flat_map(|x| {
            (0..field[0].len())
                .map(move |y| visibility_score(field[x][y], &visibility_lines(field, x, y)))
        })
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str =
"30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let field = parse(&SAMPLE);
        assert_eq!(part1(&field), 21);
    }

    #[test]
    fn test_part2() {
        let field = parse(&SAMPLE);
        assert_eq!(part2(&field), 8);
    }
}
