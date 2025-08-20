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
