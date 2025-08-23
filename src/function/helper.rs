use crate::{error_lib::evaluation::EvalEror, evaluation::LgResult};

pub fn my_modulo(left: f64, right: f64) -> f64 {
    let q = (left / right) as u64;
    left - right * (q as f64)
}
pub fn bool_transform(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}
pub fn convert_sql_to_regex(pattern: &str) -> LgResult<regex::Regex> {
    let regex_pattern = format!(
        "(?i)^{}$",
        regex::escape(pattern).replace("%", ".*").replace("_", ".")
    );

    let b = regex::Regex::new(&regex_pattern);
    match b {
        Ok(r) => Ok(r),
        Err(_) => Err(EvalEror::regex_invalid(pattern.to_string())),
    }
}
