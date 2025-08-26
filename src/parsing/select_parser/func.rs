use crate::error_lib::parsing::{into_nom_error, token_not_found};
use crate::general_struct::constant::{FROM_SIGN, SELECT_SIGN, WHERE_SIGN};
use crate::general_struct::structure::{Condition, SelectRqst, TableWithAlias};
use crate::parsing::select_parser::field_parser::parse_fieldrqst;
use crate::parsing::select_parser::from_parser::parse_from;
use crate::parsing::select_parser::where_parser::parse_where;
use crate::tokenizer::{scan_token, Token};
use crate::IResult;

pub fn parse_select(input: &str) -> IResult<&str, SelectRqst> {
 
    let (mut input, token) = scan_token(input)?;
    match token {
        Token::Other(word) if word.eq_ignore_ascii_case(SELECT_SIGN) => {
            let (rest, fields) = parse_fieldrqst(input)?;
            input = rest;

            
            let (rest, from) = parse_optional_from(input)?;
            input = rest;

            let (rest, where_clause) = parse_optional_where(input)?;
            input = rest;

           
            Ok((input, SelectRqst::new(fields, from, where_clause)))
        }
        _ => Err(into_nom_error(token_not_found(input))),
    }
}


fn parse_optional_from(input: &str) -> IResult<&str, Option<TableWithAlias>> {
    let (maybe_input, tok) = scan_token(input)?;
    match tok {
        Token::Other(word) if word.eq_ignore_ascii_case(FROM_SIGN) => {
            let (rest, origin) = parse_from(maybe_input)?;
            Ok((rest, Some(*origin)))
        }
        _ => Ok((input, None)), 
    }
}


fn parse_optional_where(input: &str) -> IResult<&str, Option<Condition>> {
    let (maybe_input, tok) = scan_token(input)?;
    match tok {
        Token::Other(word) if word.eq_ignore_ascii_case(WHERE_SIGN) => {
            let (rest, cond) = parse_where(maybe_input)?;
            Ok((rest, Some(*cond)))
        }
        _ => Ok((input, None)), // rien consommé
    }
}
