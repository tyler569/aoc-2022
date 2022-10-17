use std::collections::{HashMap, hash_map::Entry};
use aoc_rs::get_input;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input(2018, 2)?;
    let ids: Vec<&str> = input.split_ascii_whitespace().collect();

    part1(&ids);
    part2(&ids);

    Ok(())
}

fn count_letters(str: &str) -> HashMap<char, usize> {
    let mut map = HashMap::<char, usize>::new();

    str.chars().for_each(|c| match map.entry(c) {
        Entry::Occupied(o) => { *o.into_mut() += 1 }
        Entry::Vacant(v) => { v.insert(1); }
    });

    map
}

fn part1(ids: &[&str]) {
    let counts = ids.iter().map(|s| count_letters(*s));

    let mut twos = 0;
    let mut threes = 0;

    for count in counts {
        if count.values().any(|&v| v == 2) { twos += 1 }
        if count.values().any(|&v| v == 3) { threes += 1 }
    }

    println!("{:?}", twos * threes);
}

fn common(a: &str, b: &str) -> String {
    a.chars().zip(b.chars())
        .filter_map(|(a, b)| if a == b { Some(a) } else { None })
        .collect()
}

fn part2(ids: &[&str]) {
    let longest = ids
        .iter()
        .permutations(2)
        .map(|v| common(v[0], v[1]))
        .max_by_key(|s| s.len())
        .expect("No answer found!");

    println!("{}", longest);
}