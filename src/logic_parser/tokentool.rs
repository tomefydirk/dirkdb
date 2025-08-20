// tokentools.rs
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, multispace1};
use nom::combinator::opt;
use nom::error::Error;
use nom::{IResult, Parser, bytes::complete::take_while1};

use crate::logic_parser::cond_source::Condition;

fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    String(String),
    FieldName(String),
    Other(&'a str),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Token::Other(value)
    }
}

impl From<String> for Token<'_> {
    fn from(value: String) -> Self {
        Token::String(value)
    }
}

impl From<f64> for Token<'_> {
    fn from(value: f64) -> Self {
        Token::Number(value)
    }
}

// ----------- Parsers de tokens --------------

pub fn tag_float(input: &str) -> IResult<&str, f64> {
    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((
            rest3,
            format!("{first_part}.{second_part}").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?,
        ))
    } else {
        Ok((
            rest,
            format!("{first_part}.0").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?,
        ))
    }
}

pub fn tag_name(input: &str) -> IResult<&str, String> {
    let (rest, first) = take_while1(is_ident_start)(input)?;
    let (rest, rest_chars) = opt(take_while1(is_ident_char)).parse(rest)?;
    match rest_chars {
        Some(val) => Ok((rest, format!("{}{}", first, val))),
        None => Ok((rest, first.to_string())),
    }
}

pub fn tag_string(input: &str) -> IResult<&str, String> {
    let (rest, _) = tag("'")(input)?;
    let (rest, content) = take_while1(|c: char| c != '\'')(rest)?;
    let (rest, _) = tag("'")(rest)?;
    Ok((rest, content.to_string()))
}
pub fn scan_float(input: &str) -> IResult<&str, Token> {
    let a = tag_float(input)?;
    Ok((a.0, Token::Number(a.1)))
}

pub fn scan_name(input: &str) -> IResult<&str, Token> {
    let a = tag_name(input)?;
    Ok((a.0, Token::FieldName(a.1)))
}

pub fn scan_string(input: &str) -> IResult<&str, Token> {
    let a = tag_string(input)?;
    Ok((a.0, Token::String(a.1)))
}
pub fn tag_is_not(input: &str) -> IResult<&str, &str> {
    let (input, _) = (
        tag_no_case("is"),
        multispace1,
        tag_no_case("not"),
        multispace1,
    ).parse(input)?;
    Ok((input, ("is not")))
}
pub fn scan_other(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag("<="),
        tag(">="),
        tag("!="),
        tag("="),
        tag("<"),
        tag(">"),
        tag("("),
        tag(")"),
        tag_is_not,
        tag_no_case("or"),
        tag_no_case("and"),
        tag_no_case("is"),
        tag_no_case("not"),
        tag_no_case("null"),
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(input: &str) -> IResult<&str, Token> {
    let a = alt((scan_float, scan_other, scan_name, scan_string)).parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}

impl Token<'_> {
    pub fn to_condition<'a>(&self, input: &'a str) -> IResult<&'a str, Box<Condition>> {
        match self {
            Token::Number(n) => Ok((input, Box::new(Condition::Number(*n)))),
            Token::String(f) => Ok((input, Box::new(Condition::String(f.clone())))),
            Token::FieldName(a) => Ok((input, Box::new(Condition::Identifier(a.clone())))),
            Token::Other(a) if a.eq_ignore_ascii_case("null") => {
                Ok((input, Box::new(Condition::Null)))
            }
            a => {
                println!("to_condition : {a:?}");
                Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
            }
        }
    }
}
