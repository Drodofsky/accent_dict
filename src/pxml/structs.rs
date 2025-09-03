use std::fmt;

use serde::{Deserialize, Serialize};

use crate::circle::to_circle;
#[derive(Debug, Serialize, Deserialize)]
pub struct DicItem(pub Id, pub Vec<HeadG>, pub Vec<Josushi>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Id(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadG(pub Head, pub Body);
#[derive(Debug, Serialize, Deserialize)]
pub enum Head {
    H(Vec<H>),
    Joshiword(Joshiword),
    Ref(Vec<RefHead>),
    None,
}

impl fmt::Display for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Head::H(h) => {
                write!(
                    f,
                    "{}",
                    h.iter().map(|h| format!("{h} ")).collect::<String>()
                )
            }
            _ => write!(f, "none"),
        }
    }
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

impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inner::DAngleBrackets(d) => write!(f, "{d}"),
            Inner::RoundBrackets(s) => write!(f, "{s}"),
            Inner::Text(s) => write!(f, "{s}"),
            Inner::Span(s) => write!(f, "{s}"),
            Inner::Ruby((ru)) => write!(
                f,
                "{}",
                ru.iter()
                    .map(|(r, os)| if let Some(s) = os.as_deref() {
                        let mut r = format!("{r}");
                        r.push_str(s);
                        r
                    } else {
                        format!("{r}")
                    })
                    .collect::<String>()
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum H {
    Headword(String),
    HW(String, Option<(Vec<Inner>, char)>),
    SquareBrackets(String),
    RoundBrackets(String),
    SquareBox(String),
    Subheadword(ID, String),
    BlackBranckets(String, Option<(Inner, char)>),
    DAngleBrackets(String),
    AngleBrackets(String),
    Dia(String),
}

impl fmt::Display for H {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            H::Headword(s) => write!(f, "{s}"),
            H::SquareBrackets(s) => write!(f, "{s}"),
            H::RoundBrackets(s) => write!(f, "{s}"),
            H::SquareBox(s) => write!(f, "{s}"),
            H::DAngleBrackets(s) => write!(f, "{s}"),
            H::AngleBrackets(s) => write!(f, "{s}"),
            H::Dia(s) => write!(f, "{s}"),
            H::HW(start, inner) => {
                let mut start = start.clone();
                if let Some((inn, end)) = inner {
                    start.push_str(&inn.iter().map(|i| format!("{i}")).collect::<String>());
                    start.push(*end)
                }
                write!(f, "{start}")
            }
            H::BlackBranckets(start, inner) => {
                let mut start = start.clone();
                if let Some((inn, end)) = inner {
                    start.push_str(&format!("{inn}"));
                    start.push(*end)
                }
                write!(f, "{start}")
            }
            H::Subheadword(n, _s) => write!(f, "{}", n.0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Joshiword(pub ID, pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ID(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct DAngleBrackets(pub String);

impl fmt::Display for DAngleBrackets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body(pub Vec<BodyContent>);

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
pub struct AccentHead(pub SquareBox);

#[derive(Debug, Serialize, Deserialize)]
pub struct Audio(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct SquareBox(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct Accent(pub Option<AccentHead>, pub Vec<AccentText>);

impl fmt::Display for Accent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.1.iter().map(|s| format!("{s}")).collect();
        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccentText {
    Text(String),
    SymbolMacron(String),
    SymbolBackslash(String),
    RoundBox(String),
    Sound(String),
    SquareBox(String),
    NoteRef(String),
}

impl fmt::Display for AccentText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccentText::Text(s) => write!(f, "{s}"),
            AccentText::SymbolMacron(s) => write!(f, "{s}"),
            AccentText::SymbolBackslash(s) => write!(f, "{s}"),
            AccentText::RoundBox(s) => write!(f, "{}", to_circle(s)),
            _ => write!(f, ""),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundBrackets(pub Vec<AccentText>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleHead(pub ID, pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Example(pub ExampleHead, pub Vec<ExampleContent>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleContent {
    AccentExample(Vec<AccentText>),
    SquareBrackets(String),
    Ref(Id, Vec<RefContent>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Josushi(
    pub JosushiNumber,
    pub Vec<Accent>,
    pub Vec<Indent>,
    pub Option<Notes>,
);
#[derive(Debug, Serialize, Deserialize)]
pub struct JosushiNumber(pub ID, pub String);

#[derive(Debug, Serialize, Deserialize)]

pub struct Indent(pub Vec<AccentText>);
#[derive(Debug, Serialize, Deserialize)]

pub struct Notes(pub Vec<(Option<Num>, Vec<NoteContent>)>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Num(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum NoteContent {
    Text(String),
    Accent(Accent),
    SymbolBackslash(String),
    RoundBox(String),
    SymbolMacron(String),
    Ref(Id, Vec<RefContent>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rb(pub String);

impl fmt::Display for Rb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rt(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruby(pub Rb, pub Rt);

impl fmt::Display for Ruby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // unicode furigana is not supported
        // write!(f, "\u{FFF9}{}\u{FFFA}{}\u{FFFB}", self.0,self.1)
        write!(f, "{}", self.0)
    }
}
