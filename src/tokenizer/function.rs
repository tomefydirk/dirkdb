use nom::{
    Parser,
    bytes::complete::{tag, tag_no_case},
};

use crate::{IResult, general_const::PARENS_0};

pub fn tag_name<'a>(keyword: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (mut new_input, matched) = tag_no_case(keyword).parse(input)?;
        (new_input, _) = tag(PARENS_0).parse(new_input.trim_start())?;

        Ok((new_input, matched))
    }
}
