use crate::tokenizer::helper::Factorable;
use crate::tokenizer::{scan_token};
use crate::IResult;

use crate::general_struct::structure::{ TableWithAlias};

pub fn parse_from(input:&str)->IResult<&str,Box<TableWithAlias>>{
        if input.is_factor_parens(){
            let (input,_)=scan_token(input)?;
            parse_from(input)
        }else {
            todo!()
        }
}