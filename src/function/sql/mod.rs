pub mod datesql;
pub mod names;
pub mod numbersql;
pub mod stringsql;

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use crate::{
    error_lib::evaluation::EvalEror, evaluation::LgResult, general_struct::element::TableCell,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    name: String,
    parameter: usize,
}

impl Signature {
    pub fn new(name: String, parameter: usize) -> Self {
        Self { name, parameter }
    }
}

impl Hash for Signature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.parameter.hash(state);
    }
}

type FuncSQL = fn(Vec<TableCell>) -> TableCell;

pub struct FunctionRegistry {
    funcs: HashMap<Signature, FuncSQL>,
}

impl FunctionRegistry {
    pub fn match_with(&self, s: Signature) -> LgResult<FuncSQL> {
        match self.funcs.get(&s) {
            Some(f) => Ok(*f),
            None => Err(EvalEror::<String>::function_not_found(s)),
        }
    }
}
