pub mod datesql;
pub mod list_func;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::LgResult,
    function::sql::list_func::{datediff, now, sqrt},
    general_struct::element::TableCell,
};

#[derive(Debug, Clone)]
pub struct Signature {
    name: String,
    parameter: usize,
}

impl Signature {
    pub fn new(name: String, parameter: usize) -> Self {
        Self { name, parameter }
    }
}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq_ignore_ascii_case(&other.name) && self.parameter == other.parameter
    }
}
impl Eq for Signature {}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.to_lowercase().hash(state);
        self.parameter.hash(state);
    }
}

type FuncSQL = fn(Vec<TableCell>) -> LgResult<TableCell>;

pub struct FunctionRegistry {
    funcs: HashMap<Signature, FuncSQL>,
}
impl FunctionRegistry {
    pub fn new() -> Self {
        let mut funcs: HashMap<Signature, FuncSQL> = HashMap::new();
        funcs.insert(Signature::new("sqrt".to_string(), 1), sqrt as FuncSQL);
        funcs.insert(
            Signature::new("datediff".to_string(), 2),
            datediff as FuncSQL,
        );
        funcs.insert(Signature::new("now".to_string(), 0), now as FuncSQL);
        Self { funcs }
    }
}

impl Default for FunctionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionRegistry {
    pub fn match_with(&self, s: Signature) -> LgResult<FuncSQL> {
        match self.funcs.get(&s) {
            Some(f) => Ok(*f),
            None => Err(EvalEror::<String>::function_not_found(s)),
        }
    }
    pub fn call(&self, name: &str, args: Vec<TableCell>) -> LgResult<TableCell> {
        let sig = Signature::new(name.to_string(), args.len());
        let func = self.match_with(sig)?;
        func(args.clone())
    }
}
