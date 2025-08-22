use crate::general_struct::element::{CompareOp, TableCell};

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
pub trait Comparator<T>
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool;
}

impl<T> Comparator<T> for CompareOp
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool {
        match self {
            CompareOp::Eq => l == r,
            CompareOp::Neq => l != r,
            CompareOp::Lt => l < r,
            CompareOp::Lte => l <= r,
            CompareOp::Gt => l > r,
            CompareOp::Gte => l >= r,
            CompareOp::Like=>todo!(),
            _ => false,
        }
    }
}

//transformer bool ---> 0 ou 1

///pour accépter les (1!=1)+1  c'est égal à false+1  or false=0 donc le resultat est 1
pub fn bool_transform(b: bool) -> f64 {
    if b { 1.0 } else { 0.0 }
}
