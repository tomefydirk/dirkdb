use crate::{
    error_lib::evaluation::EvalEror,
    evaluation::LgResult,
    general_struct::structure::{
        CompareOp, QualifiedIdentifier, TableAliasMap, TableCell, TableRow,
    },
    tokenizer::{Token, scan_float},
};
use chrono::{Datelike, NaiveDate};
impl PartialEq for TableCell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Null, Self::Null) => true,
            _ => false,
        }
    }
}
pub trait Comparator<T>
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool;
}

impl<T> Comparator<T> for CompareOp
where
    T: PartialEq + PartialOrd,
{
    fn comparing(&self, l: T, r: T) -> bool {
        match self {
            CompareOp::Eq => l == r,
            CompareOp::Neq => l != r,
            CompareOp::Lt => l < r,
            CompareOp::Lte => l <= r,
            CompareOp::Gt => l > r,
            CompareOp::Gte => l >= r,
            _ => false,
        }
    }
}
impl From<TableCell> for bool {
    fn from(value: TableCell) -> Self {
        match value {
            TableCell::String(a) => !a.is_empty(),
            TableCell::Number(n) => n != 0.0,
            TableCell::Date(_) => true,
            TableCell::Null => false,
        }
    }
}
impl From<bool> for TableCell {
    fn from(value: bool) -> Self {
        match value {
            true => TableCell::Number(1.0),
            false => TableCell::Number(0.0),
        }
    }
}
impl TableCell {
    pub fn to_string_value(&self) -> String {
        match self {
            TableCell::Number(n) => n.to_string(),
            TableCell::String(s) => s.clone(),
            TableCell::Null => "NULL".to_string(),
            TableCell::Date(naive_date) => naive_date.to_string(),
        }
    }

    pub fn convert_to_date(&self) -> LgResult<NaiveDate> {
        match self {
            TableCell::String(v) => {
                let a = (v)
                    .parse::<NaiveDate>()
                    .map_err(|_| EvalEror::incorrect_date_value(v.clone()))?;
                Ok(a)
            }
            TableCell::Number(n) => {
                let date_opt = NaiveDate::from_num_days_from_ce_opt(*n as i32);
                match date_opt {
                    Some(date) => Ok(date),
                    None => Err(EvalEror::incorrect_date_value(n.to_string())),
                }
            }
            TableCell::Date(naive_date) => Ok(*naive_date),
            TableCell::Null => Err(EvalEror::incorrect_date_value("NULL".to_string())),
        }
    }
}

impl TableCell {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            TableCell::Number(n) => Some(*n),
            TableCell::String(s) => {
                let g = scan_float(s);
                match g {
                    Ok((_, Token::Number(n))) => Some(n),
                    _ => Some(0.0),
                }
            }
            TableCell::Date(d) => {
                let days = d.num_days_from_ce();
                Some(days as f64)
            }
            _ => None,
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            TableCell::String(s) => !s.is_empty(),
            TableCell::Number(n) => *n != 0.0,
            TableCell::Date(_) => true,
            TableCell::Null => false,
        }
    }
}
use std::hash::{Hash, Hasher};

impl Hash for QualifiedIdentifier {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(ref t) = self.src {
            t.to_lowercase().hash(state);
        }
        self.name.to_lowercase().hash(state);
    }
}
pub trait RowAlias {
    fn get_column(
        &self,
        qid: &QualifiedIdentifier,
        aliases: &TableAliasMap,
    ) -> LgResult<&TableCell>;
}

impl RowAlias for TableRow {
    fn get_column(
        &self,
        qid: &QualifiedIdentifier,
        aliases: &TableAliasMap,
    ) -> LgResult<&TableCell> {
        match &qid.src {
            Some(table_name) => {
                let real_table = aliases.get(table_name).unwrap_or(table_name);

                let normalized = QualifiedIdentifier {
                    src: Some(real_table.clone()),
                    name: qid.name.to_string().clone(),
                };

                match self.get(&normalized) {
                    Some(retour) => Ok(retour),
                    None => {
                        println!("{normalized:?}");
                        Err(EvalEror::<String>::field_notfound(qid.to_string()))
                    }
                }
            }
            None => {
                let mut matches: Vec<&TableCell> = self
                    .iter()
                    .filter_map(|(k, v)| {
                        if k.name == qid.name {
                            Some(v)
                        } else {
                            None
                        }
                    })
                    .collect();

                match matches.len() {
                    0 => Err(EvalEror::<String>::field_notfound(qid.to_string())),
                    1 => Ok(matches.remove(0)),
                    _ => Err(EvalEror::<String>::ambiguous_name(qid.to_string())),
                }
            }
        }
    }
}
