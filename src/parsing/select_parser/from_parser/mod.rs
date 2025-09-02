use crate::IResult;
use crate::error_lib::parsing::{
    alias_needed_parsing, factor_error, into_nom_error, into_nom_failure,
};
use crate::general_struct::constant::{PARENS_1, SELECT_SIGN};
use crate::general_struct::structure::{TableOrigin, TableWithAlias};
use crate::parsing::parse_optional_alias;
use crate::parsing::select_parser::func::parse_select;
use crate::tokenizer::helper::Factorable;
use crate::tokenizer::{Token, scan_token};

pub fn parse_from(input: &str) -> IResult<&str, Box<TableWithAlias>> {
    let (input, origin) = parse_from_base1(input)?;
    let (input, alias) = parse_optional_alias(input)?;
    match (alias, origin) {
        (None, TableOrigin::SubRequest(_)) => Err(into_nom_error(alias_needed_parsing())),
        (a, t) => Ok((
            input,
            Box::new(TableWithAlias {
                origin: t,
                alias: a,
            }),
        )),
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
            Token::Variable(qid) if qid.src.is_none() => {
                Ok((input, TableOrigin::Name(qid.name)))
            }
            _ => Err(into_nom_failure(factor_error(input))),
        }
    }
}
