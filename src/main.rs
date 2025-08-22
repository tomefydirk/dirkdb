use std::collections::HashMap;

use dirkdb::{
    general_struct::element::{Condition, TableCell},
    parsing::logic_parser::func::parse_logical,
};

fn main() {
    // 1) Contexte
    let mut ctx = HashMap::new();
    ctx.insert("x".to_string(), TableCell::Number(10.0));
    ctx.insert("y".to_string(), TableCell::Number(2.0));
    ctx.insert("name".to_string(), TableCell::String("rust".into()));
    ctx.insert("z".to_string(), TableCell::Null);

    // 2) Phrase à parser
    let input = "z is   not null";

    // 3) Parsing → Condition
    let (_, cond): (&str, Box<Condition>) = parse_logical(input).unwrap();

    // 4) Évaluation
    println!("{cond:?}");
    let result = cond.eval(&ctx);

    println!("{:?}", result);
}
