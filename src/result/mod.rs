use std::fmt::Display;

use crate::error_lib::parsing::{Error, input_invalide, into_nom_failure};

//Token result
pub type TokenResult<I, O, E = Error<I>> = nom::IResult<I, O, E>;

//parsing result
pub type ParsingResult<I, O, E = Error<String>> = nom::IResult<I, O, E>;

pub trait ErrorMappingIntoParsingError<I, O> {
    fn map_err_into_parsing(self) -> ParsingResult<I, O>;
}

impl<I, O> ErrorMappingIntoParsingError<I, O> for TokenResult<I, O>
where
    I: Display + Clone,
    O: Clone,
{
    fn map_err_into_parsing(self) -> ParsingResult<I, O> {
        self.map_err(|_| into_nom_failure(input_invalide()))
    }
}
