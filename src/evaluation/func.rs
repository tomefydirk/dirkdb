use std::collections::HashMap;

use crate::{
    evaluation::utils::Comparator, function::helper::my_modulo, general_struct::element::{
        BinOp, CompareOp, Condition, LogicResult, LogicalOp, PrimitiveElement, TableCell,
    }
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
    pub fn default_apply(&self, left: &TableCell, right: &TableCell) -> bool {
        match (left, right) {
            (TableCell::Number(l), TableCell::Number(r)) => self.comparing(*l, *r),
            (TableCell::String(l), TableCell::String(r)) => self.comparing(l, r),
            (a, TableCell::Null) => match (self, a) {
                (CompareOp::Is, TableCell::Null) => true,
                (CompareOp::IsNot, b) if *b != TableCell::Null => true,
                _ => false,
            },

            //TODO LIKE!!!
            _ => false,
        }
    }
}

impl BinOp {
    pub fn default_apply(&self, left: f64, right: f64) -> f64 {
        match self {
            BinOp::Add => left + right,
            BinOp::Sub => left - right,
            BinOp::Mul => left + right,
            BinOp::Div => left / right,
            BinOp::Pow => left.powf(right),
            BinOp::Mod => my_modulo(left, right),
        }
    }
}

impl Condition {
    fn eval_value(&self, ctx: &HashMap<String, TableCell>) -> Option<TableCell> {
        match self {
            Condition::Primitive(PrimitiveElement::Identifier(name)) => ctx.get(name).cloned(),
            Condition::Primitive(PrimitiveElement::Number(n)) => Some(TableCell::Number(*n)),
            Condition::Primitive(PrimitiveElement::String(s)) => Some(TableCell::String(s.clone())),
            Condition::Null => Some(TableCell::Null),
            a => Some(a.eval(ctx).into()),
        }
    }
}

impl Condition {
    pub fn eval(&self, ctx: &HashMap<String, TableCell>) -> LogicResult {
        match self {
            // --- Comparaison ---
            Condition::Comparison { left, op, right } => {
                let l = left.eval_value(ctx).unwrap_or_default();

                let r = right.eval_value(ctx).unwrap_or_default();

                LogicResult::Boolean(op.default_apply(&l, &r))
            }

            // --- Logique (AND / OR) ---
            Condition::Logical { left, op, right } => {
                let l: bool = left.eval(ctx).into();
                let r: bool = right.eval(ctx).into();
                LogicResult::Boolean(op.default_apply(l, r))
            }

            // --- Opérateurs binaires arithmétiques ---
            Condition::BinaryOp { left, op, right } => {
                let l = left.eval(ctx).as_number();
                let r = right.eval(ctx).as_number();
                match (l, r) {
                    (Some(a), Some(b)) => {
                        LogicResult::Other(TableCell::Number(op.default_apply(a, b)))
                    }
                    _ => LogicResult::Other(TableCell::Null),
                }
            }

            // --- Négation arithmétique (-x) ---
            Condition::Negate(inner) => match inner.eval(ctx).as_number() {
                Some(n) => LogicResult::Other(TableCell::Number(-n)),
                None => LogicResult::Other(TableCell::Null),
            },

            // --- NOT logique ---
            Condition::Not(inner) => {
                let val: bool = inner.eval(ctx).into();
                LogicResult::Boolean(!val)
            }

            // --- Valeurs primitives (identifiants, nombres, chaînes, NULL) ---
            Condition::Primitive(_) | Condition::Null => self
                .eval_value(ctx)
                .map_or(LogicResult::Other(TableCell::Null), LogicResult::Other),
        }
    }
}

impl LogicResult {
    /// Extraire un nombre (bool → 0/1, string → None)
    pub fn as_number(&self) -> Option<f64> {
        match self {
            LogicResult::Other(TableCell::Number(n)) => Some(*n),
            LogicResult::Other(TableCell::String(_)) => Some(0.0),
            LogicResult::Boolean(b) => Some(crate::evaluation::utils::bool_transform(*b)),
            _ => None,
        }
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::general_struct::element::{
        BinOp, CompareOp, Condition, LogicResult, LogicalOp, PrimitiveElement, TableCell,
    };

    fn ctx() -> HashMap<String, TableCell> {
        let mut map = HashMap::new();
        map.insert("x".into(), TableCell::Number(10.0));
        map.insert("y".into(), TableCell::Number(5.0));
        map.insert("name".into(), TableCell::String("rust".into()));
        map.insert("empty".into(), TableCell::Null);
        map
    }

    #[test]
    fn test_comparisons() {
        let c = Condition::Comparison {
            left: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "x".into(),
            ))),
            op: CompareOp::Gt,
            right: Box::new(Condition::Primitive(PrimitiveElement::Number(3.0))),
        };
        assert_eq!(c.eval(&ctx()), LogicResult::Boolean(true));

        let c = Condition::Comparison {
            left: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "empty".into(),
            ))),
            op: CompareOp::Is,
            right: Box::new(Condition::Null),
        };
        assert_eq!(c.eval(&ctx()), LogicResult::Boolean(true));
    }

    #[test]
    fn test_logical() {
        let left = Condition::Comparison {
            left: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "x".into(),
            ))),
            op: CompareOp::Gt,
            right: Box::new(Condition::Primitive(PrimitiveElement::Number(3.0))),
        };
        let right = Condition::Comparison {
            left: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "y".into(),
            ))),
            op: CompareOp::Lt,
            right: Box::new(Condition::Primitive(PrimitiveElement::Number(2.0))),
        };
        let cond = Condition::Logical {
            left: Box::new(left),
            op: LogicalOp::And,
            right: Box::new(right),
        };
        assert_eq!(cond.eval(&ctx()), LogicResult::Boolean(false));
    }

    #[test]
    fn test_binary_ops() {
        let c = Condition::BinaryOp {
            left: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "x".into(),
            ))),
            op: BinOp::Add,
            right: Box::new(Condition::Primitive(PrimitiveElement::Identifier(
                "y".into(),
            ))),
        };
        assert_eq!(c.eval(&ctx()), LogicResult::Other(TableCell::Number(15.0)));

        let c = Condition::BinaryOp {
            left: Box::new(Condition::Primitive(PrimitiveElement::Number(1.0))),
            op: BinOp::Add,
            right: Box::new(Condition::Primitive(PrimitiveElement::Number(2.0))),
        };
        assert_eq!(c.eval(&ctx()), LogicResult::Other(TableCell::Number(3.0)));
    }

    #[test]
    fn test_negate_and_not() {
        let negate = Condition::Negate(Box::new(Condition::Primitive(PrimitiveElement::Number(
            5.0,
        ))));
        assert_eq!(
            negate.eval(&ctx()),
            LogicResult::Other(TableCell::Number(-5.0))
        );

        let not = Condition::Not(Box::new(Condition::Primitive(PrimitiveElement::Number(
            0.0,
        ))));
        assert_eq!(not.eval(&ctx()), LogicResult::Boolean(true));
    }

    #[test]
    fn test_primitives() {
        let c = Condition::Primitive(PrimitiveElement::Identifier("name".into()));
        assert_eq!(
            c.eval(&ctx()),
            LogicResult::Other(TableCell::String("rust".into()))
        );

        let c = Condition::Primitive(PrimitiveElement::Number(42.0));
        assert_eq!(c.eval(&ctx()), LogicResult::Other(TableCell::Number(42.0)));

        let c = Condition::Null;
        assert_eq!(c.eval(&ctx()), LogicResult::Other(TableCell::Null));
    }
}
