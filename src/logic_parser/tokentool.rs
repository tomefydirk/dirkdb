// tokentools.rs
use nom::branch::alt;
use nom::{IResult, Parser};
use crate::tokenizer::{scan_float, scan_name, scan_other, scan_string, tag_binop_token, Token};



pub fn scan_token(input: &str) -> IResult<&str, Token> {
    let a = alt((scan_float, scan_other, scan_name, scan_string,tag_binop_token)).parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}


