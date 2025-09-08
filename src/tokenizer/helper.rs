use crate::{
    IResult,
    general_struct::constant::*,
    tokenizer::{Token, scan_token},
};

pub(crate) fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

pub(crate) fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
pub fn codon_stop(input: &str) -> bool {
    input.trim().starts_with(PARENS_1)
        || input.trim().is_empty()
        || input.trim().starts_with(COMMA_SIGN)
        || input.trim().starts_with(SEMICOLON_SIGN)
}
pub fn is_func_valid(input: &str) -> bool {
    !key_word_list().contains(&input.to_lowercase())
}

pub trait Factorable {
    fn is_factor_parens(&self) -> bool;
}
pub trait Tokenizable {
    fn scan_token(&self) -> IResult<Self, Token>
    where
        Self: Sized;
    fn starts_with_token(&self, value: &Token) -> bool;
    fn starts_with_join_op(&self) -> bool
    where
        Self: Sized,
    {
        self.starts_with_token(&Token::Other(JOIN))
            || matches!(self.scan_token(), Ok((_,Token::Mkw(mkw))) if (mkw == left_join() || mkw == right_join() || mkw == full_join()))
    }
}
impl Factorable for &str {
    fn is_factor_parens(&self) -> bool {
        self.trim().starts_with(PARENS_0)
    }
}
impl Tokenizable for &str {
    fn scan_token(&self) -> IResult<Self, Token>
    where
        Self: Sized,
    {
        scan_token(self)
    }
    fn starts_with_token(&self, value: &Token) -> bool {
        let a = self.scan_token();
        match &a {
            Ok(t) => t.1 == *value,
            Err(_) => todo!(),
        }
    }
}
