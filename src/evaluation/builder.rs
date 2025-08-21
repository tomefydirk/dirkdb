use crate::general_struct::element::LogicResult;



impl From<bool> for  LogicResult{
    fn from(value: bool) -> Self {
        LogicResult::Boolean(value)
    }
}