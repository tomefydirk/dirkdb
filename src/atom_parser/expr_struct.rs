use crate::{atom_parser::expr_constant::{ADD_SIGN, DIV_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN}, general_struct::PrimitiveElement};
use nom::IResult;

#[derive(Debug,Clone)]
pub enum Expr {
    Primitive(PrimitiveElement),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Negate(Box<Expr>),
}

#[derive(Debug,Clone)]
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

impl Expr {
    //binary operation
    pub fn box_binop_from(
        left_box: Box<Expr>,
        right_box: Box<Expr>,
        operation: BinOp,
    ) -> Box<Expr> {
        Box::new(Expr::BinaryOp {
            left: left_box,
            op: operation,
            right: right_box,
        })
    }

    //factor operation
    pub fn box_factorop_from(current_expr: Box<Expr>, token: &str) -> Box<Expr> {
        match token.to_uppercase().as_str() {
            val if val == MINUS_SIGN.to_uppercase().as_str() => {
                Box::new(Expr::Negate(current_expr))
            }
            a => {
                println!("operateur non trouvé :: {a}");
                Box::new(Expr::Negate(current_expr))
            }
        }
    }

    //transfrome un nombre --> result
    pub fn result_number(input: &str, number: f64) -> IResult<&str, Box<Expr>> {
        let a:PrimitiveElement=number.into();
        let result = (input, Box::new(Expr::Primitive(a)));
        IResult::Ok(result)
    }

    pub fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
        IResult::Ok((input, current_expr))
    }
    pub fn is_factor_op(str_token: &str) -> bool {
        str_token.eq_ignore_ascii_case(MINUS_SIGN)
    }
}
