use crate::general_struct::element::{Condition, TableCell};

pub trait SQLfunction : Default {
    fn build(att: Condition);
    fn invoke(&self)->TableCell;
}