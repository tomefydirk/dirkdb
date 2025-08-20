use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, space0};
use nom::combinator::opt;
use nom::error::Error;
use nom::{IResult, Parser};
use std::f64::consts::{E, PI};

use crate::atom_parser::expr_constant::{ABS_SIGN, ADD_SIGN, COS_SIGN, DIV_SIGN, LN_SIGN, MINUS_SIGN, MUL_SIGN, PARENS_0, PARENS_1, POWER_SIGN, SIN_SIGN, SQRT_SIGN};
/*
Explication :
    un "token" peut etre un "float"(f64) ou ('-','+','*','/','(',')','ln','V') appelé "other_token"


    space0 est utilisé ici par convention
*/

pub fn scan_float(input: &str) -> IResult<&str, Token> {
    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((
            rest3,
            Token::Number(format!("{first_part}.{second_part}").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?),
        ))
    } else {
        Ok((
            rest,
            Token::Number(format!("{first_part}.0").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit))
            })?),
        ))
    }
}
pub fn scan_constant(input: &str) -> IResult<&str, Token> {
    let a = alt((tag("E"), tag("PI"))).parse(input)?;

    if a.1 == "E" {
        let r = E;
        Ok((a.0, Token::Number(r)))
    } else if a.1 == "PI" {
        let r = PI;
        Ok((a.0, Token::Number(r)))
    } else {
        Err(nom::Err::Error(Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}
pub fn tag_function(input: &str) -> IResult<&str, Token> {
    let a = alt((tag_no_case(LN_SIGN), tag_no_case(SQRT_SIGN), tag_no_case(COS_SIGN), tag_no_case(SIN_SIGN), tag_no_case(ABS_SIGN))).parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}
pub fn tag_other_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag(MINUS_SIGN),
        tag(ADD_SIGN),
        tag(MUL_SIGN),
        tag(DIV_SIGN),
        tag(POWER_SIGN),
        tag(PARENS_0),
        tag(PARENS_1),
        space0,
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(mut input: &str) -> IResult<&str, Token> {
    input = input.trim();
    let a = alt((scan_float, scan_constant, tag_function, tag_other_token)).parse(input)?;

    Ok((a.0.trim(), a.1))
}
//Enum pour les token :
#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    Other(&'a str),
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
