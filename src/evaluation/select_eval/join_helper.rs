use crate::general_struct::structure::{Condition, Table, TableAliasMap, TableRow};
use crate::evaluation::{EvaluableAsQuery, LgResult};

pub fn inner_join(
    t1: &Table,
    t2: &Table,
    cond: &Condition,
    aliases: &TableAliasMap,
) -> LgResult<Table> {
    let mut result: Table = Vec::new();

    for row1 in t1 {
        for row2 in t2 {

            let mut combined: TableRow = row1.clone();
            for (k, v) in row2 {
                combined.insert(k.clone(), v.clone());
            }

            match cond.eval_dyn(&combined, aliases) {
                Ok(val) if val.as_bool() => result.push(combined),
                Ok(_) => {} 
                Err(e) => return Err(e),
            }
        }
    }

    Ok(result)
}

