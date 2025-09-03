use super::*;
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::opt,
    multi::{many0, many1},
    sequence::preceded,
};

pub fn parse_josuhi(input: &str) -> IResult<&str, Josushi> {
    xml_tag(
        "div",
        ((
            alt((parse_subhead_number, parse_subheadword_josushi)),
            many1(parse_accent),
            many0(parse_indet),
            opt(parse_note),
        ))
            .map(|(n, a, i, note)| Josushi(n, a, i, note)),
    )(input)
    .verify_class("josushi")
}

fn parse_indet(input: &str) -> IResult<&str, Indent> {
    xml_tag("span", parse_accent_text.map(Indent))(input).verify_class("indent")
}

fn parse_subhead_number(input: &str) -> IResult<&str, JosushiNumber> {
    xml_tag("span", parse_named_word.map(|(n, w)| JosushiNumber(n, w)))(input)
        .verify_class("subheadword number")
}
fn parse_subheadword_josushi(input: &str) -> IResult<&str, JosushiNumber> {
    xml_tag("span", parse_named_word.map(|(n, w)| JosushiNumber(n, w)))(input)
        .verify_class("subheadword josushi")
}

fn parse_note_num(input: &str) -> IResult<&str, Num> {
    xml_tag("span", text.map(|s| Num(s.into())))(input).verify_class("note_num")
}
fn parse_note(input: &str) -> IResult<&str, Notes> {
    xml_tag(
        "span",
        many1(preceded(
            opt(parse_br),
            (
                opt(parse_note_num),
                many1(alt((
                    text.map(|s| NoteContent::Text(s.into())),
                    parse_accent.map(NoteContent::Accent),
                    parse_symbol_backslash.map(|s| NoteContent::SymbolBackslash(s.into())),
                    parse_round_box.map(|s| NoteContent::RoundBox(s.into())),
                    parse_symbol_macron.map(|s| NoteContent::SymbolMacron(s.into())),
                    parse_bodyref.map(|(i, c)| NoteContent::Ref(i, c)),
                ))),
            ),
        ))
        .map(|n| Notes(n)),
    )(input)
    .verify_class("note")
}
fn parse_br(input: &str) -> IResult<&str, ()> {
    empty_xml_tag("br", input).map(|(rem, _attr)| (rem, ()))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn josushi_1() {
        let s = "<div class=\"josushi\"><span class=\"subheadword josushi\"><a name=\"75733-0001\" class=\"anchor\">1</a></span><span class=\"accent\"><span class=\"accent_text\">イチア<span class=\"symbol_backslash\">＼</span>ール<span class=\"sound\"><a href=\"20180411131924.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span></span></span></div>";
        parse_josuhi(s).unwrap();
    }
}
