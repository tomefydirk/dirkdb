use crate::{
    IResult, general_struct::structure::Condition,
    parsing::other_parser::logic_parser::func::parse_logical,
};

pub fn parse_where(input: &str) -> IResult<&str, Box<Condition>> {
    parse_logical(input)
}
