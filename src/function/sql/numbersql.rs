use crate::{
    error_lib::evaluation::*, evaluation::LgResult, function::sql::Signature,
    general_struct::element::TableCell,
};
pub fn sqrt(args: Vec<TableCell>) -> LgResult<TableCell> {
    if args.len() != 1 {
        return Err(EvalEror::<String>::function_not_found(Signature::new(
            "sqrt".to_string(),
            args.len(),
        )));
    }

    match &args[0] {
        TableCell::Number(f) => {
            if *f < 0.0 {
                Err(EvalEror::<String>::negative_into_sqrt(*f))
            } else {
                Ok(TableCell::Number(f.sqrt()))
            }
        }
        other => Err(EvalEror::<String>::incompatible_type(other)),
    }
}
