// tokentools.rs
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case};
use nom::character::complete::{multispace1, space0};
use nom::error::Error;
use nom::{IResult, Parser};

use crate::atom_parser::expr_constant::{ADD_SIGN, DIV_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN};
use crate::general_const::{PARENS_0, PARENS_1};
use crate::general_struct::PrimitiveElement;
use crate::logic_parser::cond_constant::{
    AND_SIGN, EQ_SIGN, GT_E_SIGN, GT_SIGN, IS_NOT_SIGN, IS_SIGN, LT_E_SIGN, LT_SIGN, NOT_EQ_SIGN,
    NOT_SIGN, NULL_SIGN, OR_SIGN,
};
use crate::logic_parser::cond_source::Condition;
use crate::tokenizer::{Token, scan_float, scan_name, scan_string};

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
pub fn scan_other(input: &str) -> IResult<&str, Token> {
    let a = alt((
        tag(LT_E_SIGN),
        tag(GT_E_SIGN),
        tag(NOT_EQ_SIGN),
        tag(EQ_SIGN),
        tag(LT_SIGN),
        tag(GT_SIGN),
        tag(PARENS_0),
        tag(PARENS_1),
        tag_is_not,
        tag_no_case(OR_SIGN),
        tag_no_case(AND_SIGN),
        tag_no_case(IS_SIGN),
        tag_no_case(NOT_SIGN),
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
        space0,
    ))
    .parse(input)?;

    Ok((a.0, Token::Other(a.1)))
}

pub fn scan_token(input: &str) -> IResult<&str, Token> {
    println!("input scan_token :'{input}'");
    let a = alt((scan_float, scan_other, scan_name, scan_string,tag_binop_token)).parse(input.trim())?;
    Ok((a.0.trim(), a.1))
}

impl Token<'_> {
    pub fn to_condition<'a>(&self, input: &'a str) -> IResult<&'a str, Box<Condition>> {
        match self {
            Token::Number(n) => {
                let val: PrimitiveElement = PrimitiveElement::from(*n);
                Ok((input, Box::new(Condition::Primitive(val))))
            }
            Token::String(f) => {
                let val: PrimitiveElement = PrimitiveElement::from(f.clone());
                Ok((input, Box::new(Condition::Primitive(val))))
            }
            Token::FieldName(a) => {
                let val: PrimitiveElement = PrimitiveElement::from_id(a.clone());
                Ok((input, Box::new(Condition::Primitive(val))))
            }
            Token::Other(a) if a.eq_ignore_ascii_case(NULL_SIGN) => {
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
