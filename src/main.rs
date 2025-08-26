use std::collections::HashMap;

use chrono::NaiveDate;
use dirkdb::{
    general_struct::structure::{Condition, TableCell},
    parsing::logic_parser::func::parse_logical,
};

fn main() {
    // 1) Contexte
    let mut ctx = HashMap::new();
    ctx.insert(
        "x".to_string(),
        TableCell::Date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    ctx.insert("y".to_string(), TableCell::Number(2.0));
    ctx.insert("name".to_string(), TableCell::String("rust".into()));
    ctx.insert("test".to_string(), TableCell::String("R%".into()));
    ctx.insert("z".to_string(), TableCell::Null);

    // 2) Phrase à parser
    let input = "n.y";

    // 3) Parsing → Condition
    let (i, cond): (&str, Box<Condition>) = parse_logical(input).unwrap();

    // 4) Évaluation
    println!("{i:?} {cond:?}");
   /*  let result = cond.eval(&ctx);

    println!("{:?}", result);
    println!("{:?}", result);*/
}
