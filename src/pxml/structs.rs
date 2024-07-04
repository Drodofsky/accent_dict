use serde::{Serialize,Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct DicItem(pub Id,pub  Vec<HeadG>,pub  Vec<Josushi>);
#[derive(Debug, Serialize, Deserialize)]
pub struct Id(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct HeadG(pub Head,pub  Body);
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
pub struct Joshiword(pub Name,pub  String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Name(pub String);


#[derive(Debug, Serialize, Deserialize)]
pub struct DAngleBrackets(pub String);
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
pub struct Accent(pub Option<AccentHead>,pub  Vec<AccentText>);

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
pub struct RoundBrackets(pub Vec<AccentText>);


#[derive(Debug, Serialize, Deserialize)]
pub struct ExampleHead(pub Name,pub  String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Example(pub ExampleHead,pub  Vec<ExampleContent>);

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleContent {
    AccentExample(Vec<AccentText>),
    SquareBrackets(String),
    Ref(Id, Vec<RefContent>),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Josushi(pub JosushiNumber,pub  Vec<Accent>, pub Vec<Indent>,pub  Option<Notes>);
#[derive(Debug, Serialize, Deserialize)]
pub struct JosushiNumber(pub Name,pub  String);

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Rt(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Ruby(pub Rb,pub  Rt);