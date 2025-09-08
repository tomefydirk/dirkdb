pub mod condition_eval;
pub mod join_helper;
pub mod context;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{select_eval::context::CtxSELECT,  EvaluableAsQuery, LgResult},
    general_struct::structure::{
        Field, FieldRqst,  QualifiedIdentifier, SelectRqst, Table, TableAliasMap,
        TableRow,
    },
};
use std::collections::HashMap;

impl EvaluableAsQuery<Table, TableAliasMap, Table> for Vec<Field> {
    fn eval_dyn(&self, ctx: &Table, aliases: &TableAliasMap) -> LgResult<Table> {
        let mut result: Table = Vec::new();

        for row in ctx {
            let mut new_row: TableRow = HashMap::new();

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

    fn static_eval(&self) -> LgResult<Table> {
        let mut new_row: TableRow = HashMap::new();

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
    fn eval_dyn(&self, ctx: &Table, aliases: &TableAliasMap) -> LgResult<Table> {
         match self.condition.as_ref() {
            Some(c) => {
                let a = ctx
                    .iter()
                    .filter_map(|row| -> Option<LgResult<TableRow>> {
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
                    .collect::<LgResult<Vec<_>>>()?;

                self.handle_fields(a, aliases)
            }
            None => self.handle_fields(ctx.clone(), aliases),
        }
    }
    fn static_eval(&self) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Err(EvalEror::<String>::not_static_variable()),
            FieldRqst::Selected(fields) => fields.static_eval(),
        }
    }
}
impl SelectRqst {
    pub fn handle_fields(&self, ctx_where: Table, aliases: &TableAliasMap) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Ok(ctx_where),
            FieldRqst::Selected(fields) => {
                // println!("{ctx_where:?}");
                fields.eval_dyn(&ctx_where, aliases)
            }
        }
    }

    pub fn eval(&self) -> LgResult<Table> {
            let ctx=LgResult::<CtxSELECT>::from(self)?;
        match &self.from {
            Some(_) =>{
                self.eval_dyn(&ctx.get_new_table_from_rqst(self)?, &ctx.alias)
            } ,
            None => self.static_eval(),
        }
    }
}
