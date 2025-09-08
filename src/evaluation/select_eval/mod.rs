pub mod condition_eval;
pub mod join_helper;
pub mod context;
use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{select_eval::context::CtxSELECT, AliasGetter, EvaluableAsQuery, LgResult},
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

impl EvaluableAsQuery<CtxSELECT, TableAliasMap, Table> for SelectRqst {
    fn eval_dyn(&self, ctx: &CtxSELECT, aliases: &TableAliasMap) -> LgResult<Table> {
        todo!()
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
            Some(t) =>{
                let ctx=LgResult::<CtxSELECT>::from(self);
                self.eval_dyn(&ctx?, &t.get_alias_map()?)
            } ,
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
   
}
impl AliasGetter for TableWithAlias {
    fn get_alias_map(&self) -> LgResult<HashMap<String, String>> {
        let mut retour = HashMap::<String, String>::new();
        match (&self.alias, &self.origin) {
            (Some(alias), TableOrigin::Name(n)) => {
                retour.insert(alias.clone(), n.clone());
                Ok(retour)
            }
            (Some(alias), TableOrigin::SubRequest { rqst:_, id }) =>{
                 retour.insert(alias.clone(), id.clone());
                Ok(retour)
            },
            _ => Ok(retour),
        }
    }
}
impl AliasGetter for JoinElement {
    fn get_alias_map(&self) -> LgResult<HashMap<String, String>> {
       self.table.get_alias_map()
    }
}

impl AliasGetter for Vec<JoinElement> {
    fn get_alias_map(&self) -> LgResult<HashMap<String, String>> {
        let mut retour = HashMap::<String, String>::new();
        for a in self {
            retour.extend(a.get_alias_map()?);
        }
        Ok(retour)
    }
}

impl AliasGetter for SelectRqst{
    fn get_alias_map(&self)->LgResult<HashMap<String,String>> {
        let mut retour = HashMap::<String, String>::new();
        if let Some(t) = &self.from { retour.extend(t.get_alias_map()?) }
        retour.extend(self.join.get_alias_map()?);
        Ok(retour)
    }
}