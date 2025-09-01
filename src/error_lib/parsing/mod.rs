use thiserror::Error;

use crate::error_lib::parsing::mini_err::*;
pub mod mini_err;
#[derive(Debug)]
pub enum ErrorKind {
    Parens1Missing,
    TokenNotfound,
    AfterIsorIsnot(String),
    Aliasnotvalid,
    AliasNeeded
}
#[derive(Debug, Error)]
#[error("{code:?} : '{input}' ")]
pub struct ParserErr<I> {
    input: I,
    code: ErrorKind,
}
impl<I> ParserErr<I> {
    pub fn build(input: I, code: ErrorKind) -> Self {
        Self { input, code }
    }
}

pub fn factor_error(input: &str) -> ParserErr<&str> {
    ParserErr::build(input, ErrorKind::Parens1Missing)
}
pub fn alias_needed_parsing() -> ParserErr<&'static str> {
    ParserErr::build("Every derived table must have its own alias", ErrorKind::AliasNeeded)
}
pub fn token_not_found(input: &str) -> ParserErr<&str> {
    ParserErr::build(input, ErrorKind::TokenNotfound)
}
pub fn after_is_or_isnot(input: &str) -> ParserErr<&str> {
    let msg = "Désolé aprés 'IS' ou 'IS NOT' est forçément 'NULL'".to_string();
    ParserErr::build(input, ErrorKind::AfterIsorIsnot(msg))
}
pub fn alias_not_valid(input: &str) -> ParserErr<&str>{
    ParserErr::build(input, ErrorKind::Aliasnotvalid)
}
#[derive(Debug, thiserror::Error)]
pub enum Error<I> {
    Nom(#[from] nom::error::Error<I>),
    Parser(#[from] ParserErr<I>),
    Nested(Vec<Self>),
    FromStrBinOp(#[from] FromStrBinOpError),
    FromStrCmpOp(#[from] FromStrCmpOpError),
    FromStrLgclOp(#[from] FromStrLogicalOpError),
}

pub type IResult<I, O, E = Error<I>> = nom::IResult<I, O, E>;

impl<I> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self::Nom(nom::error::Error::new(input, kind))
    }

    fn append(input: I, kind: nom::error::ErrorKind, other: Self) -> Self {
        let nom_err = Self::Nom(nom::error::Error::new(input, kind));
        let nested = match other {
            Self::Nested(mut v) => {
                v.push(nom_err);
                v
            }
            other => {
                vec![nom_err, other]
            }
        };
        Self::Nested(nested)
    }
}

pub(crate) fn into_nom_failure<I, E>(e: E) -> nom::Err<Error<I>>
where
    E: Into<Error<I>>,
{
    nom::Err::Failure(e.into())
}

pub(crate) fn into_nom_error<I, E>(e: E) -> nom::Err<Error<I>>
where
    E: Into<Error<I>>,
{
    nom::Err::Error(e.into())
}
