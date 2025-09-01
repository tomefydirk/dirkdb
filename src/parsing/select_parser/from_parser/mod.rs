use crate::IResult;
use crate::error_lib::parsing::{
    alias_needed_parsing, factor_error, into_nom_error, into_nom_failure,
};
use crate::general_struct::constant::{PARENS_1, SELECT_SIGN};
use crate::general_struct::structure::{TableOrigin, TableWithAlias};
use crate::parsing::select_parser::func::parse_select;
use crate::tokenizer::helper::Factorable;
use crate::tokenizer::{Token, scan_token};

pub fn parse_from(input: &str) -> IResult<&str, Box<TableWithAlias>> {
    let (input, origin) = parse_from_base1(input)?;
    let (input, alias) = parse_optional_alias(input)?;
    match (alias, origin) {
        (None, TableOrigin::SubRequest(_)) => {
            Err(into_nom_error(alias_needed_parsing()))
        }
        (a, t) =>{
           // println!("{a:?}");
            Ok((
            input,
            Box::new(TableWithAlias {
                origin: t,
                alias: a,
            }),
        ))
        } ,
    }
}
fn parse_from_base1(input: &str) -> IResult<&str, TableOrigin> {
    if input.is_factor_parens() {
        let (input, _) = scan_token(input)?;
        let (input, retour) = parse_from_base1(input)?;
        let (input, t) = scan_token(input)?;
        match t {
            Token::Other(PARENS_1) => Ok((input, retour)),
            _ => Err(into_nom_failure(factor_error(input))),
        }
    } else if input.trim_start().starts_with(SELECT_SIGN) {
        let (input, sub_select) = parse_select(input)?;
        Ok((input, TableOrigin::SubRequest(Box::new(sub_select))))
    } else {
        let (input, token) = scan_token(input)?;
        match token {
            Token::FieldName(qid) if qid.table.is_none() => {
                Ok((input, TableOrigin::Name(qid.column)))
            }
            _ => Err(into_nom_failure(factor_error(input))),
        }
    }
}
//ALIAS FUNCTION :

/// Parse un alias explicite : "AS alias"
fn parse_explicit_alias(input: &str) -> IResult<&str, Option<String>> {
    let (next_input, tok) = scan_token(input)?;
    match tok {
        Token::Other(word) if word.eq_ignore_ascii_case("as") => {
            let (after_as, alias_tok) = scan_token(next_input)?;
            match alias_tok {
                Token::FieldName(qid) => Ok((after_as, Some(qid.column))),
                _ => Err(into_nom_failure(factor_error(next_input))),
            }
        }
        _ => Ok((input, None)),
    }
}

/// Parse un alias implicite : "table alias"
fn parse_implicit_alias(input: &str) -> IResult<&str, Option<String>> {
    let (next_input, tok) = scan_token(input)?;
    // println!("{next_input}");
    match tok {
        Token::FieldName(qid) => Ok((next_input, Some(qid.column))),
        _ => Ok((input, None)),
    }
}

/// Détecte un alias optionnel, explicite ou implicite
pub fn parse_optional_alias(input: &str) -> IResult<&str, Option<String>> {
    if let Ok((next_input, alias)) = parse_explicit_alias(input) {
        if alias.is_some() {
            return Ok((next_input, alias));
        }
    }

    if let Ok((next_input, alias)) = parse_implicit_alias(input) {
        if alias.is_some() {
            return Ok((next_input, alias));
        }
    }

    // Sinon pas d'alias
    Ok((input, None))
}
