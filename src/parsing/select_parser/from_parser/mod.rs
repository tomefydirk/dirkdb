use crate::ParsingResult;
use crate::error_lib::parsing::{
    alias_needed_parsing, factor_error, into_nom_error, into_nom_failure, token_not_found,
};
use crate::general_struct::constant::{PARENS_1, SELECT_SIGN};
use crate::general_struct::structure::{TableOrigin, TableWithAlias};
use crate::parsing::parse_optional_alias;
use crate::parsing::select_parser::func::parse_select;
use crate::tokenizer::helper::Factorable;
use crate::tokenizer::{Token, scan_token};

pub fn parse_from(input: &str) -> ParsingResult<&str, Box<TableWithAlias>> {
    let (input, origin) = parse_from_base1(input)?;
    let (input, alias) = parse_optional_alias(input)?;
    match (alias, origin) {
        (None, TableOrigin::SubRequest{ rqst: _, id:_ }) => Err(into_nom_error(alias_needed_parsing())),
        (a, t) => Ok((
            input,
            Box::new(TableWithAlias {
                origin: t,
                alias: a,
            }),
        )),
    }
}
fn parse_from_base1(input: &str) -> ParsingResult<&str, TableOrigin> {
    if input.is_factor_parens() {
        let (input, _) = scan_token(input)?;
        let (input, retour) = parse_from_base1(input)?;
        let (input, t) = scan_token(input)?;
        match t {
            Token::Other(PARENS_1) => Ok((input, retour)),
            a => Err(into_nom_failure(factor_error(a.to_string()))),
        }
    } else if input.trim_start().starts_with(SELECT_SIGN) {
        let (input, sub_select) = parse_select(input)?;
        Ok((input,sub_select.into()))
    } else {
        let old_input = input;
        let (input, token) = scan_token(input)?;
        match token {
            Token::Variable(qid) if qid.src.is_none() => Ok((input, TableOrigin::build_as_name(qid.name))),
            a => Err(into_nom_failure(token_not_found(a.to_string()))),
        }
    }
}
