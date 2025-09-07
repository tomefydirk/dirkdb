pub mod helper;
pub mod tag_func;
use std::fmt::Display;

use crate::IResult;
use crate::general_struct::constant::*;
use crate::general_struct::structure::{ManyKeyWord, QualifiedIdentifier};
use crate::tokenizer::helper::is_func_valid;
use crate::tokenizer::tag_func::{
    tag_float, tag_is_not, tag_key_word_logic, tag_string, tag_variable,
};
use nom::Parser;
///TOKENTOOL::
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;

#[derive(Debug)]
pub enum Token<'a> {
    Number(f64),
    String(String),
    Variable(QualifiedIdentifier),
    Func(QualifiedIdentifier),
    Mkw(ManyKeyWord<&'a str>),
    Other(&'a str),
}

impl<'a> From<&'a str> for Token<'a> {
    fn from(value: &'a str) -> Self {
        Token::Other(value)
    }
}
impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Number(n) => write!(f, "\"{}\"", n),
            Token::String(s) => write!(f, "\"{}\"", s),
            Token::Variable(qualified_identifier) => {
                write!(f, "{}", qualified_identifier)
            }
            Token::Func(qualified_identifier) => write!(f, "{}", qualified_identifier),
            Token::Other(t) => write!(f, "{t}"),
            Token::Mkw(many_key_word) =>write!(f, "{many_key_word:?}"),
        }
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
    let a = tag_variable(input)?;

    Ok((a.0, Token::Variable(a.1)))
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
        tag_key_word_logic(IS_SIGN),
        tag_key_word_logic(NULL_SIGN),
        tag_key_word_logic(AS_SIGN),
        tag_key_word_logic(SELECT_SIGN),
        tag_key_word_logic(FROM_SIGN),
        tag_key_word_logic(WHERE_SIGN),
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
    let (input, func_name) = tag_variable(input)?;
    let (input, token) = scan_token(input.trim())?;
    match token {
        Token::Other(PARENS_0) if is_func_valid(&func_name.name) => {
            Ok((input, Token::Func(func_name)))
        }
        _ => Err(nom::Err::Error(
            nom::error::Error::new(input, nom::error::ErrorKind::Tag).into(),
        )),
    }
}

///forcement en dernier !!!!!!
pub fn default_token(input: &str) -> IResult<&str, Token> {
    let a = space0(input)?;
    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        scan_logic_token,
        scan_function,
        scan_float,
        scan_binop_token,
        scan_name,
        scan_string,
        default_token,
    ))
    .parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}
