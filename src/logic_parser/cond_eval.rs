use crate::logic_parser::cond_source::{CompareOp, Condition, LogicalOp};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum TableCell {
    String(String),
    Number(f64),
    Null,
}

impl Default for TableCell {
    fn default() -> Self {
        Self::Null
    }
}
impl PartialEq for TableCell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}
impl Condition {
    pub fn evaluate(&self, ctx: &HashMap<String, TableCell>) -> bool {
        match self {
            Condition::Comparison { left, op, right } => {
                match (left.eval_value(ctx), right.eval_value(ctx)) {
                    (Some(l), Some(r)) => op.apply(&l, &r),
                    _ => false,
                }
            }
            Condition::Logical { left, op, right } => {
                op.apply(left.evaluate(ctx), right.evaluate(ctx))
            }
            Condition::Identifier(name) => ctx
                .get(name)
                .map(|s| match s {
                    TableCell::String(str) => !str.is_empty(),
                    TableCell::Number(n) => *n != 0.0,
                    TableCell::Null => false,
                })
                .unwrap_or(false),
            Condition::Number(n) => *n != 0.0,
            Condition::String(s) => !s.is_empty(),
            Condition::Null => false,
            Condition::Not(a) => !a.evaluate(ctx),
        }
    }
    fn eval_value(&self, ctx: &HashMap<String, TableCell>) -> Option<TableCell> {
        match self {
            Condition::Identifier(name) => ctx.get(name).cloned(),
            Condition::Number(n) => Some(TableCell::Number(*n)),
            Condition::String(s) => Some(TableCell::String(s.clone())),
            Condition::Null => Some(TableCell::Null),
            _ => None,
        }
    }
}

impl CompareOp {
    pub fn apply(&self, left: &TableCell, right: &TableCell) -> bool {
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
trait Comparator<T>
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool;
}

impl<T> Comparator<T> for CompareOp
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool {
        match self {
            CompareOp::Eq => l == r,
            CompareOp::Neq => l != r,
            CompareOp::Lt => l < r,
            CompareOp::Lte => l <= r,
            CompareOp::Gt => l > r,
            CompareOp::Gte => l >= r,
            _ => false,
        }
    }
}
impl LogicalOp {
    pub fn apply(&self, l: bool, r: bool) -> bool {
        match self {
            LogicalOp::And => l && r,
            LogicalOp::Or => l || r,
        }
    }
}
