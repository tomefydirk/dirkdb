use crate::parsing::select_parser::field_parser::list_parser::parse_fieldrqst_expr_list;
use crate::tokenizer::helper::Factorable;
use crate::IResult;
use crate::error_lib::parsing::{factor_error, into_nom_failure};
use crate::general_struct::constant::{PARENS_1};
use crate::{
    general_struct::structure::{ FieldRqst},
    tokenizer::{Token, scan_token},
};
pub mod list_parser;

fn parse_fieldrqst_parens(input: &str) -> IResult<&str, FieldRqst> {
    let (input, _) = scan_token(input)?;
    let (input, retour) = parse_fieldrqst(input)?;
    let (input, next_token) = scan_token(input)?;
    match next_token {
        Token::Other(PARENS_1) => Ok((input, retour)),
        _ => Err(into_nom_failure(factor_error(input))),
    }
}

fn parse_fieldrqst_all(input: &str) -> IResult<&str, FieldRqst> {
    let (input, _) = scan_token(input)?;
    Ok((input, FieldRqst::All))
}


pub fn parse_fieldrqst(input: &str) -> IResult<&str, FieldRqst> {
    // Parenthèses
    if input.is_factor_parens(){
        return parse_fieldrqst_parens(input);
    }

    // Champ global "*"
    if input.trim_start().starts_with('*') {
        return parse_fieldrqst_all(input);
    }

    // Liste d'expressions
    parse_fieldrqst_expr_list(input)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::general_struct::{ident, structure::FieldRqst};
    

   

    #[test]
    fn test_parse_all_fields() {
        let input = "*";
        let (rest, rqst) = parse_fieldrqst(input).unwrap();
        assert!(rest.trim().is_empty());
        match rqst {
            FieldRqst::All => {}
            _ => panic!("Expected FieldRqst::All"),
        }
    }

    #[test]
    fn test_parse_single_field() {
        let input = "name";
        let (rest, rqst) = parse_fieldrqst(input).unwrap();
        assert!(rest.trim().is_empty());
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].expr, ident("name"));
                assert_eq!(fields[0].alias, None);
            }
            _ => panic!("Expected FieldRqst::Selected"),
        }
    }

    #[test]
    fn test_parse_multiple_fields() {
        let input = "(name, age, city)";
        let (rest, rqst) = parse_fieldrqst(input).unwrap();
        assert!(rest.trim().is_empty());
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 3);
                assert_eq!(fields[0].expr, ident("name"));
                assert_eq!(fields[1].expr, ident("age"));
                assert_eq!(fields[2].expr, ident("city"));
            }
            _ => panic!("Expected FieldRqst::Selected"),
        }
    }

    #[test]
    fn test_parse_parenthesized_field() {
        let input = "(name)";
        let (rest, rqst) = parse_fieldrqst(input).unwrap();
        assert!(rest.trim().is_empty());
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].expr, ident("name"));
            }
            _ => panic!("Expected FieldRqst::Selected"),
        }
    }
      #[test]
    fn test_field_with_alias_as() {
        let input = "name AS username";
        let (_, rqst) = parse_fieldrqst_expr_list(input).unwrap();
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].expr, ident("name"));
                assert_eq!(fields[0].alias.as_deref(), Some("username"));
            }
            _ => panic!("Expected Selected"),
        }
    }

    #[test]
    fn test_field_with_alias_implicit() {
        let input = "name username";
        let (_, rqst) = parse_fieldrqst_expr_list(input).unwrap();
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].expr, ident("name"));
                assert_eq!(fields[0].alias.as_deref(), Some("username"));
            }
            _ => panic!("Expected Selected"),
        }
    }

    #[test]
    fn test_multiple_fields_with_alias() {
        let input = "name AS username, age years";
        let (_, rqst) = parse_fieldrqst_expr_list(input).unwrap();
        match rqst {
            FieldRqst::Selected(fields) => {
                assert_eq!(fields.len(), 2);

                assert_eq!(fields[0].expr, ident("name"));
                assert_eq!(fields[0].alias.as_deref(), Some("username"));

                assert_eq!(fields[1].expr, ident("age"));
                assert_eq!(fields[1].alias.as_deref(), Some("years"));
            }
            _ => panic!("Expected Selected"),
        }
    }

}
