use crate::general_struct::element::{CompareOp, LogicalOp, TableCell};

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
impl PartialOrd for  TableCell{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self,other) {
            (TableCell::String(a), TableCell::String(b)) => a.partial_cmp(b),
            (TableCell::Number(a), TableCell::Number(b)) => a.partial_cmp(b),
            _=>None,
        }
    }
}
trait Comparator<T>
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
impl LogicalOp {
    pub fn apply(&self, l: bool, r: bool) -> bool {
        match self {
            LogicalOp::And => l && r,
            LogicalOp::Or => l || r,
        }
    }
}
