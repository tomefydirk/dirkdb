// tokentools.rs
use nom::branch::alt;
use nom::error::Error;
use nom::{IResult, Parser};

use crate::general_struct::PrimitiveElement;

use crate::logic_parser::cond_constant::NULL_SIGN;
use crate::logic_parser::cond_source::Condition;
use crate::tokenizer::{scan_float, scan_name, scan_other, scan_string, tag_binop_token, Token};



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
