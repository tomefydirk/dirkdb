use std::collections::HashMap;

use crate::{evaluation::select_eval::context::CtxSELECT, general_struct::structure::Table};

pub mod helper;
pub mod select_eval;
pub type LgResult<T, E = crate::error_lib::evaluation::EvalEror<String>> =
    std::result::Result<T, E>;

pub trait EvaluableAsQuery<Ctx, Aliases, O> {
    fn eval_dyn(&self, ctx: &Ctx, aliases: &Aliases) -> LgResult<O>;
    fn static_eval(&self) -> LgResult<O>;
}

pub trait OperatorQuery<T, O> {
    fn default_apply(&self, left: T, right: T) -> O;
}

pub trait AliasGetter {
    fn get_alias_map(&self)->LgResult<HashMap<String,String>>;
}

pub trait JoinOpperand {
    fn apply_as_join(&self,origin_table: Box<Table>, ctx: &CtxSELECT)->LgResult<Table>;  
}

pub trait AliasMap : AliasGetter {
    fn extends_aliases<T: AliasMap>(&mut self,other:T);
    fn get_original_name<K>(&self,alias:K)->&String;
}