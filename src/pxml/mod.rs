mod example;
mod josushi;
mod ruby;
mod xml;
use example::*;
use josushi::*;
use nom::{
    branch::alt,
    bytes::complete::take_until,
    character::complete::char,
    combinator::opt,
    error::{ErrorKind, ParseError},
    multi::{many0, many1},
    sequence::{preceded, tuple},
    IResult, Parser,
};
pub use ruby::*;
use serde::{Deserialize, Serialize};
pub use xml::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DicItem(Id, Vec<HeadG>, Vec<Josushi>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Id(String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadG(Head, Body);
#[derive(Debug, Serialize, Deserialize)]
pub enum Head {
    H(Vec<H>),
    Joshiword(Joshiword),
    Ref(Vec<RefHead>),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RefHead {
    Refheadword(String),
    BlackBranckets(String, Option<(Inner, char)>),
    DAngleBrackets(String),
    RoundBrackets(String),
    SquareBrackets(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Inner {
    DAngleBrackets(DAngleBrackets),
    Ruby(Vec<(Ruby, Option<String>)>),
    RoundBrackets(String),
    Text(String),
    Span(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum H {
    Headword(String),
    HW(String, Option<(Vec<Inner>, char)>),
    SquareBrackets(String),
    RoundBrackets(String),
    SquareBox(String),
    Subheadword(Name, String),
    BlackBranckets(String, Option<(Inner, char)>),
    DAngleBrackets(String),
    AngleBrackets(String),
    Dia(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Joshiword(Name, String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Name(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DAngleBrackets(String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Body(Vec<BodyContent>);

#[derive(Debug, Serialize, Deserialize)]
pub enum BodyContent {
    Accent(Vec<Accent>),
    AccentRound(RoundBrackets, Option<Audio>),
    Ref(Id, Vec<RefContent>),
    ConTable(Vec<ConTableContent>),
    SquareBox(String),
    Example(Example),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConTableContent {
    Accent(Vec<Accent>),
    AccentRound(RoundBrackets, Option<Audio>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RefContent {
    Text(String),
    RoundBrackets(String),
    TextSpan(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccentHead(SquareBox);

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SquareBox(String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Accent(Option<AccentHead>, Vec<AccentText>);

#[derive(Debug, Serialize, Deserialize)]
pub enum AccentText {
    Text(String),
    SymbolMacron(String),
    SymbolBackslash(String),
    RoundBox(String),
    Sound(String),
    SquareBox(String),
    NoteRef(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundBrackets(Vec<AccentText>);

pub fn parse_xml(xml: &str) -> DicItem {
    let xml = xml
        .strip_prefix(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>
",
        )
        .unwrap();
    let (rem, dic_item) = parse_html(xml).expect(&format!("failed to parse\n{xml}\n"));
    assert!(rem.trim().is_empty());
    dic_item
}

fn parse_html(input: &str) -> IResult<&str, DicItem> {
    xml_tag("html", preceded(parse_head, parse_body))(input).map(|(rem, (_, d))| (rem, d))
}

fn parse_head(input: &str) -> IResult<&str, ()> {
    xml_tag("head", take_until("</head>"))(input).map(|(rem, _)| (rem, ()))
}

fn parse_body(input: &str) -> IResult<&str, DicItem> {
    xml_tag("body", parse_dic_item)(input).map(|(rem, (_, d))| (rem, d))
}

fn parse_dic_item(input: &str) -> IResult<&str, DicItem> {
    xml_tag("span", tuple((many1(parse_head_g), many0(parse_josuhi))))(input).and_then(
        |(rem, (attr, (i, j)))| {
            if attr.attr("class") == Some("dic-item") {
                let id = Id(attr.attr("id").unwrap().parse().unwrap());

                Ok((rem, DicItem(id, i, j)))
            } else {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    input,
                    ErrorKind::Satisfy,
                )))
            }
        },
    )
}

fn parse_head_g(input: &str) -> IResult<&str, HeadG> {
    xml_tag(
        "span",
        tuple((
            alt((parse_dic_head, parse_dic_head_empty)),
            alt((parse_dic_body, parse_dic_body_empty)),
        ))
        .map(|(h, b)| HeadG(h, b)),
    )(input)
    .verify_class("head-g")
}

fn parse_dic_head(input: &str) -> IResult<&str, Head> {
    xml_tag(
        "div",
        alt((
            parse_h.map(Head::H),
            parse_joshi_word.map(Head::Joshiword),
            parse_refhead.map(Head::Ref),
        )),
    )(input)
    .verify_class("head")
}

fn parse_dic_head_empty(input: &str) -> IResult<&str, Head> {
    empty_xml_tag("div", input)
        .verify_class("head")
        .map(|(rem, _res)| (rem, Head::None))
}

fn parse_dic_body_empty(input: &str) -> IResult<&str, Body> {
    empty_xml_tag("div", input)
        .verify_class("body")
        .map(|(rem, _res)| (rem, Body(Vec::new())))
}

fn parse_joshi_word(input: &str) -> IResult<&str, Joshiword> {
    xml_tag("span", parse_named_word.map(|(n, w)| Joshiword(n, w)))(input).verify_class("joshiword")
}

fn parse_subheadword(input: &str) -> IResult<&str, H> {
    xml_tag("span", parse_named_word.map(|(n, w)| H::Subheadword(n, w)))(input)
        .verify_class("subheadword ")
}

fn parse_named_word(input: &str) -> IResult<&str, (Name, String)> {
    xml_tag("a", text)(input)
        .map(|(rem, (attrs, x))| {
            let name = Name(attrs.attr("name").unwrap().into());
            (rem, (attrs, (name, x.into())))
        })
        .verify_class("anchor")
}

fn parse_h(input: &str) -> IResult<&str, Vec<H>> {
    xml_tag(
        "span",
        many0(alt((
            parse_headword.map(|x| H::Headword(x.into())),
            parse_hw.map(|(hw, o)| H::HW(hw.into(), o.map(|(l, r)| (l, r.into())))),
            parse_round_brackets.map(|x| H::RoundBrackets(x.into())),
            parse_square_brackets.map(|x| H::SquareBrackets(x.into())),
            parse_square_box.map(|x| H::SquareBox(x.into())),
            parse_subheadword,
            parse_black_branckets
                .map(|(b, o)| H::BlackBranckets(b.into(), o.map(|(l, r)| (l, r.into())))),
            parse_d_angle_brackets.map(|s| H::DAngleBrackets(s.into())),
            parse_angle_brackets.map(|s| H::AngleBrackets(s.into())),
            parse_dia.map(|s| H::Dia(s.into())),
        ))),
    )(input)
    .verify_class("h")
}

fn parse_headword(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("headword")
}

fn parse_hw(input: &str) -> IResult<&str, (&str, Option<(Vec<Inner>, char)>)> {
    xml_tag(
        "span",
        tuple((
            text,
            opt(tuple((
                many1(alt((
                    parse_d_angle_brackets.map(|s| Inner::DAngleBrackets(DAngleBrackets(s.into()))),
                    many1(tuple((parse_ruby, opt(kana.map(|s| s.to_string())))))
                        .map(|r| Inner::Ruby(r)),
                    parse_round_brackets.map(|r| Inner::RoundBrackets(r.into())),
                    h_text.map(|s| Inner::Text(s.into())),
                    parse_span.map(|s| Inner::Span(s.into())),
                ))),
                char('】'),
            ))),
        )),
    )(input)
    .verify_class("hw")
}
fn parse_black_branckets(input: &str) -> IResult<&str, (&str, Option<(Inner, char)>)> {
    xml_tag(
        "span",
        tuple((
            text,
            opt(tuple((
                alt((
                    parse_d_angle_brackets.map(|s| Inner::DAngleBrackets(DAngleBrackets(s.into()))),
                    many1(tuple((parse_ruby, opt(kana.map(|s| s.to_string())))))
                        .map(|r| Inner::Ruby(r)),
                    parse_round_brackets.map(|r| Inner::RoundBrackets(r.into())),
                )),
                char('】'),
            ))),
        )),
    )(input)
    .verify_class("black_branckets")
}

fn parse_span(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).and_then(|(rem, (attr, t))| {
        if attr.len() == 0 {
            Ok((rem, t))
        } else {
            Err(nom::Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::Tag,
            )))
        }
    })
}

fn parse_d_angle_brackets(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("d_angle_brackets")
}

fn parse_angle_brackets(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("angle_brackets")
}

fn parse_dia(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("dia")
}

fn parse_square_brackets(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("square_brackets")
}

fn parse_square_box(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("square_box")
}

fn parse_round_brackets(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("round_brackets")
}

fn parse_refhead(input: &str) -> IResult<&str, Vec<RefHead>> {
    xml_tag(
        "span",
        many1(alt((
            parse_refheadword.map(|s| RefHead::Refheadword(s.into())),
            parse_black_branckets
                .map(|(b, d)| RefHead::BlackBranckets(b.into(), d.map(|(d, r)| (d, r.into())))),
            parse_d_angle_brackets.map(|s| RefHead::DAngleBrackets(s.into())),
            parse_round_brackets.map(|s| RefHead::RoundBrackets(s.into())),
            parse_square_brackets.map(|s| RefHead::SquareBrackets(s.into())),
        ))),
    )(input)
    .verify_class("ref")
}
fn parse_refheadword(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("refheadword")
}

fn parse_dic_body(input: &str) -> IResult<&str, Body> {
    xml_tag(
        "div",
        many1(alt((
            parse_con_table.map(BodyContent::ConTable),
            many1(parse_accent).map(BodyContent::Accent),
            parse_bodyref.map(|(i, c)| BodyContent::Ref(i, c)),
            parse_accent_round.map(|(r, a)| BodyContent::AccentRound(r, a)),
            parse_square_box.map(|s| BodyContent::SquareBox(s.into())),
            parse_example.map(BodyContent::Example),
        ))),
    )(input)
    .verify_class("body")
    .map(|(rem, c)| (rem, Body(c)))
}

fn parse_con_table(input: &str) -> IResult<&str, Vec<ConTableContent>> {
    xml_tag(
        "span",
        many1(alt((
            many1(alt((parse_accent, parse_accent2))).map(ConTableContent::Accent),
            parse_accent_round.map(|(r, a)| ConTableContent::AccentRound(r, a)),
        ))),
    )(input)
    .verify_class("con_table")
}

fn parse_accent2(input: &str) -> IResult<&str, Accent> {
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
    .verify_class("accent")
    .map(|(rem, a)| (rem, Accent(None, a)))
}

fn parse_accent_head(input: &str) -> IResult<&str, AccentHead> {
    xml_tag(
        "accent_head",
        parse_square_box.map(|(s)| AccentHead(SquareBox(s.into()))),
    )(input)
    .map(|(rem, (_, a))| (rem, a))
}

fn parse_bodyref(input: &str) -> IResult<&str, (Id, Vec<RefContent>)> {
    xml_tag("span", parse_ref)(input).verify_class("ref")
}

fn parse_ref(input: &str) -> IResult<&str, (Id, Vec<RefContent>)> {
    xml_tag(
        "a",
        many1(alt((
            parse_round_brackets.map(|s| RefContent::RoundBrackets(s.into())),
            parse_text.map(|s| RefContent::TextSpan(s.into())),
            ref_text.map(|s| RefContent::Text(s.into())),
        ))),
    )(input)
    .map(|(rem, (attrs, c))| {
        let id = Id(attrs.attr("href").unwrap().into());

        (rem, (id, c))
    })
}

fn parse_text(input: &str) -> IResult<&str, &str> {
    xml_tag("text", text)(input).map(|(rem, (_attr, t))| (rem, t))
}

fn parse_accent(input: &str) -> IResult<&str, Accent> {
    xml_tag(
        "span",
        tuple((opt(parse_accent_head), parse_accent_text)).map(|(h, a)| Accent(h, a)),
    )(input)
    .verify_class("accent")
}

fn parse_accent_text(input: &str) -> IResult<&str, Vec<AccentText>> {
    xml_tag(
        "span",
        many0(alt((
            parse_symbol_macron.map(|x| AccentText::SymbolMacron(x.to_string())),
            parse_symbol_backslash.map(|x| AccentText::SymbolBackslash(x.to_string())),
            parse_round_box.map(|x| AccentText::RoundBox(x.to_string())),
            parse_sound.map(AccentText::Sound),
            text.map(|x| AccentText::Text(x.to_string())),
            parse_square_box.map(|s| AccentText::SquareBox(s.into())),
            parse_note_ref.map(|s| AccentText::NoteRef(s.into())),
        ))),
    )(input)
    .verify_class("accent_text")
}

fn parse_note_ref(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("note_ref")
}

fn parse_round_brackets2(input: &str) -> IResult<&str, RoundBrackets> {
    xml_tag(
        "span",
        many1(alt((
            parse_symbol_macron.map(|x| AccentText::SymbolMacron(x.to_string())),
            parse_symbol_backslash.map(|x| AccentText::SymbolBackslash(x.to_string())),
            parse_round_box.map(|x| AccentText::RoundBox(x.to_string())),
            parse_sound.map(AccentText::Sound),
            text.map(|x| AccentText::Text(x.to_string())),
        )))
        .map(RoundBrackets),
    )(input)
    .verify_class("round_brackets")
}

fn parse_accent_round(input: &str) -> IResult<&str, (RoundBrackets, Option<Audio>)> {
    xml_tag(
        "span",
        tuple((parse_round_brackets2, opt(parse_sound.map(Audio)))),
    )(input)
    .verify_class("accent accent_round")
}

fn parse_symbol_macron(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("symbol_macron")
}

fn parse_symbol_backslash(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("symbol_backslash")
}

fn parse_round_box(input: &str) -> IResult<&str, &str> {
    xml_tag("span", text)(input).verify_class("round_box")
}

fn parse_sound(input: &str) -> IResult<&str, String> {
    xml_tag("span", parse_sound_link)(input).verify_class("sound")
}

fn parse_sound_link(input: &str) -> IResult<&str, String> {
    xml_tag("a", parse_img)(input)
        .map(|(rem, (attr, _))| (rem, attr.attr("href").unwrap().to_owned()))
}

fn parse_img(input: &str) -> IResult<&str, ()> {
    empty_xml_tag("img", input).verify("alt", "音声")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn head() {
        let s = "<head><meta http-equiv=\"Content-Type\" content=\"text/html;charset=utf-8\"/><meta name=\"viewport\" content=\"width=device-width, initial-scale = 1.0, user-scalable = yes, minimum-scale=0.333, maximum-scale=3.0\"/><link rel=\"stylesheet\" href=\"nhk_accent.css\" media=\"all\"/></head>";
        let (rem, _res) = parse_head(s).unwrap();
        assert!(rem.is_empty());
    }
    #[test]
    fn sound() {
        let s = "<span class=\"sound\"><a href=\"20170630130152.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span>";
        let (rem, sound) = parse_sound(s).unwrap();
        assert!(rem.is_empty());
        assert_eq!(sound, "20170630130152.aac")
    }

    #[test]
    fn symbol_macron() {
        let s = "<span class=\"symbol_macron\">▔</span>";
        let (rem, _) = parse_symbol_macron(s).unwrap();
        assert!(rem.is_empty());
    }
    #[test]
    fn accent() {
        let s = "<span class=\"accent_text\"><span class=\"symbol_macron\">▔</span><span class=\"sound\"><a href=\"20170630130152.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span></span>";
        let (rem, acc) = parse_accent_text(s).unwrap();
    }

    #[test]
    fn accent2() {
        let s = "<span class=\"accent_text\"><span class=\"symbol_macron\">▔</span><span class=\"sound\"><a href=\"20170630130152.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span></span>";
        let (rem, acc) = parse_accent_text(s).unwrap();
    }

    #[test]
    fn round_brackets2() {
        let s = "<span class=\"round_brackets\">（現在の「オ<span class=\"round_box\">シ</span>フィエ<span class=\"symbol_backslash\">＼</span>ンチム」）</span><span class=\"sound\"><a href=\"20170630141404.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span>";
        let (rem, r) = parse_round_brackets2(s).unwrap();
    }
    #[test]
    fn acccent_round() {
        let s = "<span class=\"accent accent_round\"><span class=\"round_brackets\">（現在の「オ<span class=\"round_box\">シ</span>フィエ<span class=\"symbol_backslash\">＼</span>ンチム」）</span><span class=\"sound\"><a href=\"20170630141404.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span></span>";
        let (rem, r) = parse_accent_round(s).unwrap();
    }

    #[test]
    fn head_g_2() {
        let s = "<span class=\"head-g\"><div class=\"head\"><span class=\"h\"><span class=\"subheadword \"><a name=\"01611-0001\" class=\"anchor\">あぶらげ</a></span><span class=\"d_angle_brackets\">《×油揚》</span></span></div><div class=\"body\"><span class=\"accent\"><span class=\"accent_text\">アブラ<span class=\"symbol_backslash\">＼</span>ケ゚<span class=\"sound\"><a href=\"20170714114529.aac\"><img alt=\"音声\" src=\"HMDicAudio.png\"/></a></span></span></span></div></span></span>";
        let (rem, res) = parse_head_g(s).unwrap();
    }
    #[test]
    fn head_2() {
        let s = "<div class=\"head\"><span class=\"h\"><span class=\"subheadword \"><a name=\"01611-0001\" class=\"anchor\">あぶらげ</a></span><span class=\"d_angle_brackets\">《×油揚》</span></span></div>";
        let (rem, res) = parse_dic_head(s).unwrap();
    }
    #[test]
    fn hw_2() {
        let s = "<span class=\"hw\">【<span>葛</span>城】</span>";
        let (rem, res) = parse_hw(s).unwrap();
    }
    #[test]
    fn kanji_1() {
        kanji("城!").unwrap();
        // kanji("丁!").unwrap();
    }

    #[test]
    fn ref_text_1() {
        let (rem, res) = ref_text("☞クァルテット").unwrap();
        assert_eq!(res, "☞クァルテット");
        // kanji("丁!").unwrap();
    }
    #[test]
    fn ref_text_2() {
        let (rem, res) = ref_text("棟［ムネ］").unwrap();
        assert_eq!(res, "棟［ムネ］");
        // kanji("丁!").unwrap();
    }
    #[test]
    fn ref_2() {
        let s =
            "<a href=\"55030\"><text>☞</text>ばぬし</a></span></div></span></span></body></html>\n";
        parse_ref(s).unwrap();
    }
    // TODO extract all attr
    #[test]
    fn ref_3() {
        let s = "<span class=\"ref\"><a href=\"appendix/furoku_02_01.html#furoku_p44\" title=\"Ⅰ\u{3000}複合名詞の発音とアクセント\"><text>☞ 個人名＋肩書きなどのアクセント</text></a></span>";
        parse_bodyref(s).unwrap();
    }
}
