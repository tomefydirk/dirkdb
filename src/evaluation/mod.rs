mod func;
mod helper;
pub type LgResult<T, E = crate::error_lib::evaluation::EvalEror<String>> =
    std::result::Result<T, E>;
