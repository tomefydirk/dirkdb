use std::collections::HashMap;

use crate::{evaluation::utils::Comparator, general_struct::element::{BinOp, CompareOp, Condition, LogicResult, LogicalOp, PrimitiveElement, TableCell}};

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

impl BinOp{
    pub fn default_apply(&self,left:f64,right:f64)->f64{
        match self {
            BinOp::Add =>left+right,
            BinOp::Sub => left-right,
            BinOp::Mul => left+right,
            BinOp::Div => left/right,
            BinOp::Pow => left.powf(right),
            BinOp::Mod =>( (left / right ) as u64 ) as f64,
        }
    }
}

impl Condition {
    pub fn eval(&self, ctx: &HashMap<String, TableCell>) -> LogicResult {
       todo!()
    }
      fn eval_value(&self, ctx: &HashMap<String, TableCell>) -> Option<TableCell> {
        match self {
            Condition::Primitive(PrimitiveElement::Identifier(name)) => ctx.get(name).cloned(),
            Condition::Primitive(PrimitiveElement::Number(n)) => Some(TableCell::Number(*n)),
             Condition::Primitive(PrimitiveElement::String(s)) => Some(TableCell::String(s.clone())),
            Condition::Null => Some(TableCell::Null),
            _ => None,
        }
    }
}
