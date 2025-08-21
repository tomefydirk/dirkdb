use crate::general_struct::element::{LogicResult, TableCell};

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
impl Default for TableCell {
    fn default() -> Self {
        Self::Null
    }
}