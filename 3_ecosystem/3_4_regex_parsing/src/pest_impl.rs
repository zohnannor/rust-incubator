use crate::{Precision, Sign};

use pest::Parser;

#[allow(clippy::empty_line_after_outer_attr)]
#[derive(pest_derive::Parser)]
#[grammar_inline = r##"
format_spec  = ${ (fill? ~ align)? ~ sign? ~ "#"? ~ "0"? ~ width? ~ ("." ~ precision)? ~ type_? }

fill         = _{ ASCII_ALPHANUMERIC | !sign }
align        = _{ "<" | "^" | ">" }
sign         =  { "+" | "-" }
width        =  { count }
precision    =  { count | "*" }
type_        = _{ identifier | "?" | "" }
count        = _{ parameter | integer }
parameter    = _{ argument ~ "$" }

identifier   = _{ ASCII_ALPHANUMERIC+ }
integer      = _{ ASCII_DIGIT+ }
argument     = _{ integer }
"##]
struct FmtParser;

#[allow(clippy::missing_panics_doc)]
pub fn parse_impl(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let parsed = FmtParser::parse(Rule::format_spec, input)
        .unwrap()
        .next()
        .unwrap();

    let (mut sign, mut width, mut precision) = (None, None, None);

    for pair in parsed.into_inner() {
        dbg!(&pair);
        match pair.as_rule() {
            Rule::sign => {
                sign = match pair.as_str() {
                    "+" => Some(Sign::Plus),
                    "-" => Some(Sign::Minus),
                    _ => unreachable!(),
                };
            }
            Rule::width => {
                width = Some(pair.as_str().parse().unwrap());
            }
            Rule::precision => {
                precision = match pair.as_str() {
                    "*" => Some(Precision::Asterisk),
                    p if p.ends_with('$') => {
                        Some(Precision::Argument(p[..p.len() - 1].parse().unwrap()))
                    }
                    p => Some(Precision::Integer(p.parse().unwrap())),
                }
            }
            _ => unreachable!(),
        }
    }

    dbg!((sign, width, precision))
}
