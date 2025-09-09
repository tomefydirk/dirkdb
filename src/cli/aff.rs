use std::fmt;

use crate::general_struct::structure::{Table, TableCell, TableRow};
use crate::general_struct::structure::QualifiedIdentifier;
impl fmt::Display for TableCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TableCell::Number(n) => write!(f, "{}", n),
            TableCell::String(s) => write!(f, "'{}'", s),
            TableCell::Date(d) => write!(f, "{}", d),
            TableCell::Null => write!(f, "NULL"),
        }
    }
}

// Newtype wrappers
pub struct PrettyTable<'a>(pub &'a Table);
pub struct PrettyRow<'a>(pub &'a TableRow);

impl fmt::Display for PrettyTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table = self.0;

        if table.is_empty() {
            return writeln!(f, "Empty table");
        }

       
        let mut headers: Vec<&QualifiedIdentifier> = Vec::new();
        if let Some(first_row) = table.first() {
            headers.extend(first_row.keys());
        }

       
        let mut rows_str: Vec<Vec<String>> = Vec::new();
        for row in table {
            let mut vals = Vec::new();
            for h in &headers {
                match row.get(h) {
                    Some(v) => vals.push(format!("{}", v)),
                    None => vals.push("NULL".to_string()),
                }
            }
            rows_str.push(vals);
        }

       
        let widths: Vec<usize> = headers
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let header_len = h.name.len();
                let col_max = rows_str.iter().map(|r| r[i].len()).max().unwrap_or(0);
                header_len.max(col_max)
            })
            .collect();

       
        let print_sep = |f: &mut fmt::Formatter<'_>| -> fmt::Result {
            write!(f, "+")?;
            for w in &widths {
                write!(f, "{}+", "-".repeat(*w + 2))?;
            }
            writeln!(f)
        };

      
        print_sep(f)?;
        write!(f, "|")?;
        for (h, w) in headers.iter().zip(&widths) {
            write!(f, " {:<width$} |", h.name, width = *w)?;
        }
        writeln!(f)?;
        print_sep(f)?;

       
        for row in rows_str {
            write!(f, "|")?;
            for (val, w) in row.iter().zip(&widths) {
                write!(f, " {:<width$} |", val, width = *w)?;
            }
            writeln!(f)?;
        }
        print_sep(f)?;

        Ok(())
    }
}
