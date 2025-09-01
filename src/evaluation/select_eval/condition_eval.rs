use std::collections::HashMap;

use crate::{
    evaluation::{
        EvaluableAsQuery, LgResult, OperatorQuery,
        helper::{Comparator, RowAlias},
    },
    function::{self, helper::my_modulo, sql::FunctionRegistry},
    general_struct::structure::{
        BinOp, CompareOp, Condition, LogicalOp, PrimitiveElement, QualifiedIdentifier,
        TableAliasMap, TableCell, TableRow,
    },
};
impl OperatorQuery<bool, bool> for LogicalOp {
    fn default_apply(&self, left: bool, right: bool) -> bool {
        match self {
            LogicalOp::And => left && right,
            LogicalOp::Or => left || right,
        }
    }
}
impl OperatorQuery<&TableCell, LgResult<bool>> for CompareOp {
    fn default_apply(&self, left: &TableCell, right: &TableCell) -> LgResult<bool> {
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

impl OperatorQuery<f64, f64> for BinOp {
    fn default_apply(&self, left: f64, right: f64) -> f64 {
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
    fn eval_value(&self, ctx: &TableRow, aliases: &TableAliasMap) -> LgResult<TableCell> {
        match self {
            Condition::Primitive(PrimitiveElement::Identifier(qid)) => {
                let a = ctx.get_column(qid, aliases)?;
                Ok(a.clone())
            }
            Condition::Primitive(PrimitiveElement::Number(n)) => Ok(TableCell::Number(*n)),
            Condition::Primitive(PrimitiveElement::String(s)) => Ok(TableCell::String(s.clone())),
            Condition::Null => Ok(TableCell::Null),
            a => Ok(a.eval_dyn(ctx, aliases)?),
        }
    }
}

impl EvaluableAsQuery<TableRow, TableAliasMap, TableCell> for Condition {
    fn eval_dyn(&self, ctx: &TableRow, aliases: &TableAliasMap) -> LgResult<TableCell> {
        match self {
            Condition::Comparison { left, op, right } => {
                let l = left.eval_value(ctx, aliases)?;
                let r = right.eval_value(ctx, aliases)?;

                let a = (op.default_apply(&l, &r)?).into();
                Ok(a)
            }
            Condition::Logical { left, op, right } => {
                let l: bool = left.eval_dyn(ctx, aliases)?.into();
                let r: bool = right.eval_dyn(ctx, aliases)?.into();
                Ok((op.default_apply(l, r)).into())
            }
            Condition::BinaryOp { left, op, right } => {
                let l = left.eval_dyn(ctx, aliases)?.as_number();
                let r = right.eval_dyn(ctx, aliases)?.as_number();
                Ok(match (l, r) {
                    (Some(a), Some(b)) => TableCell::Number(op.default_apply(a, b)),
                    _ => TableCell::Null,
                })
            }
            Condition::Negate(inner) => match inner.eval_dyn(ctx, aliases)?.as_number() {
                Some(n) => Ok(TableCell::Number(-n)),
                None => Ok(TableCell::Null),
            },
            Condition::Not(inner) => {
                let val: bool = inner.eval_dyn(ctx, aliases)?.into();
                Ok((!val).into())
            }
            Condition::Primitive(_) | Condition::Null => Ok(self.eval_value(ctx, aliases)?),

            /*
               ÉVALUATION STATIQUE POUR LE MOMENT :

               TODO!()
            */
            Condition::Func { name, parameter } => {
                let func_list = FunctionRegistry::new();

                func_list.call(name, change_args_type(parameter, ctx, aliases)?)
            }
        }
    }
    fn static_eval(&self) -> LgResult<TableCell> {
        let ctx = HashMap::<QualifiedIdentifier, TableCell>::new();
        self.eval_dyn(&ctx, &HashMap::new())
    }
}

pub fn change_args_type(
    args: &Vec<Condition>,
    ctx: &TableRow,
    aliases: &TableAliasMap,
) -> LgResult<Vec<TableCell>> {
    let mut retour = Vec::<TableCell>::new();
    for a in args {
        retour.push(a.eval_dyn(ctx, aliases)?);
    }
    Ok(retour)
}
