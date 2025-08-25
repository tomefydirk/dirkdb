use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub enum PrimitiveElement {
    Identifier(String),
    Number(f64),
    String(String),
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
    Func {
        name: String,
        parameter: Vec<Condition>,
    },
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
    Like,
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
    Date(NaiveDate),
    Null,
}

///La question la plus importante est :
///    COMMENT GÉRER LES ALIAS ?
///
///
#[derive(Debug, Clone)]
pub enum FieldRqst {
    All,
    Selected(Vec<Condition>),
}
#[derive(Debug, Clone)]
pub enum TableOrigin {
    Name(String),
    SubRequest(Box<SelectRqst>),
}
#[derive(Debug, Clone)]
pub struct SelectRqst {
    fields: FieldRqst,
    from: Option<TableOrigin>,
    condition: Option<Condition>, /*
                                         APRÉS LIMIT ,SORT , JOIN , GROUP BY
                                         Mais cela ne sont pas encore la pripriorité
                                  */
}
