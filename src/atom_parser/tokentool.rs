use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space0};
use nom::error::Error;
use nom::{IResult, Parser};
use std::f64::consts::{E, PI};

use crate::atom_parser::expr_constant::{ADD_SIGN, DIV_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN};
use crate::general_const::{PARENS_0, PARENS_1};
use crate::tokenizer::{scan_float, scan_name, scan_string, Token};
/*
Explication :
    un "token" peut etre un "float"(f64) ou ('-','+','*','/','(',')','ln','V') appelé "other_token"


    space0 est utilisé ici par convention
*/
pub fn tag_other_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag(MINUS_SIGN),
        tag(ADD_SIGN),
        tag(MUL_SIGN),
        tag(DIV_SIGN),
        tag(POWER_SIGN),
        tag(PARENS_0),
        tag(PARENS_1),
        space0
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(mut input: &str) -> IResult<&str, Token> {
    input = input.trim();
    let a = alt((scan_string,scan_name,scan_float, tag_other_token)).parse(input)?;

    Ok((a.0.trim(), a.1))
}


#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
