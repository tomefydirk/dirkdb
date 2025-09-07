use nom::{multi::many0, Parser};

use crate::{
    error_lib::parsing::{into_nom_failure, token_not_found}, general_struct::{
        constant::{full_join, inner_join, left_join, right_join},
        structure::{ CompareOp, Condition, JoinElement, JoinOp, PrimitiveElement},
    }, parsing::select_parser::{from_parser::parse_from, where_parser::parse_where}, tokenizer::{scan_token, Token}, IResult
};

pub fn parse_joins(input: &str) -> IResult<&str, Vec<JoinElement>> {
    many0(parse_single_join).parse(input)
}

pub fn parse_single_join(input: &str) -> IResult<&str, JoinElement> {
    let (input, token) = scan_token(input)?;

    let op = match token {
        Token::Mkw(mkw) if mkw == inner_join() => JoinOp::Inner,
        Token::Mkw(mkw) if mkw == left_join() => JoinOp::Left,
        Token::Mkw(mkw) if mkw == right_join() => JoinOp::Right,
        Token::Mkw(mkw) if mkw == full_join() => JoinOp::Full,
        Token::Other(word) if word.eq_ignore_ascii_case("join") => JoinOp::Inner,
        _ => {
            return Err(into_nom_failure(token_not_found(input)));
        }
    };

    let (input, table) = parse_from(input)?;
    let (input, cond) = parse_on(input)?;
    Ok((
        input,
        JoinElement ::new(op, *table, *cond),
    ))
}

pub fn parse_on(input: &str)->IResult<&str,Box<Condition>>{
    let (input, token) = scan_token(input)?;
    match token {
        Token::Other(word) if word.eq_ignore_ascii_case("on") => {
            parse_where(input)
        }
        _ => {
           Err(into_nom_failure(token_not_found(input)))
        }
    }

    
}

#[test]
fn test_parse_join() {
    let sql = "INNER JOIN employee AS e ON e.dept_id = dept.id";
    let (rest, join) = parse_single_join(sql).unwrap();

    assert!(rest.trim().is_empty());

    assert_eq!(join.op, JoinOp::Inner);
    assert_eq!(join.table.alias, Some("e".to_string()));

    // Vérifier la condition
    match join.on_condition {
        Condition::Comparison { ref left, ref op, ref right } => {
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
