use std::collections::HashMap;

use crate::general_struct::element::{Condition, LogicResult, TableCell};

impl Condition {
    pub fn eval(&self, ctx: &HashMap<String, TableCell>) -> LogicResult {
        todo!()
    }
}
