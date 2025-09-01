use std::collections::HashMap;

pub mod from_registry;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{LgResult, select_eval::from_registry::make_tables},
    general_struct::structure::{
        Field, FieldRqst, QualifiedIdentifier, SelectRqst, Table, TableAliasMap, TableOrigin,
        TableRow, TableWithAlias,
    },
};

pub mod condition_eval;

pub trait FieldEval {
    fn static_eval(&self) -> LgResult<Table>;
    fn eval(&self, ctx: &Table, aliases: &TableAliasMap) -> LgResult<Table>;
}

impl FieldEval for Vec<Field> {
    fn eval(&self, ctx: &Table, aliases: &TableAliasMap) -> LgResult<Table> {
        let mut result: Table = Vec::new();

        for row in ctx {
            let mut new_row: TableRow = HashMap::new();

            for field in self {
                let val = field.expr.eval(row, aliases)?;

                // Si on a un alias, on le met comme "column" dans le QualifiedIdentifier
                let qid = match &field.alias {
                    Some(alias) => QualifiedIdentifier {
                        table: field.default_name.table.clone(),
                        column: alias.clone(),
                    },
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
                Some(alias) => QualifiedIdentifier {
                    table: field.default_name.table.clone(),
                    column: alias.clone(),
                },
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

impl TableWithAlias {
    pub fn change_table_owner(table: Table, owner: String) -> LgResult<Table> {
        let mut result: Table = Vec::new();
        for row in table {
            let mut new_row: TableRow = HashMap::new();

            for (name, value) in row.iter() {
                new_row.insert(
                    QualifiedIdentifier::new(Option::Some(owner.clone()), name.column.clone()),
                    value.clone(),
                );
            }
            result.push(new_row);
        }
        Ok(result)
    }
    pub fn get_alias_map(&self) -> HashMap<String, String> {
        let mut retour = HashMap::<String, String>::new();
        match (&self.origin, &self.alias) {
            (TableOrigin::Name(n), Some(alias)) => {
                retour.insert(n.clone(), alias.clone());
                retour
            }
            _ => retour,
        }
    }

    fn eval(&self) -> LgResult<Table> {
        match &self.origin {
            TableOrigin::Name(n) => {
                let g = make_tables();
                /*todo!! */
                let a = g.get(n).unwrap();
                Ok(a.clone())
            }
            TableOrigin::SubRequest(select_rqst) => match &self.alias {
                Some(owner) => {
                    TableWithAlias::change_table_owner(select_rqst.clone().eval()?, owner.clone())
                }
                None => Err(EvalEror::<String>::alias_need()),
            },
        }
    }
}

impl SelectRqst {
    pub fn handle_fields(&self, ctx_where: Table, aliases: &TableAliasMap) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Ok(ctx_where),
            FieldRqst::Selected(fields) =>{
               // println!("{ctx_where:?}");
                fields.eval(&ctx_where, aliases)
            } ,
        }
    }
    pub fn eval_dyn(&self, ctx: &Table, aliases: &TableAliasMap) -> LgResult<Table> {
        match self.condition.as_ref() {
            Some(c) => {
                let a = ctx
                    .iter()
                    .filter_map(|row| -> Option<LgResult<TableRow>> {
                        match c.eval(row, aliases) {
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
    pub fn static_eval(&self) -> LgResult<Table> {
        match &self.fields {
            FieldRqst::All => Err(EvalEror::<String>::not_static_variable()),
            FieldRqst::Selected(fields) => fields.static_eval(),
        }
    }
    pub fn eval(&self) -> LgResult<Table> {
        match &self.from {
            Some(t) => self.eval_dyn(&t.eval()?, &t.get_alias_map()),
            None => self.static_eval(),
        }
    }
}
