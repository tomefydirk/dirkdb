//binop const
pub const PARENS_0: &str = "(";
pub const PARENS_1: &str = ")";
pub const MINUS_SIGN: &str = "-";
pub const ADD_SIGN: &str = "+";
pub const MUL_SIGN: &str = "*";
pub const DIV_SIGN: &str = "/";
pub const POWER_SIGN: &str = "^";
pub const MOD_SIGN: &str = "%";

//----->logic const ::
pub const EQ_SIGN: &str = "=";
pub const NOT_EQ_SIGN: &str = "!=";
pub const LT_SIGN: &str = "<";
pub const LT_E_SIGN: &str = "<=";
pub const GT_SIGN: &str = ">";
pub const GT_E_SIGN: &str = ">=";

pub const IS_SIGN: &str = "is";
pub const IS_NOT_SIGN: &str = "is not";
pub const OR_SIGN: &str = "or";
pub const NOT_SIGN: &str = "not";
pub const NULL_SIGN: &str = "null";
pub const AND_SIGN: &str = "and";
pub const LIKE_SIGN: &str = "like";
pub const AS_SIGN:&str="as";
//--- virgule et point virugle
pub const COMMA_SIGN: &str = ",";
pub const SEMICOLON_SIGN: &str = ";";

pub fn key_word_list() -> Vec<String> {
    vec![
        IS_SIGN.to_string(),
        IS_NOT_SIGN.to_string(),
        OR_SIGN.to_string(),
        NOT_SIGN.to_string(),
        NULL_SIGN.to_string(),
        AND_SIGN.to_string(),
        LIKE_SIGN.to_string(),
    ]
}
