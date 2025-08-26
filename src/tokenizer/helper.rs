use crate::general_struct::constant::*;

pub(crate) fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub(crate) fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
pub fn codon_stop(input: &str) -> bool {
    input.trim().starts_with(PARENS_1)
        && input.trim().is_empty()
        && input.trim().starts_with(COMMA_SIGN)
        && input.trim().starts_with(SEMICOLON_SIGN)
}
pub fn is_func_valid(input: &String) -> bool {
    !key_word_list().contains(input)
}

pub trait Factorable {
     fn is_factor_parens(&self)->bool;
}
impl Factorable for &str {
    fn is_factor_parens(&self)->bool {
        self.trim().starts_with(PARENS_0)
    }
}