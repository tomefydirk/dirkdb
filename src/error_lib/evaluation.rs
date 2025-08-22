#[derive(Debug)]
pub enum EvalErrorkind {
    FieldNotFound,
    RegexInvalid
}

#[derive(Debug, thiserror::Error)]
#[error("{code:?} : '{input}' ")]
pub struct  EvalEror<I> {
   pub input:I,
   pub code:EvalErrorkind
}

impl<I> EvalEror<I> {
    pub fn build(input:I,code:EvalErrorkind)->Self{
        Self { input, code }
    }
}
