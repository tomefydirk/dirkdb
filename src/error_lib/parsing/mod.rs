use nom::Needed;
use thiserror::Error;

use crate::{error_lib::parsing::mini_err::*};
pub mod mini_err;
#[derive(Debug, Clone)]
pub enum ErrorKind {
    Parens1Missing,
    TokenNotfound,
    AfterIsorIsnot(String),
    Aliasnotvalid,
    AliasNeeded,
    InputIncomplet,
    InputInvalid,
    Andifiication
}
#[derive(Debug, Error,Clone)]
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

pub fn factor_error(input: String) -> ParserErr<String> {
    ParserErr::build(format!("une parenthèse n'est jamais fermé ! [à la place : '{}']",input), ErrorKind::Parens1Missing)
}
pub fn alias_needed_parsing() -> ParserErr<String> {
    ParserErr::build(
        "Every derived table must have its own alias".to_string(),
        ErrorKind::AliasNeeded,
    )
}
pub fn token_not_found(input: String) -> ParserErr<String> {
    ParserErr::build(input, ErrorKind::TokenNotfound)
}
pub fn after_is_or_isnot(input: String) -> ParserErr<String> {
    let msg = "Désolé aprés 'IS' ou 'IS NOT' est forçément 'NULL'".to_string();
    ParserErr::build(input, ErrorKind::AfterIsorIsnot(msg))
}
pub fn alias_not_valid(input: String) -> ParserErr<String> {
    ParserErr::build(input, ErrorKind::Aliasnotvalid)
}
pub fn input_incomplet(input: String) -> ParserErr<String> {
    ParserErr::build(input, ErrorKind::InputIncomplet)
}
pub fn input_invalide()->ParserErr<String>{
    ParserErr::build("votre phrase contient des tokens invalide".to_string(), ErrorKind::InputInvalid)
}
pub fn and_ification_err()->ParserErr<String>{
    ParserErr::build("Le procéssus and_ification semble avoir eu un erreur".to_string(), ErrorKind::InputInvalid)
}
#[derive(Debug, thiserror::Error,Clone)]
pub enum Error<I> {
    Nom(#[from] nom::error::Error<I>),
    Parser(#[from] ParserErr<I>),
    Nested(Vec<Self>),
    FromStrBinOp(#[from] FromStrBinOpError),
    FromStrCmpOp(#[from] FromStrCmpOpError),
    FromStrLgclOp(#[from] FromStrLogicalOpError),
}




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
pub(crate) fn into_nom_incomplete<I>(e: Needed) -> nom::Err<Error<I>>
where
{
    nom::Err::Incomplete(e)
}
//maping Error(&str) ---> Error(String)

pub trait ErreurStringable {
    fn to_string_err(&self) -> nom::Err<Error<String>>;
}
impl ParserErr<&str> {
    fn to_string_err(&self) -> ParserErr<String> {
        ParserErr::build(self.input.to_string(), self.code.clone())
    }
}
impl Error<&str> {
    fn to_string_err(&self) -> Error<String> {
        match &self {
            Error::Nom(error) => nom::error::Error::new(error.input.to_string(), error.code).into(),
            Error::Parser(parser_err) => parser_err.to_string_err().into(),
            Error::Nested(_) => ParserErr::build(
                "Erreur votre requete est littérament invalide".to_string(),
                ErrorKind::TokenNotfound,
            )
            .into(),
            Error::FromStrBinOp(b) => b.clone().into(),
            Error::FromStrCmpOp(c) => c.clone().into(),
            Error::FromStrLgclOp(l) => l.clone().into(),
        }
    }
}
impl ErreurStringable for nom::Err<Error<&str>> {
    fn to_string_err(&self) -> nom::Err<Error<String>> {
        match &self {
            nom::Err::Incomplete(needed) => into_nom_incomplete(*needed),
            nom::Err::Error(a) => into_nom_error(a.to_string_err()),
            nom::Err::Failure(a) => into_nom_error(a.to_string_err()),
        }
    }
}
