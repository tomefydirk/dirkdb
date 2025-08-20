use crate::{atom_parser::expr_function::parse_expr, logic_parser::{ cond_source::parse_logical}};
//use std::collections::HashMap;

mod atom_parser;
mod general_const;
mod logic_parser;
mod general_struct;

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
    let input = "not (age is not null) and 'Alice'=nom";

    //
    // -------------------
    // Parsing
    // -------------------
    let (a, cond) = parse_logical(input).expect("Erreur de parsing");

    println!("{:?}", a);
    println!("AST = {:?}", cond);

    

    //other testing :
     let a = "---1";

    let v = parse_expr(a);
  

    match v {
        Ok((rest, expr)) => {
            println!("Expr : {:?}", expr);
            let result = expr.eval();
            println!("Result : {}", result);
            if !rest.is_empty() {
                println!("input_reste : \"{rest}\"");
            }
        }
        Err(_) => {
            println!("Parsing impossible")
        }
    }

}
