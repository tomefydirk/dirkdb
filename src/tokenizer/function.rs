use nom::{
    branch::alt, bytes::complete::{tag, tag_no_case}, Parser
};

use crate::{general_const::PARENS_0, tokenizer::Token, IResult};

pub fn tag_name_fn<'a>(keyword: &'static str) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (mut new_input, matched) = tag_no_case(keyword).parse(input)?;
        (new_input, _) = tag(PARENS_0).parse(new_input.trim_start())?;

        Ok((new_input, matched))
    }
}

pub fn scan_function(input:&str) -> IResult<&str, Token>{
    let a=alt((
        tag_name_fn("sqrt"),
        tag_name_fn("sqrt"),
    )).parse(input)?;
    Ok((a.0, Token::Func(a.1.to_string())))
}