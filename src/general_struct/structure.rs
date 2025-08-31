use std::collections::HashMap;

use chrono::NaiveDate;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedIdentifier {
    pub table: Option<String>,
    pub column: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveElement {
    Identifier(QualifiedIdentifier),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum LogicalOp {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone)]
pub struct TableWithAlias {
    pub origin: TableOrigin,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub enum TableOrigin {
    Name(String),
    SubRequest(Box<SelectRqst>),
}

#[derive(Debug, Clone)]
pub struct SelectRqst {
    pub fields: FieldRqst,
    pub from: Option<TableWithAlias>,
    pub condition: Option<Condition>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub expr: Condition,
    pub default_name:QualifiedIdentifier,
    pub alias: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FieldRqst {
    All,
    Selected(Vec<Field>),
}

pub enum TableSourceCtxEval{
    Name(String),
    Table(Table)
}
pub type Table = Vec<TableRow>;
pub type TableRow=HashMap<QualifiedIdentifier,TableCell>;
pub type TableAliasMap=HashMap<String,TableSourceCtxEval>;
