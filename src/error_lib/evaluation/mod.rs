use crate::{function::sql::Signature, general_struct::structure::TableCell};

#[derive(Debug)]
pub enum EvalErrorkind {
    FieldNotFound,
    RegexInvalid,
    IncorrectDateValue,
    FunctionNotFound,
    IncompatibleType,
    NotStaticVariable,
    AmbiguousName,
    AliasNeeded,
    NotInDatabases,
    FunctionError,
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
        EvalEror::build(input, EvalErrorkind::IncorrectDateValue)
    }
    pub fn regex_invalid(input: I) -> Self {
        EvalEror::build(input, EvalErrorkind::RegexInvalid)
    }
    pub fn field_notfound(input: I) -> Self {
        EvalEror::build(input, EvalErrorkind::FieldNotFound)
    }
    pub fn function_not_found(s: Signature) -> EvalEror<String> {
        EvalEror::<String>::build(format!("{s:?}"), EvalErrorkind::FunctionNotFound)
    }
    pub fn function_error(msg: String) -> EvalEror<String> {
        EvalEror::<String>::build(msg, EvalErrorkind::FunctionError)
    }
    pub fn incompatible_type(t: &TableCell) -> EvalEror<String> {
        EvalEror::<String>::build(format!("{t:?}"), EvalErrorkind::IncompatibleType)
    }
    pub fn not_static_variable() -> EvalEror<String> {
        EvalEror::<String>::build("*".to_string(), EvalErrorkind::NotStaticVariable)
    }
    pub fn ambiguous_name(field_name: String) -> EvalEror<String> {
        EvalEror::<String>::build(field_name, EvalErrorkind::AmbiguousName)
    }
    pub fn alias_need() -> EvalEror<String> {
        EvalEror::<String>::build(
            "Every derived table must have its own alias".to_string(),
            EvalErrorkind::AliasNeeded,
        )
    }
    pub fn not_in_database(table: String) -> EvalEror<String> {
        EvalEror::build(table, EvalErrorkind::NotInDatabases)
    }
}
