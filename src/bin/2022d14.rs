use std::collections::HashMap;
use aoc_rs::get_input;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 14)?;
    let mut field = parse(&input);

    println!("part1: {}", part1(field.clone()));
    modify_for_part_2(&mut field);
    println!("part2: {}", part1(field));

    Ok(())
}

fn parse(input: &str) -> HashMap<(i32, i32), u8> {
   let iter = input 
        .trim()
        .split("\n")
        .map(|line| {
            line
                .split(" -> ")
                .map(|entry| entry
                        .split(",")
                        .map(|n| n
                            .parse::<i32>()
                            .unwrap()
                        )
                    )
        });
    
    let mut field = HashMap::new();

    for line in iter {
        for (mut from, mut to) in line.tuple_windows() {
            let x1 = from.next().unwrap();
            let y1 = from.next().unwrap();
            let x2 = to.next().unwrap();
            let y2 = to.next().unwrap();

            // println!("{x1}-{x2} {y1}-{y2}");
            
            for x in x1..=x2 {
                for y in y1..=y2 {
                    field.insert((x, y), b'#');
                }
            }
            for x in x2..=x1 {
                for y in y2..=y1 {
                    field.insert((x, y), b'#');
                }
            }
        }
    }

    field
}

fn modify_for_part_2(field: &mut HashMap<(i32, i32), u8>) {
    let highest_position = field.keys().max_by_key(|(_x, y)| y).unwrap().1;
    for i in -500..1500 {
        field.insert((i, highest_position + 2), b'#');
    }
}

fn print_sample(field: &HashMap<(i32, i32), u8>) {
    for y in 0..=9 {
        for x in 494..=503 {
            print!("{}", *field.get(&(x, y)).unwrap_or(&b'.') as char)
        }
        println!();
    }
}

fn place_one_grain(field: &mut HashMap<(i32, i32), u8>) -> Result<(i32, i32), ()> {
    let mut place = (500, 0);
    if field.contains_key(&place) {
        return Err(());
    }
    let max_y = field.keys().max_by_key(|(_x, y)| y).unwrap().1;

    loop {
        if place.1 > max_y {
            return Err(());
        }

        if !field.contains_key(&(place.0, place.1 + 1)) {
            place.1 += 1;
            continue;
        }

        if !field.contains_key(&(place.0 - 1, place.1 + 1)) {
            place.0 -= 1;
            place.1 += 1;
            continue;
        }

        if !field.contains_key(&(place.0 + 1, place.1 + 1)) {
            place.0 += 1;
            place.1 += 1;
            continue;
        }

        field.insert(place, b'o');
        return Ok(place);
    }
}

fn part1(mut field: HashMap<(i32, i32), u8>) -> usize {
    while place_one_grain(&mut field).is_ok() {
        // print_sample(&field);
    }

    field.values().filter(|&&v| v == b'o').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() {
        let field = parse(SAMPLE);
        println!("{:?}", field);
        print_sample(&field);
        assert_eq!(part1(field), 24);
    }

    #[test]
    fn test_part2() {
        let mut field = parse(SAMPLE);
        modify_for_part_2(&mut field);
        assert_eq!(part1(field), 93);
    }
}