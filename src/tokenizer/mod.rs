pub mod helper;
pub mod tag_func;
use crate::IResult;
use crate::general_const::{
    ADD_SIGN, AND_SIGN, COMMA_SIGN, DIV_SIGN, EQ_SIGN, GT_E_SIGN, GT_SIGN, IS_SIGN, LIKE_SIGN,
    LT_E_SIGN, LT_SIGN, MINUS_SIGN, MOD_SIGN, MUL_SIGN, NOT_EQ_SIGN, NOT_SIGN, NULL_SIGN, OR_SIGN,
    POWER_SIGN, SEMICOLON_SIGN,
};
use crate::general_const::{PARENS_0, PARENS_1};
use crate::tokenizer::tag_func::{
    tag_float, tag_function, tag_is_not, tag_key_word_logic, tag_name, tag_string,
};
use nom::Parser;
///TOKENTOOL::
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::space0;

#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    String(String),
    FieldName(String),
    Func(String),
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

pub fn scan_logic_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag(LT_E_SIGN),
        tag(GT_E_SIGN),
        tag(NOT_EQ_SIGN),
        tag(EQ_SIGN),
        tag(LT_SIGN),
        tag(GT_SIGN),
        tag(PARENS_0),
        tag(PARENS_1),
        tag(COMMA_SIGN),
        tag(SEMICOLON_SIGN),
        tag_is_not,
        tag_key_word_logic(OR_SIGN),
        tag_key_word_logic(AND_SIGN),
        tag_key_word_logic(NOT_SIGN),
        tag_key_word_logic(LIKE_SIGN),
        tag_no_case(IS_SIGN),
        tag_no_case(NULL_SIGN),
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}
pub fn scan_binop_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag(MINUS_SIGN),
        tag(ADD_SIGN),
        tag(MUL_SIGN),
        tag(DIV_SIGN),
        tag(POWER_SIGN),
        tag(MOD_SIGN),
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}
pub fn scan_function(input: &str) -> IResult<&str, Token> {
    let a = tag_function.parse(input)?;
    Ok((a.0, Token::Func(a.1)))
}

///forcement en dernier !!!!!!
pub fn default_token(input: &str) -> IResult<&str, Token> {
    let a = space0(input)?;
    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        scan_function,
        scan_float,
        scan_logic_token,
        scan_binop_token,
        scan_name,
        scan_string,
        default_token,
    ))
    .parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}
