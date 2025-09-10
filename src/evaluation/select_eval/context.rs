use indexmap::IndexMap;

use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{AliasGetter, JoinOpperand, EvalResult},
    from_registry::get_tables,
    general_struct::structure::{JoinElement, SelectRqst, Table, TableOrigin, TableWithAlias},
};
#[derive(Debug, Clone)]
pub struct CtxSELECT {
    pub base: IndexMap<String, Table>,
    pub alias: IndexMap<String, String>,
}
impl TableWithAlias {
    pub fn get_source(&self) -> EvalResult<(String, Table)> {
        match &self.origin {
            TableOrigin::Name { name, id } => get_tables(id.clone(), name),
            TableOrigin::SubRequest { rqst, id } => match &self.alias {
                Some(owner) => Ok((
                    id.clone(),
                    TableWithAlias::change_table_owner(rqst.clone().eval()?, owner.clone())?,
                )),
                None => Err(EvalEror::<String>::alias_need()),
            },
        }
    }
}
impl CtxSELECT {
    pub fn new(base: IndexMap<String, Table>, alias: IndexMap<String, String>) -> Self {
        Self { base, alias }
    }

    pub fn init_base(rqst: &SelectRqst) -> EvalResult<IndexMap<String, Table>> {
        let mut a = IndexMap::<String, Table>::new();
        if let Some(tb)= &rqst.from {
               let source = tb.get_source()?;
                a.insert(source.0, source.1);
        }

        for tb in rqst.join.iter() {
            let source = tb.table.get_source()?;
            a.insert(source.0, source.1);
        }
        Ok(a)
    }
    pub fn init_alias(rqst: &SelectRqst) -> EvalResult<IndexMap<String, String>> {
        rqst.get_alias_map()
    }
    pub fn get_table(&self, name: &String) -> EvalResult<&Table> {
        match self.base.get(name) {
            Some(t) => Ok(t),
            None => Err(EvalEror::<String>::not_in_database(name.clone())),
        }
    }
}

impl From<&SelectRqst> for EvalResult<CtxSELECT> {
    fn from(value: &SelectRqst) -> Self {
        let a = CtxSELECT::new(CtxSELECT::init_base(value)?, CtxSELECT::init_alias(value)?);
        Ok(a)
    }
}

impl CtxSELECT {
    pub fn get_new_table(
        &self,
        origin_table: &String,
        joinop: &Vec<JoinElement>,
    ) -> EvalResult<Table> {
        joinop.apply_as_join(Box::new(self.get_table(origin_table)?.clone()), self)
    }
    pub fn get_new_table_from_rqst(&self, value: &SelectRqst) -> EvalResult<Table> {
        match &value.from {
            Some(t) => self.get_new_table(&t.get_name(), &value.join),
            None => Ok(Vec::new()),
        }
    }
}
