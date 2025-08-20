use crate::atom_parser::expr_constant::{
    ABS_SIGN, ADD_SIGN, COS_SIGN, DIV_SIGN, LN_SIGN, MINUS_SIGN, MUL_SIGN, POWER_SIGN, SIN_SIGN,
    SQRT_SIGN,
};
use nom::IResult;

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    BinaryOp {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Negate(Box<Expr>),
    Ln(Box<Expr>),
    Sqrt(Box<Expr>),
    Cos(Box<Expr>),
    Sin(Box<Expr>),
    Abs(Box<Expr>),
}

#[derive(Debug)]
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
            val if val == SQRT_SIGN.to_uppercase().as_str() => Box::new(Expr::Sqrt(current_expr)),
            val if val == LN_SIGN.to_uppercase().as_str() => Box::new(Expr::Ln(current_expr)),
            val if val == MINUS_SIGN.to_uppercase().as_str() => {
                Box::new(Expr::Negate(current_expr))
            }
            val if val == COS_SIGN.to_uppercase().as_str() => Box::new(Expr::Cos(current_expr)),
            val if val == SIN_SIGN.to_uppercase().as_str() => Box::new(Expr::Sin(current_expr)),
            val if val == ABS_SIGN.to_uppercase().as_str() => Box::new(Expr::Abs(current_expr)),
            a => {
                println!("operateur non trouvé :: {a}");
                Box::new(Expr::Negate(current_expr))
            }
        }
    }
    pub fn result_number(input: &str, number: f64) -> IResult<&str, Box<Expr>> {
        let result = (input, Box::new(Expr::Number(number)));
        IResult::Ok(result)
    }

    pub fn result_from_current(input: &str, current_expr: Box<Expr>) -> IResult<&str, Box<Expr>> {
        IResult::Ok((input, current_expr))
    }
    pub fn is_factor_op(str_token: &str) -> bool {
        str_token.eq_ignore_ascii_case(MINUS_SIGN)
            || str_token.eq_ignore_ascii_case(SQRT_SIGN)
            || str_token.eq_ignore_ascii_case(LN_SIGN)
            || str_token.eq_ignore_ascii_case(COS_SIGN)
            || str_token.eq_ignore_ascii_case(SIN_SIGN)
            || str_token.eq_ignore_ascii_case(ABS_SIGN)
    }
}
impl Expr {
    /*Expr to float*/
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Number(n) => *n,
            Expr::BinaryOp { left, op, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    BinOp::Add => l + r,
                    BinOp::Sub => l - r,
                    BinOp::Mul => l * r,
                    BinOp::Div => {
                        if r == 0.0 {
                            panic!("Division par zéro !");
                        }
                        l / r
                    }
                    BinOp::Pow => l.powf(r),
                }
            }
            Expr::Negate(expr) => -expr.eval(),
            Expr::Ln(expr) => expr.eval().ln(),
            Expr::Sqrt(expr) => expr.eval().sqrt(),
            Expr::Cos(expr) => expr.eval().cos(),
            Expr::Sin(expr) => expr.eval().sin(),
            Expr::Abs(expr) => expr.eval().abs(),
        }
    }
}
