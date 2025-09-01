pub mod helper;
pub mod select_eval;
pub type LgResult<T, E = crate::error_lib::evaluation::EvalEror<String>> =
    std::result::Result<T, E>;

pub trait EvaluableAsQuery<Ctx, Aliases, O> {
    fn eval_dyn(&self, ctx: &Ctx, aliases: &Aliases) -> LgResult<O>;
    fn static_eval(&self) -> LgResult<O>;
}

pub trait OperatorQuery<T, O> {
    fn default_apply(&self, left: T, right: T) -> O;
}
