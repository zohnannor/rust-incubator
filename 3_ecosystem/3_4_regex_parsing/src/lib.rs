#[cfg(all(feature = "pest", not(feature = "regex")))]
mod pest_impl;
#[cfg(all(feature = "pest", not(feature = "regex")))]
pub use pest_impl::parse_impl;

#[cfg(all(feature = "regex", not(feature = "pest")))]
mod regex_impl;
#[cfg(all(feature = "regex", not(feature = "pest")))]
pub use regex_impl::parse_impl;

#[cfg(all(feature = "pest", feature = "regex"))]
compile_error!("choose either 'regex' or 'pest' feature.");

#[derive(Debug, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    crate::parse_impl(input)
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in [
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected, "{input}");
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in [
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected, "{input}");
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in [
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected, "{input}");
        }
    }
}
