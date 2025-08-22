pub fn my_modulo(left: f64, right: f64) -> f64 {
    let q = (left / right) as u64;
    left - right * (q as f64)
}
pub fn convert_sql_to_regex(pattern: &str) -> regex::Regex {
        let regex_pattern = format!(
            "(?i)^{}$",
            regex::escape(pattern)
                .replace("%", ".*")
                .replace("_", ".")
        );

        regex::Regex::new(&regex_pattern).expect("Regex LIKE invalide")
    }