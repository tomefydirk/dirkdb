use crate::general_struct::constant::*;
use crate::general_struct::structure::QualifiedIdentifier;
use crate::{
    ParsingResult,
    general_struct::structure::{BinOp, Condition, PrimitiveElement},
};

pub mod func;

impl Condition {
    //binary operation
    pub fn box_binop_from(
        left_box: Box<Condition>,
        right_box: Box<Condition>,
        operation: BinOp,
    ) -> Box<Condition> {
        Box::new(Condition::BinaryOp {
            left: left_box,
            op: operation,
            right: right_box,
        })
    }

    //factor operation
    pub fn box_factorop_from(current_expr: Box<Condition>, token: &str) -> Box<Condition> {
        match token.to_uppercase().as_str() {
            val if val == MINUS_SIGN.to_uppercase().as_str() => {
                Box::new(Condition::Negate(current_expr))
            }
            val if val == NOT_SIGN.to_ascii_uppercase().as_str() => {
                Box::new(Condition::Not(current_expr))
            }

            a => {
                println!("operateur non trouvé :: {a}");
                Box::new(Condition::Negate(current_expr))
            }
        }
    }

    pub fn result_number(input: &str, number: f64) -> crate::ParsingResult<&str, Box<Condition>> {
        let a: PrimitiveElement = number.into();
        let result = (input, Box::new(Condition::Primitive(a)));
        Ok(result)
    }
    pub fn result_string(input: &str, str: String) -> ParsingResult<&str, Box<Condition>> {
        let a: PrimitiveElement = str.into();
        let result = (input, Box::new(Condition::Primitive(a)));
        ParsingResult::Ok(result)
    }
    pub fn result_name(input: &str, str: QualifiedIdentifier) -> ParsingResult<&str, Box<Condition>> {
        let a: PrimitiveElement = PrimitiveElement::from_id(str);
        let result = (input, Box::new(Condition::Primitive(a)));
        ParsingResult::Ok(result)
    }
    pub fn result_func(
        input: &str,
        f_name: QualifiedIdentifier,
        args: Vec<Condition>,
    ) -> ParsingResult<&str, Box<Condition>> {
        Ok((
            input,
            Box::new(Condition::Func {
                name: f_name,
                parameter: args,
            }),
        ))
    }
    pub fn result_from_current(
        input: &str,
        current_expr: Box<Condition>,
    ) -> ParsingResult<&str, Box<Condition>> {
        ParsingResult::Ok((input, current_expr))
    }

    pub fn is_factor_op(str_token: &str) -> bool {
        str_token.eq_ignore_ascii_case(MINUS_SIGN) || str_token.eq_ignore_ascii_case(NOT_SIGN)
    }
}
