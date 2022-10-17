use std::str::FromStr;
use aoc_rs::{get_input, parser::Parser};

#[derive(Debug)]
struct Claim {
    number: u64,
    offset: (u64, u64),
    size: (u64, u64),
}

impl Claim {
    fn area(&self) -> u64 {
        self.size.0 * self.size.1
    }
}

impl FromStr for Claim {
    type Err = anyhow::Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(str);
        parser.eat('#')?;
        let number = parser.u64()?;
        parser.eat_str(" @ ")?;
        let ox = parser.u64()?;
        parser.eat(',')?;
        let oy = parser.u64()?;
        parser.eat_str(": ")?;
        let sx = parser.u64()?;
        parser.eat('x')?;
        let sy = parser.u64()?;

        Ok(Self { number, offset: (ox, oy), size: (sx, sy) })
    }
}

fn main() -> anyhow::Result<()> {
    let input = get_input(2018, 3)?;
    let claims: Vec<_> =
        input
            .split('\n')
            .map(|i| Claim::from_str(i))
            .collect();

    claims.iter().for_each(|c| println!("{:?}", c));

    Ok(())
}