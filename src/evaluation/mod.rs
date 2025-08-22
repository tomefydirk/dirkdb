use crate::error_lib;

mod func;
mod helper;
pub mod utils;

pub(crate) type Result<T, E = error_lib::evaluation::EvalEror<String>>= std::result::Result<T,E>;