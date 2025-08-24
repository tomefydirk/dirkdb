use nom::{
    Parser,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{digit1, multispace1},
    combinator::opt,
};

use crate::{
    IResult,
    general_const::{IS_NOT_SIGN, IS_SIGN, NOT_SIGN, PARENS_0},
    tokenizer::helper::{is_func_valid, is_ident_char, is_ident_start},
};

pub fn tag_float(input: &str) -> IResult<&str, f64> {
    let (rest, first_part) = digit1(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    if point.is_some() {
        let (rest3, second_part) = digit1(rest2)?;
        Ok((
            rest3,
            format!("{first_part}.{second_part}").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit).into())
            })?,
        ))
    } else {
        Ok((
            rest,
            format!("{first_part}.0").parse().map_err(|_| {
                nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Digit).into())
            })?,
        ))
    }
}

pub fn tag_name(input: &str) -> IResult<&str, String> {
    let (rest, first) = take_while1(is_ident_start)(input)?;
    let (rest, rest_chars) = opt(take_while1(is_ident_char)).parse(rest)?;
    match rest_chars {
        Some(val) => Ok((rest, format!("{}{}", first, val))),
        None => Ok((rest, first.to_string())),
    }
}

pub fn tag_string(input: &str) -> IResult<&str, String> {
    let (rest, _) = tag("'")(input)?;
    let (rest, content) = take_while1(|c: char| c != '\'')(rest)?;
    let (rest, _) = tag("'")(rest)?;
    Ok((rest, content.to_string()))
}

pub fn tag_key_word_logic<'a>(
    keyword: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, &'a str> {
    move |input: &'a str| {
        let (new_input, matched) = tag_no_case(keyword).parse(input)?;

        if new_input.trim_start().starts_with(PARENS_0) {
            Ok((new_input, matched))
        } else {
            let (new_input, _) = multispace1(new_input)?;
            Ok((new_input, matched))
        }
    }
}
pub fn tag_is_not(input: &str) -> IResult<&str, &str> {
    let (input, _) = (
        tag_no_case(IS_SIGN),
        multispace1,
        tag_no_case(NOT_SIGN),
        multispace1,
    )
        .parse(input)?;
    Ok((input, (IS_NOT_SIGN)))
}
pub fn tag_function(input: &str) -> IResult<&str, String> {
    let (a, func_name) = tag_name(input)?;
    let b = a.trim();
    let (input_retour, _) = tag(PARENS_0).parse(b)?;

    if is_func_valid(&func_name) {
        Ok((input_retour, func_name))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ).into()))
    }
}
