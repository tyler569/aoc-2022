use regex::Regex;
use once_cell::sync::Lazy;

macro_rules! reparse {
    ($regex:expr, $string:expr ; ( $($T:ty),* )) => {
        {
            static REGEX: Lazy<Regex> = Lazy::new(|| {
                Regex::new($regex).unwrap()
            });

            let ca = REGEX.captures($string).unwrap();
            let mut iter = ca.iter();
            iter.next();

            (
                $(iter.next().unwrap().unwrap().as_str().parse::<$T>().unwrap(),)*
            )
        }
    }
}

#[test]
fn test_regex_parse() {
    assert_eq!(reparse!(r"(\d+)", "1"; (i32)), (1i32,));

    let tests = [
        ("Santa: 1 -> 2", ("Santa".to_string(), 1, 2)),
        ("Popeye: 200 -> 4000", ("Popeye".to_string(), 200, 4000)),
        ("Jonah: 3 -> 12", ("Jonah".to_string(), 3, 12)),
    ];

    for (test, result) in tests {
        assert_eq!(reparse!(r"^(\w+): (\d+) -> (\d+)$", test; (String, i32, i32)), result);
    }
}