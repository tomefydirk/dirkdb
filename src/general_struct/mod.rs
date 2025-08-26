use crate::general_struct::structure::{
    Condition, Field, PrimitiveElement, QualifiedIdentifier, TableCell,
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
    pub fn new(expr: Condition) -> Self {
        Field { expr, alias: None }
    }

    pub fn with_alias(expr: Condition, alias: String) -> Self {
        Field {
            expr,
            alias: Some(alias),
        }
    }
}

pub fn ident(column: &str) -> Condition {
    Condition::Primitive(PrimitiveElement::Identifier(QualifiedIdentifier {
        table: None,
        column: column.to_string(),
    }))
}
