use crate::{
    ParsingResult, general_struct::structure::Condition,
    parsing::other_parser::logic_parser::func::parse_logical,
};

pub fn parse_where(input: &str) -> ParsingResult<&str, Box<Condition>> {
    parse_logical(input)
}
