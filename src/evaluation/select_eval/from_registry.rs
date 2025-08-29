use std::collections::HashMap;

use crate::{evaluation::select_eval::Table, general_struct::structure::TableCell};

pub type Database = HashMap<String, Table>;
pub fn make_tables() -> Database {
    let mut db: Database = HashMap::new();

    // --------- Table "users" ----------
    let mut row1 = HashMap::new();
    row1.insert("id".into(), TableCell::Number(1.0));
    row1.insert("name".into(), TableCell::String("Alice".into()));

    let mut row2 = HashMap::new();
    row2.insert("id".into(), TableCell::Number(2.0));
    row2.insert("name".into(), TableCell::String("Bob".into()));

    let users: Table = vec![row1, row2];
    db.insert("users".into(), users);

    // --------- Table "employees" ----------
    let mut row3 = HashMap::new();
    row3.insert("id".into(), TableCell::Number(10.0));
    row3.insert("salary".into(), TableCell::Number(3000.0));

    let mut row4 = HashMap::new();
    row4.insert("id".into(), TableCell::Number(11.0));
    row4.insert("salary".into(), TableCell::Number(4000.0));
  
    let employees: Table = vec![row3, row4];
    db.insert("employees".into(), employees);

    db
}