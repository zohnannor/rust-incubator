use regex::Regex;

use crate::{Precision, Sign};

#[allow(clippy::missing_panics_doc)]
pub fn parse_impl(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    lazy_static::lazy_static! {
        static ref FMT: Regex = Regex::new(r"^(:?[^-+]?[<^>])?(?P<sign>[+-])?#?0?(?P<width>(:?\d+\$)|\d+)?(:?\.(?P<precision>\d+\$?|\*))?(:?\w+|\?|)?$").unwrap();
    }

    let captures = FMT.captures(input).unwrap();

    dbg!(&captures);

    let sign = match captures.name("sign").map(|m| m.as_str()) {
        Some("+") => Some(Sign::Plus),
        Some("-") => Some(Sign::Minus),
        Some(_) => unreachable!(),
        None => None,
    };

    let width = captures
        .name("width")
        .map(|m| m.as_str())
        .map(|m| m.parse().unwrap());

    let precision = match captures.name("precision").map(|m| m.as_str()) {
        Some("*") => Some(Precision::Asterisk),
        Some(m) if m.ends_with('$') => Some(Precision::Argument(m[..m.len() - 1].parse().unwrap())),
        Some(m) => Some(Precision::Integer(m.parse().unwrap())),
        None => None,
    };

    (sign, width, precision)
}
