pub mod function;
pub mod helper;

use crate::tokenizer::function::scan_function;
use crate::IResult;
use crate::general_const::{
    ADD_SIGN, AND_SIGN, COMMA_SIGN, DIV_SIGN, EQ_SIGN, GT_E_SIGN, GT_SIGN, IS_NOT_SIGN, IS_SIGN, LIKE_SIGN, LT_E_SIGN, LT_SIGN, MINUS_SIGN, MOD_SIGN, MUL_SIGN, NOT_EQ_SIGN, NOT_SIGN, NULL_SIGN, OR_SIGN, POWER_SIGN, SEMICOLON_SIGN
};
use crate::general_const::{PARENS_0, PARENS_1};
use crate::tokenizer::helper::{is_ident_char, is_ident_start};
///TOKENTOOL::
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{digit1, multispace1, space0};
use nom::combinator::opt;
use nom::{Parser, bytes::complete::take_while1};

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

pub fn tag_float(input: &str) -> IResult<&str, f64> {
    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((
            rest3,
            format!("{first_part}.{second_part}").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit).into())
            })?,
        ))
    } else {
        Ok((
            rest,
            format!("{first_part}.0").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit).into())
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
        tag_no_case(IS_SIGN),
        multispace1,
        tag_no_case(NOT_SIGN),
        multispace1,
    )
        .parse(input)?;
    Ok((input, (IS_NOT_SIGN)))
}

pub fn tag_key_word_logic<'a>(
    keyword: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (new_input, matched) = tag_no_case(keyword).parse(input)?;

        if new_input.trim_start().starts_with('(') {
            Ok((new_input, matched))
        } else {
            let (new_input, _) = multispace1(new_input)?;
            Ok((new_input, matched))
        }
    }
}

pub fn tag_logic_token(input: &str) -> IResult<&str, Token> {
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
pub fn tag_binop_token(input: &str) -> IResult<&str, Token> {
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
//forcement en dernier !!!!!!
pub fn default_token(input: &str) -> IResult<&str, Token> {
    let a = space0(input)?;
    Ok((a.0, Token::Other(a.1)))
}
pub fn scan_token(input: &str) -> IResult<&str, Token> {
    let a = alt((
        scan_function,
        scan_float,
        tag_logic_token,
        tag_binop_token,
        scan_name,
        scan_string,
        default_token,
    ))
    .parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}
