use crate::{error_lib::evaluation::{EvalEror, EvalErrorkind}, evaluation::LgResult};

pub fn my_modulo(left: f64, right: f64) -> f64 {
    let q = (left / right) as u64;
    left - right * (q as f64)
}
pub fn convert_sql_to_regex(pattern: &str) -> LgResult<regex::Regex> {
        let regex_pattern = format!(
            "(?i)^{}$",
            regex::escape(pattern)
                .replace("%", ".*")
                .replace("_", ".")
        );

        let b=   regex::Regex::new(&regex_pattern);
        match  b{
            Ok(r) => {
                Ok(r)
            },
            Err(_) =>{
               Err(EvalEror::build(pattern.to_string(),EvalErrorkind::RegexInvalid))
            },
        }
     
    }