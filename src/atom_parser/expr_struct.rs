use nom::IResult;

use crate::{
    atom_parser::expr_constant::{ADD_SIGN, DIV_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN},
    general_struct::PrimitiveElement,
    logic_parser::{cond_constant::NOT_SIGN, cond_source::Condition},
};

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}
impl BinOp {
    pub fn from_str(a: &str) -> Self {
        match a {
            ADD_SIGN => BinOp::Add,
            MINUS_SIGN => BinOp::Sub,
            MUL_SIGN => BinOp::Mul,
            DIV_SIGN => BinOp::Div,
            POWER_SIGN => BinOp::Pow,
            _ => BinOp::Add,
        }
    }
}

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

    pub fn result_number(input: &str, number: f64) -> IResult<&str, Box<Condition>> {
        let a: PrimitiveElement = number.into();
        let result = (input, Box::new(Condition::Primitive(a)));
        IResult::Ok(result)
    }
    pub fn result_string(input: &str, str: String) -> IResult<&str, Box<Condition>> {
        let a: PrimitiveElement = str.into();
        let result = (input, Box::new(Condition::Primitive(a)));
        IResult::Ok(result)
    }
    pub fn result_name(input: &str, str: String) -> IResult<&str, Box<Condition>> {
        let a: PrimitiveElement = PrimitiveElement::from_id(str);
        let result = (input, Box::new(Condition::Primitive(a)));
        IResult::Ok(result)
    }

    pub fn result_from_current(
        input: &str,
        current_expr: Box<Condition>,
    ) -> IResult<&str, Box<Condition>> {
        IResult::Ok((input, current_expr))
    }
    pub fn is_factor_op(str_token: &str) -> bool {
        str_token.eq_ignore_ascii_case(MINUS_SIGN) || str_token.eq_ignore_ascii_case(NOT_SIGN)
    }
}
