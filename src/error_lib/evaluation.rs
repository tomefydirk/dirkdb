#[derive(Debug,thiserror::Error)]
#[error("Field not found :'0'")]
pub struct FieldNotFoundErr<I>(pub I);


#[derive(Debug, thiserror::Error)]
pub enum EvalEror<I> {
    FieldNotFound(#[from] FieldNotFoundErr<I>)
}