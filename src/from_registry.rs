use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::LgResult,
    general_struct::structure::{QualifiedIdentifier, Table, TableCell, TableWithAlias},
};
use std::collections::HashMap;

pub type Database = HashMap<String, Table>;
pub fn get_tables(id: String, name: &String) -> LgResult<(String,Table)> {
    let g = make_tables();
    if let Some(a) = g.get(name) {
        let b = TableWithAlias::change_table_owner(a.clone(), id.clone())?;
        Ok((id.clone(), b))
    } else {
        Err(EvalEror::<String>::not_in_database(name.clone()))
    }
}
pub fn make_tables() -> Database {
    let mut db: Database = HashMap::new();

    // ---------- Table employee ----------
    let mut employee_table: Table = Vec::new();

    // ligne 1
    let mut row1 = HashMap::new();
    row1.insert(
        QualifiedIdentifier::new(Some("employee".into()), "id".into()),
        TableCell::Number(1.0),
    );
    row1.insert(
        QualifiedIdentifier::new(Some("employee".into()), "nom".into()),
        TableCell::String("Jean".into()),
    );
    employee_table.push(row1);

    // ligne 2
    let mut row2 = HashMap::new();
    row2.insert(
        QualifiedIdentifier::new(Some("employee".into()), "id".into()),
        TableCell::Number(2.0),
    );
    row2.insert(
        QualifiedIdentifier::new(Some("employee".into()), "nom".into()),
        TableCell::String("Alice".into()),
    );
    employee_table.push(row2);

    db.insert("employee".into(), employee_table);

    // ---------- Table boss ----------
    let mut boss_table: Table = Vec::new();

    let mut boss1 = HashMap::new();
    boss1.insert(
        QualifiedIdentifier::new(Some("boss".into()), "id".into()),
        TableCell::Number(1.0),
    );
    boss1.insert(
        QualifiedIdentifier::new(Some("boss".into()), "nom".into()),
        TableCell::String("Patron de Jean".into()),
    );
    boss_table.push(boss1);

    db.insert("boss".into(), boss_table);

    db
}
