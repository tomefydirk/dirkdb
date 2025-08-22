use std::collections::HashMap;

use crate::{
    error_lib::evaluation::{EvalEror},
    evaluation::{LgResult, utils::Comparator},
    function::{self, helper::my_modulo},
    general_struct::element::{
        BinOp, CompareOp, Condition, EvalElement, LogicalOp, PrimitiveElement, TableCell,
    },
};

impl LogicalOp {
    pub fn default_apply(&self, l: bool, r: bool) -> bool {
        match self {
            LogicalOp::And => l && r,
            LogicalOp::Or => l || r,
        }
    }
}

impl CompareOp {
    pub fn default_apply(&self, left: &TableCell, right: &TableCell) -> LgResult<bool> {
        match self {
            CompareOp::Like => {
                let l_str = left.to_string_value();
                let r_str = right.to_string_value();
                let re = function::helper::convert_sql_to_regex(&r_str)?;
                Ok(re.is_match(&l_str))
            }
            _ => match (left, right) {
                (TableCell::Number(l), TableCell::Number(r)) => Ok(self.comparing(*l, *r)),
                (TableCell::String(l), TableCell::String(r)) => Ok(self.comparing(l, r)),
                (TableCell::Date(d), t) => Ok(self.comparing(d, &t.convert_to_date()?)),
                (t, TableCell::Date(d)) => Ok(self.comparing(&t.convert_to_date()?, d)),
                (a, TableCell::Null) => match (self, a) {
                    (CompareOp::Is, TableCell::Null) => Ok(true),
                    (CompareOp::IsNot, b) if *b != TableCell::Null => Ok(true),
                    _ => Ok(false),
                },
                _ => Ok(false),
            },
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
    fn eval_value(&self, ctx: &HashMap<String, TableCell>) -> LgResult<TableCell> {
        match self {
            Condition::Primitive(PrimitiveElement::Identifier(name)) => ctx
                .get(name)
                .cloned()
                .ok_or_else(|| EvalEror::field_notfound(name.to_string())),
            Condition::Primitive(PrimitiveElement::Number(n)) => Ok(TableCell::Number(*n)),
            Condition::Primitive(PrimitiveElement::String(s)) => Ok(TableCell::String(s.clone())),
            Condition::Null => Ok(TableCell::Null),
            a => Ok(a.eval(ctx)?.into()),
        }
    }
}

impl Condition {
    pub fn eval(&self, ctx: &HashMap<String, TableCell>) -> LgResult<EvalElement> {
        match self {
            // Comparaison
            Condition::Comparison { left, op, right } => {
                let l = left.eval_value(ctx)?;
                let r = right.eval_value(ctx)?;
                Ok(EvalElement::Boolean(op.default_apply(&l, &r)?))
            }

            // Logique (AND / OR)
            Condition::Logical { left, op, right } => {
                let l: bool = left.eval(ctx)?.into();
                let r: bool = right.eval(ctx)?.into();
                Ok(EvalElement::Boolean(op.default_apply(l, r)))
            }

            // Opérateurs binaires arithmétiques
            Condition::BinaryOp { left, op, right } => {
                let l = left.eval(ctx)?.as_number();
                let r = right.eval(ctx)?.as_number();
                Ok(match (l, r) {
                    (Some(a), Some(b)) => {
                        EvalElement::Other(TableCell::Number(op.default_apply(a, b)))
                    }
                    _ => EvalElement::Other(TableCell::Null),
                })
            }

            // Négation arithmétique (-x)
            Condition::Negate(inner) => match inner.eval(ctx)?.as_number() {
                Some(n) => Ok(EvalElement::Other(TableCell::Number(-n))),
                None => Ok(EvalElement::Other(TableCell::Null)),
            },

            // NOT logique
            Condition::Not(inner) => {
                let val: bool = inner.eval(ctx)?.into();
                Ok(EvalElement::Boolean(!val))
            }

            // Valeurs primitives
            Condition::Primitive(_) | Condition::Null => {
                Ok(EvalElement::Other(self.eval_value(ctx)?))
            }
        }
    }
}

impl EvalElement {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            EvalElement::Other(TableCell::Number(n)) => Some(*n),
            EvalElement::Other(TableCell::String(_)) => Some(0.0),
            EvalElement::Other(TableCell::Date(d))=>{
                //CHANGER d en jours 
                todo!()
            },
            EvalElement::Boolean(b) => Some(crate::evaluation::utils::bool_transform(*b)),
            _ => None,
        }
    }
}
