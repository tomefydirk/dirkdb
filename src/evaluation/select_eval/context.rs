use std::collections::HashMap;

use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{AliasGetter, JoinOpperand, LgResult},
    from_registry::{get_tables},
    general_struct::structure::{JoinElement, SelectRqst, Table, TableOrigin, TableWithAlias},
};
#[derive(Debug, Clone)]
pub struct CtxSELECT {
    pub base: HashMap<String, Table>,
    pub alias: HashMap<String, String>,
}
impl TableWithAlias {
    pub fn get_source(&self) -> LgResult<(String, Table)> {
        match &self.origin {
            TableOrigin::Name{name,id} => {
               get_tables(id.clone(), name)
            }
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
    pub fn new(base: HashMap<String, Table>, alias: HashMap<String, String>) -> Self {
        Self { base, alias }
    }

    pub fn init_base(rqst: &SelectRqst) -> LgResult<HashMap<String, Table>> {
        let mut a = HashMap::<String, Table>::new();
        match &rqst.from {
            Some(tb) => {
                let source = tb.get_source()?;
                a.insert(source.0, source.1);
            }
            None => return Ok(a),
        }

        for tb in rqst.join.iter() {
            let source = tb.table.get_source()?;
            a.insert(source.0, source.1);
        }
        Ok(a)
    }
    pub fn init_alias(rqst: &SelectRqst) -> LgResult<HashMap<String, String>> {
        rqst.get_alias_map()
    }
    pub fn get_table(&self, name: &String) -> LgResult<&Table> {
        match self.base.get(name) {
            Some(t) => Ok(t),
            None => Err(EvalEror::<String>::not_in_database(name.clone())),
        }
    }
}

impl From<&SelectRqst> for LgResult<CtxSELECT> {
    fn from(value: &SelectRqst) -> Self {
        let a=CtxSELECT::new(
            CtxSELECT::init_base(value)?,
            CtxSELECT::init_alias(value)?,
        );
        Ok(a)
    }
}

impl CtxSELECT {
    pub fn get_new_table(
        &self,
        origin_table: &String,
        joinop: &Vec<JoinElement>,
    ) -> LgResult<Table> {
        joinop.apply_as_join(Box::new(self.get_table(origin_table)?.clone()), self)
    }
    pub fn get_new_table_from_rqst(&self, value: &SelectRqst) -> LgResult<Table> {
        match &value.from {
            Some(t) => self.get_new_table(&t.get_name(), &value.join),
            None => Ok(Vec::new()),
        }
    }
}
