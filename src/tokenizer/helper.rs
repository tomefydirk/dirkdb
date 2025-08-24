use crate::general_const::{COMMA_SIGN, PARENS_1, SEMICOLON_SIGN};

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
