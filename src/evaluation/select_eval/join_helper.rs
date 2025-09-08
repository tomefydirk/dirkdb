use crate::evaluation::select_eval::context::CtxSELECT;
use crate::evaluation::{EvaluableAsQuery, LgResult};
use crate::general_struct::structure::{
    Condition, JoinElement, JoinOp, Table, TableAliasMap, TableRow,
};

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
impl JoinElement {
    pub fn apply(&self, origin_table: &Table, ctx: &CtxSELECT) -> LgResult<Table> {
        match self.op {
            JoinOp::Full => todo!(),
            JoinOp::Inner => {
                let to_join=ctx.get_table(&self.table.get_name())?;
                inner_join(origin_table,to_join, &self.on_condition, &ctx.alias)
            },
            JoinOp::Left => todo!(),
            JoinOp::Right => todo!(),
        }
    }
}
