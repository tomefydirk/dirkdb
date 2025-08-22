use std::collections::HashMap;

use crate::{
    evaluation::utils::Comparator,
    function::helper::my_modulo,
    general_struct::element::{
        BinOp, CompareOp, Condition, LogicResult, LogicalOp, PrimitiveElement, TableCell,
    },
};

pub(crate) type Result<T, E = crate::error_lib::evaluation::EvalEror<String>> =
    std::result::Result<T, E>;

impl LogicalOp {
    pub fn default_apply(&self, l: bool, r: bool) -> bool {
        match self {
            LogicalOp::And => l && r,
            LogicalOp::Or => l || r,
        }
    }
}

impl CompareOp {
    pub fn default_apply(&self, left: &TableCell, right: &TableCell) -> bool {
        match (left, right) {
            (TableCell::Number(l), TableCell::Number(r)) => self.comparing(*l, *r),
            (TableCell::String(l), TableCell::String(r)) => self.comparing(l, r),
            (a, TableCell::Null) => match (self, a) {
                (CompareOp::Is, TableCell::Null) => true,
                (CompareOp::IsNot, b) if *b != TableCell::Null => true,
                _ => false,
            },
            _ => false,
        }
    }
}

impl BinOp {
    pub fn default_apply(&self, left: f64, right: f64) -> f64 {
        match self {
            BinOp::Add => left + right,
            BinOp::Sub => left - right,
            BinOp::Mul => left * right,
            BinOp::Div => left / right,
            BinOp::Pow => left.powf(right),
            BinOp::Mod => my_modulo(left, right),
        }
    }
}

impl Condition {
    fn eval_value(&self, ctx: &HashMap<String, TableCell>) -> Result<TableCell> {
        match self {
            Condition::Primitive(PrimitiveElement::Identifier(name)) => {
                ctx.get(name)
                    .cloned()
                    .ok_or_else(|| crate::error_lib::evaluation::EvalEror::FieldNotFound(
                        crate::error_lib::evaluation::FieldNotFoundErr(name.clone())
                    ))
            }
            Condition::Primitive(PrimitiveElement::Number(n)) => Ok(TableCell::Number(*n)),
            Condition::Primitive(PrimitiveElement::String(s)) => {
                Ok(TableCell::String(s.clone()))
            }
            Condition::Null => Ok(TableCell::Null),
            a => Ok(a.eval(ctx)?.into()),
        }
    }
}

impl Condition {
    pub fn eval(&self, ctx: &HashMap<String, TableCell>) -> Result<LogicResult> {
        match self {
            // Comparaison
            Condition::Comparison { left, op, right } => {
                let l = left.eval_value(ctx)?;
                let r = right.eval_value(ctx)?;
                Ok(LogicResult::Boolean(op.default_apply(&l, &r)))
            }

            // Logique (AND / OR)
            Condition::Logical { left, op, right } => {
                let l: bool = left.eval(ctx)?.into();
                let r: bool = right.eval(ctx)?.into();
                Ok(LogicResult::Boolean(op.default_apply(l, r)))
            }

            // Opérateurs binaires arithmétiques
            Condition::BinaryOp { left, op, right } => {
                let l = left.eval(ctx)?.as_number();
                let r = right.eval(ctx)?.as_number();
                Ok(match (l, r) {
                    (Some(a), Some(b)) => LogicResult::Other(TableCell::Number(op.default_apply(a, b))),
                    _ => LogicResult::Other(TableCell::Null),
                })
            }

            // Négation arithmétique (-x)
            Condition::Negate(inner) => match inner.eval(ctx)?.as_number() {
                Some(n) => Ok(LogicResult::Other(TableCell::Number(-n))),
                None => Ok(LogicResult::Other(TableCell::Null)),
            },

            // NOT logique
            Condition::Not(inner) => {
                let val: bool = inner.eval(ctx)?.into();
                Ok(LogicResult::Boolean(!val))
            }

            // Valeurs primitives
            Condition::Primitive(_) | Condition::Null => {
                Ok(LogicResult::Other(self.eval_value(ctx)?))
            }
        }
    }
}

impl LogicResult {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            LogicResult::Other(TableCell::Number(n)) => Some(*n),
            LogicResult::Other(TableCell::String(_)) => Some(0.0),
            LogicResult::Boolean(b) => Some(crate::evaluation::utils::bool_transform(*b)),
            _ => None,
        }
    }
}
