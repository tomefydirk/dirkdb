use std::str::FromStr;

use nom::IResult;

use crate::{
    atom_parser::func::parse_expr, general_const::*, general_struct::element::{CompareOp, Condition, LogicalOp}, logic_parser::element::{error_builder, BuildCondition}, tokenizer::{scan_token, Token}
};

pub fn parse_logical(input: &str) -> IResult<&str, Box<Condition>> {
    let (mut input_rem, mut current) = parse_compare(input)?;

    while !input_rem.is_empty() && !input_rem.starts_with(PARENS_1) {
        let (next, token) = scan_token(input_rem)?;
        match token {
            Token::Other(a) if LogicalOp::from_str(a).is_ok() => {
                let (after_rhs, rhs) = parse_compare(next)?;
                current = LogicalOp::build(current, rhs, a.parse().unwrap());
                input_rem = after_rhs;
            }
            _ => break,
        }
    }
    Ok((input_rem, current))
}

pub fn parse_compare(input: &str) -> IResult<&str, Box<Condition>> {
    let (mut new_input, mut current) = parse_expr(input)?;
    let mut comparisons = Vec::<Condition>::new();

    while !new_input.trim().is_empty() && !new_input.trim().starts_with(PARENS_1) {
        let (next, token) = scan_token(new_input)?;
        match token {
            Token::Other(op) if CompareOp::from_str(op).is_ok() => {
                let cop: CompareOp = op.parse().unwrap();

                if matches!(cop, CompareOp::Is | CompareOp::IsNot) {
                    let (after_rhs, rhs_token) = scan_token(next)?;
                    match rhs_token {
                        Token::Other(r) if r.eq_ignore_ascii_case(NULL_SIGN) => {
                            let rhs = Box::new(Condition::Null);
                            current = CompareOp::build(current, rhs, cop);
                            comparisons.push(*current.clone());
                            new_input = after_rhs;
                        }
                        _ => {
                            return error_builder(next);
                        }
                    }
                } else {
                    let (after_rhs, rhs) = parse_expr(next)?;
                    current = CompareOp::build(current, rhs, cop);
                    comparisons.push(*current.clone());
                    new_input = after_rhs;
                }
            }
            _ => break,
        }
    }

    if comparisons.is_empty() {
        Ok((new_input, current))
    } else {
        and_ification(comparisons, new_input)
    }
}
pub fn and_ification(
    mut comparisons: Vec<Condition>,
    input: &str,
) -> IResult<&str, Box<Condition>> {
    match comparisons.len() {
        0 => error_builder(input),
        1 => Ok((input, Box::new(comparisons.pop().unwrap()))),
        _ => {
            let mut current = Box::new(comparisons.remove(0));
            for cmp in comparisons {
                current = LogicalOp::build(current, Box::new(cmp), LogicalOp::And);
            }
            Ok((input, current))
        }
    }
}

