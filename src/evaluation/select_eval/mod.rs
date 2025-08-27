use std::collections::HashMap;

use crate::{
    evaluation::LgResult,
    general_struct::structure::{
        Field, FieldRqst, SelectRqst, TableCell, TableOrigin, TableWithAlias,
    },
};

pub mod condition_eval;

pub type Table = Vec<HashMap<String, TableCell>>;

pub trait FieldEval {
    fn eval(&self, ctx: &Table) -> LgResult<Table>;
}

impl FieldEval for Vec<Field> {
    fn eval(&self, ctx: &Table) -> LgResult<Table> {
        let mut result = Vec::new();

        for row in ctx {
            let mut new_row = HashMap::new();

            for field in self {
                let val = field.expr.eval(row)?;
                let col_name = field
                    .alias
                    .clone()
                    .unwrap_or_else(|| field.default_name.clone());

                new_row.insert(col_name, val);
            }

            result.push(new_row);
        }

        Ok(result)
    }
}

impl TableWithAlias {
    fn eval(&self) -> LgResult<Table> {
        match &self.origin {
            TableOrigin::Name(n) => todo!(),
            TableOrigin::SubRequest(select_rqst) => {
                select_rqst.eval_with_from()    
            },
        }
    }
}

impl SelectRqst {
    pub fn handle_fields(&self, ctx_where: Table) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Ok(ctx_where),

            FieldRqst::Selected(fields) => fields.eval(&ctx_where),
        }
    }
    pub fn eval(&self, ctx: &Table) -> LgResult<Table> {
        match self.condition.as_ref() {
            Some(c) => {
                let a = ctx
                    .iter()
                    .filter_map(|row| -> Option<LgResult<HashMap<String, TableCell>>> {
                        match c.eval(row) {
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

                self.handle_fields(a)
            }
            None => self.handle_fields(ctx.clone()),
        }
    }
    pub fn eval_with_from(&self) -> LgResult<Table> {
        match &self.from {
            Some(a) => {
                a.eval()
            },
            None => {
                todo!()
            },
        }
    }
}
