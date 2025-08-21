use crate::general_struct::element::LogicResult;



impl From<bool> for  LogicResult{
    fn from(value: bool) -> Self {
        LogicResult::Boolean(value)
    }
}
impl From<String> for LogicResult{
    fn from(value: String) -> Self {
        LogicResult::Other(value.into())
    }
}
impl From<f64> for LogicResult{
    fn from(value: f64) -> Self {
        LogicResult::Other(value.into())
    }
}

