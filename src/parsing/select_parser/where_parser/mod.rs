use crate::{general_struct::structure::Condition, parsing::other_parser::logic_parser::func::parse_logical, IResult};

pub fn parse_where(input:&str)->IResult<&str,Box<Condition>>{
        parse_logical(input)
}