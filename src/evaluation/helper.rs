use crate::{
    evaluation::utils::bool_transform,
    general_struct::element::{LogicResult, TableCell},
};

impl From<bool> for LogicResult {
    fn from(value: bool) -> Self {
        LogicResult::Boolean(value)
    }
}
impl From<String> for LogicResult {
    fn from(value: String) -> Self {
        LogicResult::Other(value.into())
    }
}
impl From<f64> for LogicResult {
    fn from(value: f64) -> Self {
        LogicResult::Other(value.into())
    }
}

impl From<TableCell> for bool {
    fn from(value: TableCell) -> Self {
        match value {
            TableCell::String(a) => !a.is_empty(),
            TableCell::Number(n) => n != 0.0,
            TableCell::Null => false,
        }
    }
}
impl From<LogicResult> for bool {
    fn from(value: LogicResult) -> Self {
        match value {
            LogicResult::Boolean(b) => b,
            LogicResult::Other(t) => t.into(),
        }
    }
}

impl Default for LogicResult {
    fn default() -> Self {
        LogicResult::Other(TableCell::Null)
    }
}

impl From<LogicResult> for TableCell {
    fn from(value: LogicResult) -> Self {
        match value {
            LogicResult::Boolean(b) => TableCell::Number(bool_transform(b)),
            LogicResult::Other(table_cell) => table_cell,
        }
    }
}
pub fn my_modulo(left: f64, right: f64) -> f64 {
    let q = (left / right) as u64;
    left - right * (q as f64)
}
