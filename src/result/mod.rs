use crate::error_lib::parsing::{Error, input_invalide, into_nom_failure};

//Token result
pub type TokenResult<I, O, E = Error<I>> = nom::IResult<I, O, E>;

//parsing result
pub type ParsingResult<I, O, E = Error<String>> = nom::IResult<I, O, E>;

pub fn convert_tr<I,O>(value:TokenResult<I,O>)->ParsingResult<I,O>{
       value.map_err(|_| into_nom_failure(input_invalide()))
}