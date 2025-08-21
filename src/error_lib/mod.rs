use thiserror::Error;
#[derive(Debug)]
pub enum ErrorKind {
    Parens1Missing,
    TokenNotfound,
}

#[derive(Debug, Error)]
#[error("Erreur prés de : '{input}'")]
pub struct ParserErr<'a> {
    input: &'a str,
    code: ErrorKind,
}
impl<'a> ParserErr<'a> {
    pub fn build(input: &'a str, code: ErrorKind) -> Self {
        Self { input, code }
    }
}


//-------list of error :
pub fn create_factor_error(input: &str) -> ParserErr {
    ParserErr::build(input, ErrorKind::Parens1Missing)
}
