use std::collections::HashMap;
use crate::general_struct::structure::{Table, TableCell, QualifiedIdentifier};

pub type Database = HashMap<String, Table>;

pub fn make_tables() -> Database {
    let mut db: Database = HashMap::new();

    // ---------- Table employee ----------
    let mut employee_table: Table = Vec::new();

    // ligne 1
    let mut row1 = HashMap::new();
    row1.insert(
        QualifiedIdentifier { table: Some("employee".into()), column: "id".into() },
        TableCell::Number(1.0),
    );
    row1.insert(
        QualifiedIdentifier { table: Some("employee".into()), column: "nom".into() },
        TableCell::String("Jean".into()),
    );
    employee_table.push(row1);

    // ligne 2
    let mut row2 = HashMap::new();
    row2.insert(
        QualifiedIdentifier { table: Some("employee".into()), column: "id".into() },
        TableCell::Number(2.0),
    );
    row2.insert(
        QualifiedIdentifier { table: Some("employee".into()), column: "nom".into() },
        TableCell::String("Alice".into()),
    );
    employee_table.push(row2);

    db.insert("employee".into(), employee_table);

    // ---------- Table boss ----------
    let mut boss_table: Table = Vec::new();

    let mut boss1 = HashMap::new();
    boss1.insert(
        QualifiedIdentifier { table: Some("boss".into()), column: "id".into() },
        TableCell::Number(1.0),
    );
    boss1.insert(
        QualifiedIdentifier { table: Some("boss".into()), column: "nom".into() },
        TableCell::String("Patron de Jean".into()),
    );
    boss_table.push(boss1);

    db.insert("boss".into(), boss_table);

    db
}
