pub mod condition_eval;
pub mod join_helper;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{AliasGetter, EvaluableAsQuery, LgResult},
    from_registry::make_tables,
    general_struct::structure::{
        Field, FieldRqst, JoinElement, QualifiedIdentifier, SelectRqst, Table, TableAliasMap,
        TableOrigin, TableRow, TableWithAlias,
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
        match &self.from {
            Some(t) => self.eval_dyn(&t.eval()?, &t.get_alias_map()),
            None => self.static_eval(),
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
                    QualifiedIdentifier::new(Option::Some(owner.clone()), name.name.clone()),
                    value.clone(),
                );
            }
            result.push(new_row);
        }
        Ok(result)
    }
    fn eval(&self) -> LgResult<Table> {
        match &self.origin {
            TableOrigin::Name(n) => {
                let g = make_tables();
                if let Some(a) = g.get(n) {
                    Ok(a.clone())
                } else {
                    Err(EvalEror::<String>::not_in_database(n.clone()))
                }
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
impl AliasGetter for TableWithAlias {
    fn get_alias_map(&self) -> HashMap<String, String> {
        let mut retour = HashMap::<String, String>::new();
        match (&self.alias, &self.origin) {
            (Some(alias), TableOrigin::Name(n)) => {
                retour.insert(alias.clone(), n.clone());
                retour
            }
            _ => retour,
        }
    }
}
impl AliasGetter for JoinElement {
    fn get_alias_map(&self) -> HashMap<String, String> {
        self.table.get_alias_map()
    }
}

impl AliasGetter for Vec<JoinElement> {
    fn get_alias_map(&self) -> HashMap<String, String> {
        let mut retour = HashMap::<String, String>::new();
        for a in self {
            retour.extend(a.get_alias_map());
        }
        retour
    }
}
