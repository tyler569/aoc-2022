
#[macro_export]
macro_rules! reparse {
    ( ( $($T:ty),* ), $regex:literal, $string:expr) => {
        {
            use once_cell::sync::Lazy;
            use regex::Regex;

            static REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new($regex)
                    .expect("Regex did not compile")
            });

            let ca;
            let mut iter;

            if let Some(c) = REGEX.captures($string) {
                ca = c;
                iter = ca.iter();
                iter.next();
            } else {
                panic!("Regex \"{}\" did not match string \"{}\"", $regex, $string);
            }

            (
                $(iter
                    .next()
                    .expect("Not enough capture groups")
                    .expect("Nothing captured")
                    .as_str()
                    .parse::<$T>()
                    .expect("Parse failed")
                ,)*
            )
        }
    };
    ( ( $($T:ty),+ , ), $regex:expr, $string:expr) => {
        reparse!(($($T),*), $regex, $string)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_regex_parse() {
        assert_eq!(reparse!((i32,), r"(\d+)", "1"), (1i32,));

        let tests = [
            ("Santa: 1 -> 2", ("Santa".to_string(), 1, 2)),
            ("Popeye: 200 -> 4000", ("Popeye".to_string(), 200, 4000)),
            ("Jonah: 3 -> 12", ("Jonah".to_string(), 3, 12)),
        ];

        for (test, result) in tests {
            assert_eq!(reparse!((String, i32, i32), r"^(\w+): (\d+) -> (\d+)$", test), result);
        }
    }
}