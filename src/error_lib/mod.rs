use thiserror::Error;

use crate::parsing::atom_parser::element::FromStrBinOpError;
#[derive(Debug)]
pub enum ErrorKind {
    Parens1Missing,
    TokenNotfound,
}

#[derive(Debug, Error)]
#[error("Erreur prés de : '{input}'")]
pub struct ParserErr<I> {
    input: I,
    code: ErrorKind,
}
impl<I> ParserErr<I> {
    pub fn build(input: I, code: ErrorKind) -> Self {
        Self { input, code }
    }
}


//-------list of error :
pub fn create_factor_error(input: &str) -> ParserErr<&str> {
    ParserErr::build(input, ErrorKind::Parens1Missing)
}

#[derive(Debug, thiserror::Error)]
pub enum Error<I> {
    Nom(#[from] nom::error::Error<I>),
    Parser(#[from] ParserErr<I>),
    Nested(Vec<Self>),
    FromStrBinOp(#[from] FromStrBinOpError)
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

pub(crate) fn into_nom_failure<I,E>(e: E) -> nom::Err<Error<I>>
where E: Into<Error<I>> {
    nom::Err::Failure(e.into())    
}

pub(crate) fn into_nom_error<I,E>(e: E) -> nom::Err<Error<I>>
where E: Into<Error<I>> {
    nom::Err::Error(e.into())    
}