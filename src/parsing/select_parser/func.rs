use crate::ParsingResult;
use crate::error_lib::parsing::{into_nom_error, token_wrong_place};
use crate::general_struct::constant::{FROM_SIGN, SELECT_SIGN, WHERE_SIGN};
use crate::general_struct::structure::{Condition, SelectRqst, TableWithAlias};
use crate::parsing::select_parser::field_parser::parse_fieldrqst;
use crate::parsing::select_parser::from_parser::parse_from;
use crate::parsing::select_parser::join_parser::parse_joins;
use crate::parsing::select_parser::where_parser::parse_where;
use crate::tokenizer::{Token, scan_token};

pub fn parse_select(input: &str) -> ParsingResult<&str, SelectRqst> {
    let (mut input, token) = scan_token(input)?;
    match token {
        Token::Other(word) if word.eq_ignore_ascii_case(SELECT_SIGN) => {
            let (rest, fields) = parse_fieldrqst(input)?;
            input = rest;

            let (rest, from) = parse_optional_from(input)?;
            input = rest;

            let (rest, join_clause) = parse_joins(input)?;
            input = rest;

            let (rest, where_clause) = parse_optional_where(input)?;
            input = rest;

            Ok((
                input,
                SelectRqst::new(fields, from, join_clause, where_clause),
            ))
        }
        a => Err(into_nom_error(token_wrong_place(a.to_string()))),
    }
}

fn parse_optional_from(input: &str) -> ParsingResult<&str, Option<TableWithAlias>> {
    let (maybe_input, tok) = scan_token(input)?;
    match tok {
        Token::Other(word) if word.eq_ignore_ascii_case(FROM_SIGN) => {
            let (rest, origin) = parse_from(maybe_input)?;
            Ok((rest, Some(*origin)))
        }
        _ => Ok((input, None)),
    }
}

fn parse_optional_where(input: &str) -> ParsingResult<&str, Option<Condition>> {
    let (maybe_input, tok) = scan_token(input)?;
    match tok {
        Token::Other(word) if word.eq_ignore_ascii_case(WHERE_SIGN) => {
            let (rest, cond) = parse_where(maybe_input)?;
            Ok((rest, Some(*cond)))
        }
        _ => Ok((input, None)),
    }
}
