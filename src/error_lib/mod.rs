use crate::error_lib;

pub mod evaluation;
pub mod parsing;

#[derive(Debug, thiserror::Error)]
pub enum   SqlError<I>{
    Parsing(#[from] error_lib::parsing::Error<I>),
    Evaluation(#[from] error_lib::evaluation::EvalEror<I>)
}