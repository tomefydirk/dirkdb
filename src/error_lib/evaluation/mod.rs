use crate::function::sql::Signature;

#[derive(Debug)]
pub enum EvalErrorkind {
    FieldNotFound,
    RegexInvalid,
    IncorrectDateValue,
    FunctionNotFound,
}

#[derive(Debug, thiserror::Error)]
#[error("{code:?} : '{input}' ")]
pub struct EvalEror<I> {
    pub input: I,
    pub code: EvalErrorkind,
}
impl<I> EvalEror<I> {
    pub fn build(input: I, code: EvalErrorkind) -> Self {
        Self { input, code }
    }
    pub fn incorrect_date_value(input: I) -> Self {
        Self {
            input,
            code: EvalErrorkind::IncorrectDateValue,
        }
    }
    pub fn regex_invalid(input: I) -> Self {
        Self {
            input,
            code: EvalErrorkind::RegexInvalid,
        }
    }
    pub fn field_notfound(input: I) -> Self {
        Self {
            input,
            code: EvalErrorkind::FieldNotFound,
        }
    }
    pub fn function_not_found(s: Signature) -> EvalEror<String> {
        EvalEror {
            input: format!("{s:?}"),
            code: EvalErrorkind::FunctionNotFound,
        }
    }
}
