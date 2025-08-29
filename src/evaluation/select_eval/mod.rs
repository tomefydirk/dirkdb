use std::collections::HashMap;
pub mod from_registry;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{select_eval::from_registry::make_tables, LgResult},
    general_struct::structure::{
        Field, FieldRqst, SelectRqst, Table, TableCell, TableOrigin, TableWithAlias
    },
};

pub mod condition_eval;


pub trait FieldEval {
    fn static_eval(&self) -> LgResult<Table>;
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
    fn static_eval(&self) -> LgResult<Table> {
        let mut new_row = HashMap::new();

        for field in self {
            match field.expr.static_eval() {
                Ok(val) => {
                    let col_name = field
                        .alias
                        .clone()
                        .unwrap_or_else(|| field.default_name.clone());
                    new_row.insert(col_name, val);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }

        if new_row.is_empty() {
            Ok(vec![])
        } else {
            Ok(vec![new_row])
        }
    }
}

impl TableWithAlias {
    fn eval(&self) -> LgResult<Table> {
        match &self.origin {
            TableOrigin::Name(n) => {
                let g = make_tables();
                let a = g.get(n).unwrap();
                Ok(a.clone())
            }
            TableOrigin::SubRequest(select_rqst) => select_rqst.eval(),
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
    pub fn eval_dyn(&self, ctx: &Table) -> LgResult<Table> {
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
    pub fn eval(&self) -> LgResult<Table> {
        match &self.from {
            Some(a) => self.eval_dyn(&a.eval()?),
            None => self.static_eval(),
        }
    }
    pub fn static_eval(&self) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Err(EvalEror::<String>::not_static_variable()),
            FieldRqst::Selected(fields) => fields.static_eval(),
        }
    }
}
impl TableWithAlias {
    pub fn get_alias_map(&self) -> HashMap<String, String> {
        let mut retour = HashMap::<String, String>::new();
        match (&self.origin, &self.alias) {
            (TableOrigin::Name(n), Some(alias)) => {
                retour.insert(n.clone(), alias.clone());
                retour
            },
            _ => retour,
        }
    }
}
