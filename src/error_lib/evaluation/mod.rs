use crate::{function::sql::Signature, general_struct::structure::TableCell};

#[derive(Debug)]
pub enum EvalErrorkind {
    FieldNotFound,
    RegexInvalid,
    IncorrectDateValue,
    FunctionNotFound,
    NegativeintoSQRT,
    IncompatibleType,
    NotStaticVariable,
    AmbiguousName,
    AliasNeeded,
    NotInDatabases,
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
    pub fn negative_into_sqrt(number: f64) -> EvalEror<String> {
        EvalEror {
            input: number.to_string(),
            code: EvalErrorkind::NegativeintoSQRT,
        }
    }
    pub fn incompatible_type(t: &TableCell) -> EvalEror<String> {
        EvalEror {
            input: format!("{t:?}"),
            code: EvalErrorkind::IncompatibleType,
        }
    }
    pub fn not_static_variable() -> EvalEror<String> {
        EvalEror {
            input: "*".to_string(),
            code: EvalErrorkind::NotStaticVariable,
        }
    }
    pub fn ambiguous_name(field_name: String) -> EvalEror<String> {
        EvalEror {
            input: field_name,
            code: EvalErrorkind::AmbiguousName,
        }
    }
    pub fn alias_need() -> EvalEror<String> {
        EvalEror {
            input: "Every derived table must have its own alias".to_string(),
            code: EvalErrorkind::AliasNeeded,
        }
    }
    pub fn not_in_database(table: String) -> EvalEror<String> {
        EvalEror::build(table, EvalErrorkind::NotInDatabases)
    }
}
