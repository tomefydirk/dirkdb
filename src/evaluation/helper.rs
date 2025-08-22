use crate::{
    evaluation::utils::bool_transform,
    general_struct::element::{EvalElement, TableCell},
};

impl From<bool> for EvalElement {
    fn from(value: bool) -> Self {
        EvalElement::Boolean(value)
    }
}
impl From<String> for EvalElement {
    fn from(value: String) -> Self {
        EvalElement::Other(value.into())
    }
}
impl From<f64> for EvalElement {
    fn from(value: f64) -> Self {
        EvalElement::Other(value.into())
    }
}

impl From<TableCell> for bool {
    fn from(value: TableCell) -> Self {
        match value {
            TableCell::String(a) => !a.is_empty(),
            TableCell::Number(n) => n != 0.0,
            TableCell::Null => false,
        }
    }
}
impl From<EvalElement> for bool {
    fn from(value: EvalElement) -> Self {
        match value {
            EvalElement::Boolean(b) => b,
            EvalElement::Other(t) => t.into(),
        }
    }
}

impl Default for EvalElement {
    fn default() -> Self {
        EvalElement::Other(TableCell::Null)
    }
}

impl From<EvalElement> for TableCell {
    fn from(value: EvalElement) -> Self {
        match value {
            EvalElement::Boolean(b) => TableCell::Number(bool_transform(b)),
            EvalElement::Other(table_cell) => table_cell,
        }
    }
}

impl TableCell {
    
    pub fn to_string_value(&self) -> String {
        match self {
            TableCell::Number(n) => n.to_string(),
            TableCell::String(s) => s.clone(),
            TableCell::Null => "NULL".to_string(),
        }
    }
}