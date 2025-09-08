use std::collections::HashMap;

use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::{AliasGetter, LgResult},
    from_registry::make_tables,
    general_struct::structure::{SelectRqst, Table, TableOrigin, TableWithAlias},
};
#[derive(Debug, Clone)]
pub struct CtxSELECT {
    pub base: HashMap<String, Table>,
    pub alias: HashMap<String, String>,
}
impl TableWithAlias {
    pub fn get_source(&self) -> LgResult<(String, Table)> {
        match &self.origin {
            TableOrigin::Name(e) => {
                let g = make_tables();
                if let Some(a) = g.get(e) {
                    Ok((e.clone(), a.clone()))
                } else {
                    Err(EvalEror::<String>::not_in_database(e.clone()))
                }
            }
            TableOrigin::SubRequest { rqst, id } => Ok((id.clone(), rqst.eval()?)),
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
}

impl From<&SelectRqst> for LgResult<CtxSELECT> {
    fn from(value: &SelectRqst) -> Self {
        Ok(CtxSELECT::new(
            CtxSELECT::init_base(value)?,
            CtxSELECT::init_alias(value)?,
        ))
    }
}
