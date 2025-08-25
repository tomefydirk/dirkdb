use crate::general_struct::structure::{CompareOp, Condition, LogicalOp};

pub mod func;

pub trait BuildCondition {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition>
    where
        Self: Sized;
}

impl BuildCondition for CompareOp {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition> {
        Box::new(Condition::Comparison { left, op, right })
    }
}
impl BuildCondition for LogicalOp {
    fn build(left: Box<Condition>, right: Box<Condition>, op: Self) -> Box<Condition> {
        Box::new(Condition::Logical { left, op, right })
    }
}
