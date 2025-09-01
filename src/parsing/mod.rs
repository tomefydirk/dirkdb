use crate::{error_lib::parsing::{factor_error, into_nom_failure}, tokenizer::{scan_token, Token}, IResult};

pub mod atom_parser;
pub mod logic_parser;
pub mod select_parser;

//ALIAS FUNCTION :

/// Parse un alias explicite : "AS alias"
pub fn parse_explicit_alias(input: &str) -> IResult<&str, Option<String>> {
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
pub fn parse_implicit_alias(input: &str) -> IResult<&str, Option<String>> {
    let (next_input, tok) = scan_token(input)?;
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