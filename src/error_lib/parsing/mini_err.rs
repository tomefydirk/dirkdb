use std::str::FromStr;

use crate::{
    general_const::*,
    general_struct::element::{BinOp, CompareOp, LogicalOp},
};

#[derive(Debug, thiserror::Error)]
#[error("Invalid token bin-op {0}")]
pub struct FromStrBinOpError(String);

impl FromStr for BinOp {
    type Err = FromStrBinOpError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            ADD_SIGN => BinOp::Add,
            MINUS_SIGN => BinOp::Sub,
            MUL_SIGN => BinOp::Mul,
            DIV_SIGN => BinOp::Div,
            POWER_SIGN => BinOp::Pow,
            MOD_SIGN => BinOp::Mod,
            _ => return Err(FromStrBinOpError(s.into())),
        };
        Ok(op)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid token compare-op {0}")]
pub struct FromStrCmpOpError(String);

impl FromStr for CompareOp {
    type Err = FromStrCmpOpError;
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
            LIKE_SIGN => Ok(Self::Like),
            _ => Err(FromStrCmpOpError(s.into())),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Invalid token logical-op {0}")]
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
