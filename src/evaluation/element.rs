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
impl Default for TableCell {
    fn default() -> Self {
        Self::Null
    }
}
impl PartialEq for TableCell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}