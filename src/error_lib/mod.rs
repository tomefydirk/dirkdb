use crate::error_lib;

pub mod evaluation;
pub mod parsing;

#[derive(Debug, thiserror::Error)]
pub enum   SqlError<T>{
    Parsing(#[from] nom::Err<error_lib::parsing::Error<T>>),
    Evaluation(#[from] error_lib::evaluation::EvalEror<T>)
}