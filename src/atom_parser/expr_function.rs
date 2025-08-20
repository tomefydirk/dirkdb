use crate::atom_parser::expr_struct::{BinOp};
use crate::atom_parser::tokentool::{ scan_token};
use crate::general_const::{PARENS_0, PARENS_1};
use crate::general_struct::PrimitiveElement;
use crate::logic_parser::cond_constant::TERMINALS;
use crate::logic_parser::cond_source::{parse_logical, Condition};
use crate::tokenizer::{is_terminal, Token};
use nom::IResult;
use nom::error::Error;
/*
rhs:Right hand side
lhs:Left hand side
*/
use crate::atom_parser::expr_constant::{ADD_SIGN, DIV_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN};

fn parse_binop_level<'a, F>(
    input: &'a str,
    lower_parser: F,
    ops: &[&str],
) -> IResult<&'a str, Box<Condition>>
where
    F: Fn(&'a str) -> IResult<&'a str, Box<Condition>>,
{
    let (mut input_rem, mut current_expr) = lower_parser(input)?;

    if input_rem.starts_with(')') {
        return Condition::result_from_current(input_rem, current_expr);
    }

    loop {
        let (next_input, token) = scan_token(input_rem)?;

        match token {
            Token::Other(op) if ops.contains(&op) => {
                let (after_rhs, rhs) = lower_parser(next_input)?;
                current_expr = Condition::box_binop_from(current_expr, rhs, BinOp::from_str(op));
                input_rem = after_rhs;
            },
            _ => return Condition::result_from_current(input_rem, current_expr),
        }
    }
}

pub fn parse_expr(input: &str) -> IResult<&str, Box<Condition>> {
    parse_binop_level(input, parse_term, &[ADD_SIGN, MINUS_SIGN])
}

pub fn parse_term(input: &str) -> IResult<&str, Box<Condition>> {
    parse_binop_level(input, parse_power, &[MUL_SIGN, DIV_SIGN])
}

pub fn parse_power(input: &str) -> IResult<&str, Box<Condition>> {
    parse_binop_level(input, parse_factor, &[POWER_SIGN])
}

pub fn parse_factor(input: &str) -> IResult<&str, Box<Condition>> {
    let (next_input, token) = scan_token(input)?;
    match token {
        Token::Number(n) => Condition::result_number(next_input, n),
        Token::String(s)=>Condition::result_string(next_input, s),
        Token::FieldName(f)=>Condition::result_name(input,f),
        Token::Other(str_token) => {
            if str_token == PARENS_0 {
                parse_real_factor(next_input)
            } else if Condition::is_factor_op(str_token) {
                 let (after, real_perm) = parse_logical(next_input)?;
                Ok((after, Condition::box_factorop_from(real_perm, str_token)))
            } else {
                Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
            }
        }
        _=>{
             Err(nom::Err::Error(Error::new(
                    input,
                    nom::error::ErrorKind::Digit,
                )))
        }
    }
}

pub fn parse_real_factor(input: &str) -> IResult<&str, Box<Condition>> {
    let (after_expr, expr) =parse_logical(input)?;
    let (after_paren, token) = scan_token(after_expr)?;

    match token {
        Token::Other(PARENS_1) => Condition::result_from_current(after_paren, expr),
        _ => Err(nom::Err::Error(Error::new(
            after_paren,
            nom::error::ErrorKind::Digit,
        ))),
    }
}
