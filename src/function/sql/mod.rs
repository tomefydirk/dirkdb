pub mod list_func;
use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::LgResult,
    function::sql::list_func::{datediff, now, sqrt},
    general_struct::structure::{QualifiedIdentifier, TableCell},
};

#[derive(Debug, Clone)]
pub struct Signature {
    name: QualifiedIdentifier,
    parameter: usize,
}

impl Signature {
    pub fn new(name: QualifiedIdentifier, parameter: usize) -> Self {
        Self { name, parameter }
    }
}
impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        if self.name.column.eq_ignore_ascii_case(&other.name.column)
            && self.parameter == other.parameter
        {
            match (&self.name.table, &other.name.table) {
                (None, None) => true,
                (Some(a), Some(b)) if a == b => true,
                _ => false,
            }
        } else {
            false
        }
    }
}
impl Eq for Signature {}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(a) = &self.name.table {
            a.hash(state);
        }
        self.name.column.to_lowercase().hash(state);
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
        funcs.insert(Signature::new("sqrt".into(), 1), sqrt as FuncSQL);
        funcs.insert(
            Signature::new("datediff".into(), 2),
            datediff as FuncSQL,
        );
        funcs.insert(Signature::new("now".into(), 0), now as FuncSQL);
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
    pub fn call(&self, name: &QualifiedIdentifier, args: Vec<TableCell>) -> LgResult<TableCell> {
        let sig = Signature::new(name.clone(), args.len());
        let func = self.match_with(sig)?;
        func(args.clone())
    }
}
