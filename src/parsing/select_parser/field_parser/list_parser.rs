use crate::{
    IResult,
    error_lib::parsing::{alias_not_valid, factor_error, into_nom_failure},
    general_struct::{
        constant::{AS_SIGN, COMMA_SIGN},
        structure::{Condition, Field, FieldRqst, PrimitiveElement, QualifiedIdentifier},
    },
    parsing::other_parser::logic_parser::func::parse_logical,
    tokenizer::{Token, scan_token},
};
impl Field {
    pub fn apply_alias(&mut self, alias: QualifiedIdentifier) -> bool {
        if self.alias.is_some() || alias.src.is_some() {
            false
        } else {
            self.alias = Some(alias.name);
            true
        }
    }
}
pub fn parse_fieldrqst_expr_list(input: &str) -> IResult<&str, FieldRqst> {
    let (mut input, first_expr) = parse_logical(input)?;
    let mut fields = vec![Field::build_field(*first_expr)];

    while let Ok((next_input, next_token)) = scan_token(input) {
        match next_token {
            Token::Other(COMMA_SIGN) => {
                // après une virgule : nouvelle expression
                let (after_expr, expr) = parse_logical(next_input)?;
                fields.push(Field::build_field(*expr));
                input = after_expr;
            }

            // alias implicite : SELECT name username
            Token::Variable(alias) => {
                if let Some(last) = fields.last_mut() {
                    if !last.apply_alias(alias) {
                        return Err(into_nom_failure(alias_not_valid(next_input)));
                    }
                }
                input = next_input;
            }

            // alias explicite avec AS : SELECT name AS username
            Token::Other(a) if a.eq_ignore_ascii_case(AS_SIGN) => {
                let (after_as, alias_token) = scan_token(next_input)?;
                match alias_token {
                    Token::Variable(alias) => {
                        if let Some(last) = fields.last_mut() {
                            if !last.apply_alias(alias) {
                                return Err(into_nom_failure(alias_not_valid(next_input)));
                            }
                        }
                        input = after_as;
                    }
                    _ => return Err(into_nom_failure(factor_error(next_input))),
                }
            }

            _ => break,
        }
    }

    Ok((input, FieldRqst::Selected(fields)))
}

impl Field {
    fn build_field(expr: Condition) -> Self {
        match &expr.clone() {
            Condition::Primitive(PrimitiveElement::Identifier(qid)) => {
                Field::new(expr, qid.clone())
            }

            _ => {
                let default_name = expr.to_string();
                Field::new(expr, QualifiedIdentifier::new(None, default_name))
            }
        }
    }
}
