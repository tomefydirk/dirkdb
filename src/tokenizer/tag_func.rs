use nom::{
    Parser,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{digit1, multispace1},
    combinator::opt,
};

use crate::{general_struct::{constant::*, structure::QualifiedIdentifier}, tokenizer::helper::codon_stop};
use crate::{
    IResult,
    tokenizer::helper::{is_ident_char, is_ident_start},
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

        if codon_stop(new_input){
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
pub fn tag_variable(input: &str) -> IResult<&str, QualifiedIdentifier> {
    let (rest, current_field) = tag_name(input)?;
    let (rest2, point) = opt(tag(".")).parse(rest)?;
    match point {
        Some(_) => {
            let (rest3, second_part) = tag_name(rest2)?;
            Ok((
                rest3,
                QualifiedIdentifier::new(Some(current_field), second_part),
            ))
        }
        None => Ok((rest, QualifiedIdentifier::new(None, current_field))),
    }
}
use crate::general_struct::structure::ManyKeyWord;

pub fn tag_manykey_word_logic<'a>(
    keywords: ManyKeyWord<&'static str>,
) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<&'a str>> {
    move |mut input: &'a str| {
        let mut matched = Vec::new();

        for word in &keywords.words {
            // parse chaque mot-clé (insensible à la casse)
            let (new_input, m) = tag_no_case(*word).parse(input)?;
            matched.push(m);

            // avancer
            input = new_input;

                let (new_input, _) = multispace1(input)?;
                input = new_input;
            
        }

        Ok((input, matched))
    }
}
