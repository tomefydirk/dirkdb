use crate::general_struct::element::{Condition, EvalElement, PrimitiveElement, TableCell};

pub mod element;

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
    pub fn from_id(value: String) -> Self {
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

//PartialEq:

impl PartialEq for EvalElement {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EvalElement::Boolean(a), EvalElement::Boolean(b)) => a == b,
            (EvalElement::Other(a), EvalElement::Other(b)) => a == b,
            _ => false,
        }
    }
}
