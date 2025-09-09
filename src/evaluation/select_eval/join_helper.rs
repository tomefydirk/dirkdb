use crate::evaluation::select_eval::context::CtxSELECT;
use crate::evaluation::{EvaluableAsQuery, JoinOpperand, LgResult};
use crate::general_struct::structure::{
    Condition, JoinElement, JoinOp, Table, TableAliasMap, TableCell, TableRow,
};

pub fn inner_join(
    t1: &Table,
    t2: &Table,
    cond: &Option<Condition>,
    aliases: &TableAliasMap,
) -> LgResult<Table> {
    let mut result: Table = Vec::new();

    for row1 in t1 {
        for row2 in t2 {
            let mut combined: TableRow = row1.clone();
            for (k, v) in row2 {
                combined.insert(k.clone(), v.clone());
            }
            match cond {
                Some(on) => match on.eval_dyn(&combined, aliases) {
                    Ok(val) if val.as_bool() => result.push(combined),
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                None => result.push(combined),
            }
        }
    }

    Ok(result)
}
pub fn left_join(
    t1: &Table,
    t2: &Table,
    cond: &Option<Condition>,
    aliases: &TableAliasMap,
) -> LgResult<Table> {
    let mut result: Table = Vec::new();

    for row1 in t1 {
        let mut any_match = false;

        for row2 in t2 {
            let mut combined: TableRow = row1.clone();
            for (k, v) in row2 {
                combined.insert(k.clone(), v.clone());
            }

            match cond {
                Some(on) => match on.eval_dyn(&combined, aliases) {
                    Ok(val) if val.as_bool() => {
                        result.push(combined);
                        any_match = true;
                    }
                    Ok(_) => {}
                    Err(e) => return Err(e),
                },
                None => {
                    result.push(combined);
                    any_match = true;
                }
            }
        }

        if !any_match {
            let mut combined: TableRow = row1.clone();

            if let Some(first_row_t2) = t2.first() {
                for key in first_row_t2.keys() {
                    if !combined.contains_key(key) {
                        combined.insert(key.clone(), TableCell::Null);
                    }
                }
            }

            result.push(combined);
        }
    }

    Ok(result)
}
pub fn right_join(
    t1: &Table,
    t2: &Table,
    cond: &Option<Condition>,
    aliases: &TableAliasMap,
) -> LgResult<Table> {
    left_join(t2, t1, cond, aliases)
}
impl JoinOpperand for JoinElement {
    fn apply_as_join(&self, origin_table: Box<Table>, ctx: &CtxSELECT) -> LgResult<Table> {
        let to_join = ctx.get_table(&self.table.get_name())?;
        match self.op {
            JoinOp::Full => todo!(),
            JoinOp::Inner => inner_join(
                origin_table.as_ref(),
                to_join,
                &self.on_condition,
                &ctx.alias,
            ),
            JoinOp::Left => left_join(
                origin_table.as_ref(),
                to_join,
                &self.on_condition,
                &ctx.alias,
            ),
            JoinOp::Right => right_join(
                origin_table.as_ref(),
                to_join,
                &self.on_condition,
                &ctx.alias,
            ),
        }
    }
}
impl JoinOpperand for Vec<JoinElement> {
    fn apply_as_join(&self, mut origin_table: Box<Table>, ctx: &CtxSELECT) -> LgResult<Table> {
        for j in self.iter() {
            origin_table = Box::new(j.apply_as_join(origin_table, ctx)?);
        }
        Ok(*origin_table)
    }
}
