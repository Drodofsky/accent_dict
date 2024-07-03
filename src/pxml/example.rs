use nom::{
    branch::alt,
    character::complete::char,
    combinator::opt,
    multi::{many0, many1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
use serde::{Deserialize, Serialize};

use super::{
    parse_body, parse_bodyref, parse_named_word, parse_round_box, parse_sound,
    parse_square_brackets, parse_symbol_backslash, parse_symbol_macron, text, xml_tag, AccentText,
    Id, Name, RefContent, VerifyClass,
};
#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleHead(Name, String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Example(ExampleHead, Vec<ExampleContent>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleContent {
    AccentExample(Vec<AccentText>),
    SquareBrackets(String),
    Ref(Id, Vec<RefContent>),
}

pub fn parse_example(input: &str) -> IResult<&str, Example> {
    xml_tag(
        "span",
        tuple((
            parse_example_head,
            many1(preceded(
                opt(char('　')),
                alt((
                    parse_accent_example.map(ExampleContent::AccentExample),
                    parse_bodyref.map(|(i, r)| ExampleContent::Ref(i, r)),
                    parse_square_brackets.map(|s| ExampleContent::SquareBrackets(s.into())),
                )),
            )),
        ))
        .map(|(h, a)| Example(h, a)),
    )(input)
    .verify_class("example")
}

fn parse_example_head(input: &str) -> IResult<&str, ExampleHead> {
    xml_tag("span", parse_named_word.map(|(n, w)| ExampleHead(n, w)))(input).verify_class("ex_head")
}

fn parse_accent_example(input: &str) -> IResult<&str, Vec<AccentText>> {
    xml_tag(
        "span",
        many0(alt((
            parse_symbol_macron.map(|x| AccentText::SymbolMacron(x.to_string())),
            parse_symbol_backslash.map(|x| AccentText::SymbolBackslash(x.to_string())),
            parse_round_box.map(|x| AccentText::RoundBox(x.to_string())),
            parse_sound.map(AccentText::Sound),
            text.map(|x| AccentText::Text(x.to_string())),
        ))),
    )(input)
    .verify_class("accent accent_example")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_content_1() {
        let s = "";
    }
}
