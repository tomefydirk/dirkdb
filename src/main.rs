use crate::{logic_parser::func::parse_logical};
//use std::collections::HashMap;

mod atom_parser;
mod general_const;
mod logic_parser;
mod general_struct;
mod tokenizer;

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
    let input = "(name is Null)+1*2";

    //
    // -------------------
    // Parsing
    // -------------------
    let (a, cond) = parse_logical(input).expect("Erreur de parsing");

    println!("{:?}", a);

    println!("AST = {:?}", cond);

    // println!("{:?}",tokenizer::is_terminal("a", &["A"]));
}
