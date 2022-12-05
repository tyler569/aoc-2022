use aoc_rs::{get_input, reparse};

fn main() -> anyhow::Result<()> {
    let input = get_input(2022, 5)?;
    /*
    "move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2";
    */

    let sample_piles = vec![
        vec!['Z', 'N'],
        vec!['M', 'C', 'D'],
        vec!['P'],
    ];

    let my_piles = vec![
        vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
        vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
        vec!['L', 'Q', 'V'],
        vec!['N', 'B', 'S', 'W', 'R', 'Q'],
        vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
        vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
        vec!['D', 'B', 'Q', 'J'],
        vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
        vec!['B', 'N', 'H', 'M', 'S'],
    ];

    let mut piles = my_piles.clone();

    let moves = parse_moves(&input);

    for (count, from, to) in moves {
        for _ in 0..count {
            let v = piles[from - 1].pop().unwrap();
            piles[to - 1].push(v);
        }
    }

    print!("part1: ");
    for pile in piles {
        print!("{}", pile.last().unwrap());
    }
    println!();

    let mut piles = my_piles.clone();

    let moves = parse_moves(&input);

    for (count, from, to) in moves {
        let fp = &mut piles[from - 1];
        let fi = fp.len() - count;
        let fv = fp.split_off(fi);
        piles[to - 1].extend_from_slice(&fv);
    }

    print!("part2: ");
    for pile in piles {
        print!("{}", pile.last().unwrap());
    }
    println!();

    Ok(())
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .trim()
        .split("\n")
        .skip_while(|line| !line.contains("move"))
        .map(|line| reparse!((usize, usize, usize), r"move (\d+) from (\d+) to (\d+)", line))
        .collect()
}
