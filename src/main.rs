use std::collections::HashMap;

use dirkdb::{evaluation::{select_eval::{FieldEval, Table}, LgResult}, general_struct::structure::{Condition, Field, PrimitiveElement, QualifiedIdentifier, TableCell}};

fn main() -> LgResult<()> {
    let mut row1 = HashMap::new();
    row1.insert("id".into(), TableCell::Number(1.0));
    row1.insert("name".into(), TableCell::String("Alice".into()));
    let mut row2 = HashMap::new();
    row2.insert("id".into(), TableCell::Number(2.0));
    row2.insert("name".into(), TableCell::String("Bob".into()));

    let table: Table = vec![row1, row2];

 
    let id=PrimitiveElement::from_id(QualifiedIdentifier{table:None,column:"id".to_string()});

    let name=PrimitiveElement::from_id(QualifiedIdentifier{table:None,column:"name".to_string()});


    let fields = vec![
        Field {
            expr: Condition::Primitive(id),
            default_name: "id".into(),
            alias: None,
        },
        Field {
            expr: Condition::Primitive(name),
            default_name: "name".into(),
            alias: Some("username".into()),
        },
    ];

    let projected = fields.eval(&table)?;
    println!("{:#?}", projected);

    Ok(())
}
