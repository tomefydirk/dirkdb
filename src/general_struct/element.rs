

#[derive(Debug, Clone)]
pub enum PrimitiveElement {
    Identifier(String),
    Number(f64),
    String(String),
}

impl From<f64> for PrimitiveElement {
    fn from(value: f64) -> Self {
        PrimitiveElement::Number(value)
    }
}
impl From<String> for PrimitiveElement {
    fn from(value: String) -> Self {
        PrimitiveElement::String(value)
    }
}
impl PrimitiveElement {
    pub fn from_id(value: String) -> Self {
        PrimitiveElement::Identifier(value)
    }
}



#[derive(Debug, Clone)]
pub enum Condition {
    Comparison {
        left: Box<Condition>,
        op: CompareOp,
        right: Box<Condition>,
    },
    Logical {
        left: Box<Condition>,
        op: LogicalOp,
        right: Box<Condition>,
    },
    BinaryOp {
        left: Box<Condition>,
        op: BinOp,
        right: Box<Condition>,
    },
    Negate(Box<Condition>),
    Not(Box<Condition>),
    Primitive(PrimitiveElement),
    Null,
}

impl Default for Condition {
    fn default() -> Self {
        Self::Null
    }
}

#[derive(Debug, Clone)]
pub enum CompareOp {
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    Is,
    IsNot,
}

#[derive(Debug, Clone)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}
#[derive(Debug, Clone)]
pub enum TableCell {
    String(String),
    Number(f64),
    Null,
}

#[derive(Debug, Clone)]
pub enum LogicResult {
    Boolean(bool),
    Other(TableCell),
}