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

//IMPLEMENTATION :

//DEFAULT for CONDITION :
impl Default for Condition {
    fn default() -> Self {
        Self::Null
    }
}

//FROM for TABLECELL :
impl From<String> for TableCell {
    fn from(value: String) -> Self {
        TableCell::String(value)
    }
}
impl From<f64> for TableCell {
    fn from(value: f64) -> Self {
        TableCell::Number(value)
    }
}

//DEFAULT for TableCell

impl Default for TableCell {
    fn default() -> Self {
        Self::Null
    }
}

//PartialEq:

impl PartialEq for LogicResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LogicResult::Boolean(a), LogicResult::Boolean(b)) => a == b,
            (LogicResult::Other(a), LogicResult::Other(b)) => a == b,
            _ => false,
        }
    }
}
