use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::LgResult,
    general_struct::structure::{QualifiedIdentifier, Table, TableCell, TableRow, TableWithAlias},
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

    // ---------------------------
    // Table EMPLOYEE
    // ---------------------------
    let mut employee: Table = Vec::new();
    let mut e1: TableRow = HashMap::new();
    e1.insert(QualifiedIdentifier::new(Some("employee".into()), "id".into()), TableCell::Number(1.0));
    e1.insert(QualifiedIdentifier::new(Some("employee".into()), "name".into()), TableCell::String("Jean".into()));
    e1.insert(QualifiedIdentifier::new(Some("employee".into()), "age".into()), TableCell::Number(30.0));
    e1.insert(QualifiedIdentifier::new(Some("employee".into()), "boss_id".into()), TableCell::Number(100.0));
    employee.push(e1);

    let mut e2: TableRow = HashMap::new();
    e2.insert(QualifiedIdentifier::new(Some("employee".into()), "id".into()), TableCell::Number(2.0));
    e2.insert(QualifiedIdentifier::new(Some("employee".into()), "name".into()), TableCell::String("Marie".into()));
    e2.insert(QualifiedIdentifier::new(Some("employee".into()), "age".into()), TableCell::Number(28.0));
    e2.insert(QualifiedIdentifier::new(Some("employee".into()), "boss_id".into()), TableCell::Number(100.0));
    employee.push(e2);

    let mut e3: TableRow = HashMap::new();
    e3.insert(QualifiedIdentifier::new(Some("employee".into()), "id".into()), TableCell::Number(3.0));
    e3.insert(QualifiedIdentifier::new(Some("employee".into()), "name".into()), TableCell::String("Paul".into()));
    e3.insert(QualifiedIdentifier::new(Some("employee".into()), "age".into()), TableCell::Null);
    e3.insert(QualifiedIdentifier::new(Some("employee".into()), "boss_id".into()), TableCell::Number(200.0));
    employee.push(e3);

    let mut e4: TableRow = HashMap::new();
    e4.insert(QualifiedIdentifier::new(Some("employee".into()), "id".into()), TableCell::Number(4.0));
    e4.insert(QualifiedIdentifier::new(Some("employee".into()), "name".into()), TableCell::String("Chloé".into()));
    e4.insert(QualifiedIdentifier::new(Some("employee".into()), "age".into()), TableCell::Number(35.0));
    e4.insert(QualifiedIdentifier::new(Some("employee".into()), "boss_id".into()), TableCell::Null);
    employee.push(e4);

    db.insert("employee".into(), employee);

    // ---------------------------
    // Table BOSS
    // ---------------------------
    let mut boss: Table = Vec::new();

    let mut b1: TableRow = HashMap::new();
    b1.insert(QualifiedIdentifier::new(Some("boss".into()), "id".into()), TableCell::Number(100.0));
    b1.insert(QualifiedIdentifier::new(Some("boss".into()), "name".into()), TableCell::String("Patron A".into()));
    boss.push(b1);

    let mut b2: TableRow = HashMap::new();
    b2.insert(QualifiedIdentifier::new(Some("boss".into()), "id".into()), TableCell::Number(200.0));
    b2.insert(QualifiedIdentifier::new(Some("boss".into()), "name".into()), TableCell::String("Patron B".into()));
    boss.push(b2);

    db.insert("boss".into(), boss);

    db
}
