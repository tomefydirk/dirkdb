use std::str::FromStr;

use crate::IResult;
use crate::error_lib::parsing::{factor_error, into_nom_error, into_nom_failure, token_not_found};
use crate::general_struct::structure::{BinOp, Condition};
use crate::parsing::other_parser::logic_parser::func::parse_logical;
use crate::tokenizer::helper::codon_stop;
use crate::tokenizer::{Token, scan_token};

use crate::general_struct::constant::*;

fn parse_binop_level<'a, F>(
    input: &'a str,
    lower_parser: F,
    ops: &[&str],
) -> IResult<&'a str, Box<Condition>>
where
    F: Fn(&'a str) -> IResult<&'a str, Box<Condition>>,
{
    let (mut input_rem, mut current_expr) = lower_parser(input)?;

    if codon_stop(input_rem) {
        return Condition::result_from_current(input_rem, current_expr);
    }

    loop {
        let (next_input, token) = scan_token(input_rem)?;

        match token {
            Token::Other(op) if ops.contains(&op) => {
                let (after_rhs, rhs) = lower_parser(next_input)?;

                current_expr = Condition::box_binop_from(
                    current_expr,
                    rhs,
                    BinOp::from_str(op).map_err(into_nom_failure)?,
                );
                input_rem = after_rhs;
            }
            _ => return Condition::result_from_current(input_rem, current_expr),
        }
    }
}

pub fn parse_atom(input: &str) -> IResult<&str, Box<Condition>> {
    parse_binop_level(input, parse_mod, &[ADD_SIGN, MINUS_SIGN])
}
pub fn parse_mod(input: &str) -> IResult<&str, Box<Condition>> {
    parse_binop_level(input, parse_term, &[MOD_SIGN])
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
        Token::String(s) => Condition::result_string(next_input, s),
        Token::Variable(f) => Condition::result_name(next_input, f),
        Token::Other(str_token) => {
            if str_token == PARENS_0 {
                parse_real_factor(next_input)
            } else if Condition::is_factor_op(str_token) {
                let (after, real_perm) = parse_factor(next_input)?;
                Ok((after, Condition::box_factorop_from(real_perm, str_token)))
            } else if str_token.to_lowercase() == NULL_SIGN {
                Ok((next_input, Box::new(Condition::Null)))
            } else {
                Err(into_nom_error(token_not_found(input)))
            }
        }
        Token::Func(f) => parse_func_factor(next_input, f),
    }
}
pub fn parse_func_factor(mut input: &str, f: String) -> IResult<&str, Box<Condition>> {
    let mut vec = Vec::<Condition>::new();
    if input.trim().starts_with(PARENS_1) {
        let (ipt, _) = scan_token(input)?;
        return Condition::result_func(ipt, f, vec);
    }
    while !codon_stop(input) {
        let a = parse_logical(input)?;
        let (ipt, token) = scan_token(a.0)?;
        match token {
            Token::Other(COMMA_SIGN) => vec.push(*a.1),
            Token::Other(PARENS_1) => {
                vec.push(*a.1);
                return Condition::result_func(ipt, f, vec);
            }
            _ => break,
        }
        input = ipt;
    }
    Err(into_nom_error(token_not_found(input)))
}
pub fn parse_real_factor(input: &str) -> IResult<&str, Box<Condition>> {
    let (after_expr, expr) = parse_logical(input)?;
    let (after_paren, token) = scan_token(after_expr)?;

    match token {
        Token::Other(PARENS_1) => Condition::result_from_current(after_paren, expr),
        _ => Err(into_nom_failure(factor_error(input))),
    }
}
