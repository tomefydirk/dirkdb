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
            _ => false,
        }
    }
}



