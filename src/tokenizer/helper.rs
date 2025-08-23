pub (crate) fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub (crate)  fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}