use crate::{general_struct::element::{CompareOp, Condition, LogicalOp}};
use crate::general_const::*;
use std::str::FromStr;
pub trait BuildCondition {
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
#[derive(Debug, thiserror::Error)]
#[error("Invalid token compare-op {0}")]
pub struct FromStrCmpOpError(String);


impl FromStr for CompareOp {
    type Err =FromStrCmpOpError;
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
            _ => Err(FromStrCmpOpError(s.into())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid token compare-op {0}")]
pub struct FromStrLogicalOpError(String);

impl FromStr for LogicalOp {
    type Err = FromStrLogicalOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            OR_SIGN => Ok(Self::Or),
            AND_SIGN => Ok(Self::And),
            _ => Err(FromStrLogicalOpError(s.into())),
        }
    }
}

