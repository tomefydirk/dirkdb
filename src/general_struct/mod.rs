use crate::general_struct::structure::{
    BinOp, CompareOp, Condition, Field, FieldRqst, LogicalOp, PrimitiveElement,
    QualifiedIdentifier, SelectRqst, TableCell, TableWithAlias,
};

pub mod constant;
pub mod structure;
//IMPLEMENTATION :

//FROM for PrimitiveElement
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
    pub fn from_id(value: QualifiedIdentifier) -> Self {
        PrimitiveElement::Identifier(value)
    }
}
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

impl Field {
    pub fn new(expr: Condition, default_name: QualifiedIdentifier) -> Self {
        Field {
            expr,
            default_name,
            alias: None,
        }
    }

    pub fn with_alias(expr: Condition, default_name: QualifiedIdentifier, alias: String) -> Self {
        Field {
            expr,
            default_name,
            alias: Some(alias),
        }
    }
}

pub fn ident(column: &str) -> Condition {
    Condition::Primitive(PrimitiveElement::Identifier(QualifiedIdentifier {
        src: None,
        name: column.to_string(),
    }))
}

impl SelectRqst {
    pub fn new(
        fields: FieldRqst,
        from: Option<TableWithAlias>,
        condition: Option<Condition>,
    ) -> Self {
        Self {
            fields,
            from,
            condition,
        }
    }
}

//DISPLAY :
use std::{fmt};

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::Comparison { left, op, right } => {
                write!(f, "{} {} {}", left, op, right)
            }
            Condition::Logical { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Condition::BinaryOp { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Condition::Negate(inner) => {
                write!(f, "-{}", inner)
            }
            Condition::Not(inner) => {
                write!(f, "NOT ({})", inner)
            }
            Condition::Primitive(prim) => {
                write!(f, "{}", prim)
            }
            Condition::Func { name, parameter } => {
                let params = parameter
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{}({})", name, params)
            }
            Condition::Null => {
                write!(f, "NULL")
            }
        }
    }
}

impl fmt::Display for PrimitiveElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveElement::Identifier(qid) => {
                if let Some(table) = &qid.src {
                    write!(f, "{}.{}", table, qid.name)
                } else {
                    write!(f, "{}", qid.name)
                }
            }
            PrimitiveElement::Number(n) => write!(f, "{}", n),
            PrimitiveElement::String(s) => write!(f, "'{}'", s),
        }
    }
}
impl fmt::Display for CompareOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CompareOp::Eq => "=",
            CompareOp::Neq => "!=",
            CompareOp::Gt => ">",
            CompareOp::Gte => ">=",
            CompareOp::Lt => "<",
            CompareOp::Lte => "<=",
            CompareOp::Is => "is",
            CompareOp::IsNot => "is not",
            CompareOp::Like => "like",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for LogicalOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogicalOp::And => "AND",
            LogicalOp::Or => "OR",
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Pow => "^",
            BinOp::Mod => "%",
        };
        write!(f, "{}", s)
    }
}
impl fmt::Display for QualifiedIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.src {
            Some(t) => write!(f, "{}.{}", t, self.name),
            None => write!(f, "{}", self.name),
        }
    }
}

impl From<String> for QualifiedIdentifier{
    fn from(value: String) -> Self {
          QualifiedIdentifier::new(None, value)      
    }
}

impl From<&str> for QualifiedIdentifier{
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}