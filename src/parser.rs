use std::{str::Chars, fmt::Display};
use itertools::Itertools;

#[derive(Debug)]
pub struct Parser {
    state: String,
    at: usize,
}

#[derive(Debug)]
pub enum ParserError {
    InvalidEat(char, Option<char>),
    CouldNotParseInteger,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidEat(a, b) => f.debug_tuple("ParserError").field(&a).field(&b).finish(),
            Self::CouldNotParseInteger => f.debug_tuple("CouldNotParseInteger").finish(),
        }
    }
}

impl std::error::Error for ParserError {}

impl Parser {
    fn str_view(&self) -> &str {
        &self.state[self.at..]
    }

    fn chars(&self) -> Chars {
        self.str_view().chars()
    }

    pub fn new(data: &str) -> Self {
        Self { state: data.to_owned(), at: 0 }
    }

    pub fn try_eat(&mut self, c: char) -> bool {
        let mut chars = self.chars();
        if chars.next() == Some(c) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    pub fn eat(&mut self, c: char) -> Result<(), ParserError> {
        let mut chars = self.chars();
        let next = chars.next();
        if next == Some(c) {
            self.at += 1;
            Ok(())
        } else {
            Err(ParserError::InvalidEat(c, next))
        }
    }

    pub fn eat_str(&mut self, s: &str) -> Result<(), ParserError> {
        for c in s.chars() {
            self.eat(c)?;
        }

        Ok(())
    }

    pub fn u64_base(&mut self, base: u32) -> Result<u64, ParserError> {
        let mut chars = self.chars();

        let number = chars
            .take_while_ref(|&v| v.is_digit(base))
            .map(|c| c.to_digit(base).expect("This was just checked to be a digit"))
            .fold(0, |a, v| a * 10 + v as u64);

        let new_state = chars.as_str();
        let taken = self.state.len() - new_state.len();
        self.at = taken;

        if taken > 0 {
            Ok(number)
        } else {
            Err(ParserError::CouldNotParseInteger)
        }
    }

    pub fn u64(&mut self) -> Result<u64, ParserError> {
        self.u64_base(10)
    }

    pub fn i64_base(&mut self, base: u32) -> anyhow::Result<i64> {
        let negate = self.try_eat('-');
        let unsigned: i64 = self.u64_base(base)?.try_into()?;
        if negate {
            Ok(-unsigned)
        } else {
            Ok(unsigned)
        }
    }

    pub fn i64(&mut self) -> anyhow::Result<i64> {
        self.i64_base(10)
    }

    pub fn str_until(&mut self, delim: char) -> Result<String, ParserError> {
        let mut chars = self.chars();
        let out = chars.take_while_ref(|&v| v != delim).collect();
        self.at = self.state.len() - chars.as_str().len();

        Ok(out)
    }

    pub fn done(&self) -> bool {
        self.state.len() == self.at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eating_character() {
        let string = "1234";
        let mut parser = Parser::new(string);
        assert!(parser.eat('1').is_ok());
        assert!(parser.eat('2').is_ok());
        assert!(parser.eat('3').is_ok());
        assert!(parser.eat('4').is_ok());
    }

    #[test]
    fn test_eating_string() {
        let string = "1234";
        let mut parser = Parser::new(string);
        assert!(parser.eat_str("123").is_ok());
        assert!(parser.eat('4').is_ok());
    }

    #[test]
    #[should_panic]
    fn test_eating_wrong_character() {
        let string = "1235";
        let mut parser = Parser::new(string);
        assert!(parser.eat('1').is_ok());
        assert!(parser.eat('2').is_ok());
        assert!(parser.eat('3').is_ok());
        assert!(parser.eat('4').is_ok());
    }

    #[test]
    #[should_panic]
    fn test_eating_too_much() {
        let string = "1234";
        let mut parser = Parser::new(string);
        assert!(parser.eat('1').is_ok());
        assert!(parser.eat('2').is_ok());
        assert!(parser.eat('3').is_ok());
        assert!(parser.eat('4').is_ok());
        assert!(parser.eat('5').is_ok());
    }

    #[test]
    fn test_parsing_number() {
        let string = "1234";
        let mut parser = Parser::new(string);
        assert_eq!(parser.u64().unwrap(), 1234);
        assert!(parser.done());
    }

    #[test]
    fn test_parsing_format() -> anyhow::Result<()> {
        let string = "10x10";
        let mut parser = Parser::new(string);
        let v1 = parser.u64()?;
        parser.eat('x')?;
        let v2 = parser.u64()?;
        
        assert_eq!((v1, v2), (10, 10));
        Ok(())
    }

    #[test]
    fn test_str_until() -> anyhow::Result<()> {
        let string = "Hello World";
        let mut parser = Parser::new(string);
        assert_eq!(parser.str_until(' ')?, "Hello");
        parser.eat(' ')?;
        assert_eq!(parser.str_until(' ')?, "World");
        Ok(())
    }

    #[test]
    fn test_signed_integers() -> anyhow::Result<()> {
        let string = "[1 -2 4 -5 10 -100 1000]";
        let mut parser = Parser::new(string);
        let expected = vec![1, -2, 4, -5, 10, -100, 1000];
        let mut found = vec![];

        parser.eat('[')?;
        while !parser.try_eat(']') {
            found.push(parser.i64()?);
            parser.try_eat(' ');
        }

        assert_eq!(expected, found);

        Ok(())
    }

    #[test]
    // #[ignore]
    fn test_unknown_length() -> anyhow::Result<()> {
        let strings = [
            "a b",
            "a b c d",
            "hello world this is johnny boy",
        ];

        for string in strings {
            let mut parser = Parser::new(string);
            while !parser.done() {
                parser.str_until(' ')?;
                parser.try_eat(' ');
            }
        }

        Ok(())
    }
}