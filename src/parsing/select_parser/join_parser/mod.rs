use crate::{
    ParsingResult,
    error_lib::parsing::{into_nom_failure, token_wrong_place},
    general_struct::{
        constant::{JOIN, ON_SIGN, full_join, inner_join, left_join, right_join},
        structure::*,
    },
    parsing::select_parser::{from_parser::parse_from, where_parser::parse_where},
    tokenizer::{
        Token,
        helper::{Tokenizable, codon_stop},
        scan_token,
    },
};

pub fn parse_joins(mut input: &str) -> ParsingResult<&str, Vec<JoinElement>> {
    let mut retour = Vec::new();
    while !codon_stop(input) {
        if !input.starts_with_join_op() {
            break;
        }
        let (current_input, join) = parse_single_join(input)?;
        retour.push(join);
        input = current_input;
    }
    Ok((input, retour))
}

pub fn parse_single_join(input: &str) -> ParsingResult<&str, JoinElement> {
    let (input, token) = scan_token(input)?;
    let op = match token {
        Token::Mkw(mkw) if mkw == inner_join() => JoinOp::Inner,
        Token::Mkw(mkw) if mkw == left_join() => JoinOp::Left,
        Token::Mkw(mkw) if mkw == right_join() => JoinOp::Right,
        Token::Mkw(mkw) if mkw == full_join() => JoinOp::Full,
        Token::Other(word) if word.eq_ignore_ascii_case(JOIN) => JoinOp::Inner,
        a => {
            return Err(into_nom_failure(token_wrong_place(a.to_string())));
        }
    };

    let (input, table) = parse_from(input)?;

    let (input, cond) = parse_on(input)?;

    Ok((input, JoinElement::new(op, *table, cond)))
}

pub fn parse_on(input: &str) -> ParsingResult<&str, Option<Condition>> {
    let (new_input, token) = scan_token(input)?;
    match token {
        Token::Other(word) if word.eq_ignore_ascii_case(ON_SIGN) => {
            let a = parse_where(new_input)?;
            Ok((a.0, Some(*a.1)))
        }
        _ => Ok((input, None)),
    }
}
#[test]
fn test_parse_join_non() {
    let sql = "Left JOIN employee";
    let (rest, join) = parse_single_join(sql).unwrap();

    assert!(rest.trim().is_empty());
    assert_eq!(join.op, JoinOp::Left);
    assert_eq!(join.table.alias, None);
    assert_eq!(join.on_condition, None);
}
#[test]
fn test_parse_join_some() {
    let sql = "INNER JOIN employee AS e ON e.dept_id = dept.id";
    println!("{:#?}", parse_single_join(sql));
    let (rest, join) = parse_single_join(sql).unwrap();

    assert!(rest.trim().is_empty());

    assert_eq!(join.op, JoinOp::Inner);
    assert_eq!(join.table.alias, Some("e".to_string()));

    // Vérifier la condition
    match join.on_condition {
        Some(Condition::Comparison {
            ref left,
            ref op,
            ref right,
        }) => {
            assert_eq!(*op, CompareOp::Eq);

            // gauche = e.dept_id
            match **left {
                Condition::Primitive(PrimitiveElement::Identifier(ref qid)) => {
                    assert_eq!(qid.src.as_ref().unwrap(), "e");
                    assert_eq!(qid.name, "dept_id");
                }
                _ => panic!("Mauvaise condition gauche"),
            }

            // droite = dept.id
            match **right {
                Condition::Primitive(PrimitiveElement::Identifier(ref qid)) => {
                    assert_eq!(qid.src.as_ref().unwrap(), "dept");
                    assert_eq!(qid.name, "id");
                }
                _ => panic!("Mauvaise condition droite"),
            }
        }
        _ => panic!("Condition de JOIN mal parsée"),
    }
}
