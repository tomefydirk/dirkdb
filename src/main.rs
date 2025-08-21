use crate::parsing::logic_parser::func::parse_logical;
//use std::collections::HashMap;

mod general_const;
mod general_struct;
mod tokenizer;
mod parsing;
mod evaluation;
fn main() {
    // -------------------
    // Jeu de données
    // -------------------
    /*   let mut ctx = HashMap::new();
       ctx.insert("age".to_string(), TableCell::Null);
       ctx.insert("nom".to_string(), TableCell::String("Alice".to_string()));
       ctx.insert("ville".to_string(), TableCell::String("Paris".to_string()));
    */
    // -------------------
    // Chaîne de condition
    // -------------------
    let input = "1*2%1*2 != (NULL and 1)";

    //
    // -------------------
    // Parsing
    // -------------------
    let (a, cond) = parse_logical(input).expect("Erreur de parsing");

    println!("{:?}", a);

    println!("AST = {:?}", cond);

    // println!("{:?}",tokenizer::is_terminal("a", &["A"]));
}
