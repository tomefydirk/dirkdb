pub mod condition_eval;
pub mod context;
pub mod join_helper;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{EvaluableAsQuery, JoinOpperand, EvalResult, select_eval::context::CtxSELECT},
    general_struct::structure::{
        Field, FieldRqst, QualifiedIdentifier, SelectRqst, Table, TableAliasMap, TableRow,
    },
};
use indexmap::IndexMap;

impl EvaluableAsQuery<Table, TableAliasMap, Table> for Vec<Field> {
    fn eval_dyn(&self, ctx: &Table, aliases: &TableAliasMap) -> EvalResult<Table> {
        let mut result: Table = Vec::new();

        for row in ctx {
            let mut new_row: TableRow = IndexMap::new();

            for field in self {
                let val = field.expr.eval_dyn(row, aliases)?;
                let qid = match &field.alias {
                    Some(alias) => {
                        QualifiedIdentifier::new(field.default_name.src.clone(), alias.clone())
                    }
                    None => field.default_name.clone(),
                };

                new_row.insert(qid, val);
            }

            result.push(new_row);
        }
        Ok(result)
    }

    fn static_eval(&self) -> EvalResult<Table> {
        let mut new_row: TableRow = IndexMap::new();

        for field in self {
            let val = field.expr.static_eval()?;

            let qid = match &field.alias {
                Some(alias) => {
                    QualifiedIdentifier::new(field.default_name.src.clone(), alias.clone())
                }
                None => field.default_name.clone(),
            };

            new_row.insert(qid, val);
        }

        if new_row.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![new_row])
        }
    }
}

impl EvaluableAsQuery<Table, TableAliasMap, Table> for SelectRqst {
    fn eval_dyn(&self, ctx: &Table, aliases: &TableAliasMap) -> EvalResult<Table> {
        match self.condition.as_ref() {
            Some(c) => {
                let a = ctx
                    .iter()
                    .filter_map(|row| -> Option<EvalResult<TableRow>> {
                        match c.eval_dyn(row, aliases) {
                            Ok(cell) => {
                                if cell.as_bool() {
                                    Some(Ok(row.clone()))
                                } else {
                                    None
                                }
                            }
                            Err(err) => Some(Err(err)),
                        }
                    })
                    .collect::<EvalResult<Vec<_>>>()?;

                self.handle_fields(a, aliases)
            }
            None => self.handle_fields(ctx.clone(), aliases),
        }
    }
    fn static_eval(&self) -> EvalResult<Table> {
        match &self.fields {
            FieldRqst::All => Err(EvalEror::<String>::not_static_variable()),
            FieldRqst::Selected(fields) => {
                let ctx = EvalResult::<CtxSELECT>::from(self)?;
                let j = self.eval_dyn(&fields.static_eval()?, &ctx.alias)?;
                self.join.apply_as_join(Box::new(j), &ctx)
            }
        }
    }
}
impl SelectRqst {
    pub fn handle_fields(&self, ctx_where: Table, aliases: &TableAliasMap) -> EvalResult<Table> {
        match &self.fields {
            FieldRqst::All => Ok(ctx_where),
            FieldRqst::Selected(fields) => fields.eval_dyn(&ctx_where, aliases),
        }
    }

    pub fn eval(&self) -> EvalResult<Table> {
        let ctx = EvalResult::<CtxSELECT>::from(self)?;
        match &self.from {
            Some(_) => self.eval_dyn(&ctx.get_new_table_from_rqst(self)?, &ctx.alias),
            None => self.static_eval(),
        }
    }
}
