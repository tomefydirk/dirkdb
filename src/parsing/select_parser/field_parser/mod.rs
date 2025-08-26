use crate::IResult;
use crate::error_lib::parsing::{factor_error, into_nom_failure};
use crate::general_struct::constant::{PARENS_0, PARENS_1};
use crate::parsing::logic_parser::func::parse_logical;
use crate::{
    general_struct::structure::{Field, FieldRqst},
    tokenizer::{Token, scan_token},
};

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

fn parse_fieldrqst_expr_list(input: &str) -> IResult<&str, FieldRqst> {
    let (mut input, first_expr) = parse_logical(input)?;
    let mut fields = vec![Field::new(*first_expr)];

    while let Ok((next_input, next_token)) = scan_token(input) {
        match next_token {
            Token::Other(",") => {
                let (after_expr, expr) = parse_logical(next_input)?;
                fields.push(Field::new(*expr));
                input = after_expr;
            }
            _ => break,
        }
    }

    Ok((input, FieldRqst::Selected(fields)))
}

pub fn parse_fieldrqst(input: &str) -> IResult<&str, FieldRqst> {
    // Parenthèses
    if input.starts_with(PARENS_0) {
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
}
