use crate::{evaluation::utils::bool_transform, general_struct::element::{LogicResult, TableCell}};

impl From<TableCell> for bool {
    fn from(value: TableCell) -> Self {
        match value {
            TableCell::String(a) => !a.is_empty(),
            TableCell::Number(n) => n!=0.0 ,
            TableCell::Null => false,
        }
    }
}
impl From<LogicResult> for bool {
    fn from(value: LogicResult) -> Self {
        match value {
            LogicResult::Boolean(b) =>b ,
            LogicResult::Other(t) =>t.into(),
        }
    }
}

impl Default for LogicResult {
    fn default() -> Self {
        LogicResult::Other(TableCell::Null)
    }
}

impl From<LogicResult> for TableCell{
    fn from(value: LogicResult) -> Self {
       match value {
        LogicResult::Boolean(b) => {
            TableCell::Number(bool_transform(b))
        },
        LogicResult::Other(table_cell) => table_cell,
           }
    }
}