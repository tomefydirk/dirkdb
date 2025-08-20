use crate::{
    atom_parser::{expr_function::parse_expr, expr_struct::BinOp},
    general_const::{ PARENS_1},
    general_struct::PrimitiveElement,
    logic_parser::{
        cond_constant::{
            AND_SIGN, EQ_SIGN, GT_E_SIGN, GT_SIGN, IS_NOT_SIGN, IS_SIGN, LT_E_SIGN, LT_SIGN,
            NOT_EQ_SIGN,  NULL_SIGN, OR_SIGN,
        },
        tokentool::*,
    },
    tokenizer::Token,
};
use nom::{
    IResult,
    error::{Error, ErrorKind},
};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Condition {
    Comparison {
        left: Box<Condition>,
        op: CompareOp,
        right: Box<Condition>,
    },
    Logical {
        left: Box<Condition>,
        op: LogicalOp,
        right: Box<Condition>,
    },
    BinaryOp {
        left: Box<Condition>,
        op: BinOp,
        right: Box<Condition>,
    },
    Negate(Box<Condition>),
    Not(Box<Condition>),
    Primitive(PrimitiveElement),
    Null,
}

impl Default for Condition {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Debug, Clone)]
pub enum CompareOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Is,
    IsNot,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

trait BuildCondition {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition>
    where
        Self: Sized;
}

impl BuildCondition for CompareOp {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition> {
        Box::new(Condition::Comparison { left, op, right })
    }
}
impl BuildCondition for LogicalOp {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition> {
        Box::new(Condition::Logical { left, op, right })
    }
}

impl FromStr for CompareOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            EQ_SIGN => Ok(Self::Eq),
            NOT_EQ_SIGN => Ok(Self::Neq),
            LT_SIGN => Ok(Self::Lt),
            LT_E_SIGN => Ok(Self::Lte),
            GT_SIGN => Ok(Self::Gt),
            GT_E_SIGN => Ok(Self::Gte),
            IS_SIGN => Ok(Self::Is),
            IS_NOT_SIGN => Ok(Self::IsNot),
            _ => Err(()),
        }
    }
}

impl FromStr for LogicalOp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            OR_SIGN => Ok(Self::Or),
            AND_SIGN => Ok(Self::And),
            _ => Err(()),
        }
    }
}

fn err(input: &str) -> IResult<&str, Box<Condition>> {
    Err(nom::Err::Error(Error::new(input, ErrorKind::Digit)))
}

pub fn parse_logical(input: &str) -> IResult<&str, Box<Condition>> {
    println!("LOGICAL {input}");
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
    let (mut new_input, mut current) = parse_atom(input)?;
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
                            return err(next);
                        }
                    }
                } else {
                    let (after_rhs, rhs) = parse_atom(next)?;
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

pub fn parse_atom(input: &str) -> IResult<&str, Box<Condition>> {
    parse_expr(input)
}

pub fn and_ification(
    mut comparisons: Vec<Condition>,
    input: &str,
) -> IResult<&str, Box<Condition>> {
    match comparisons.len() {
        0 => err(input),
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

