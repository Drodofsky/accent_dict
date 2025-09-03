use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, space0},
    error::{ErrorKind, ParseError},
    multi::many0,
    sequence::{delimited, preceded, terminated},
};

/*pub fn html_tag<'s,T:ToStr,O,F,E>(mut inner:F)-> impl FnMut(&'s str) -> IResult<&'s  str,(Vec<(&'s str,&'s str)>,O)> where F:Parser<&'s str,O,E>, E: ParseError<&'s str> {
    move |input:&str| {
        tuple((parse_open_tag::<T>,inner))(input)
    }
}
*/

pub fn xml_tag<'s, O, F, E>(
    tag: &'s str,
    mut inner: F,
) -> impl FnMut(&'s str) -> IResult<&'s str, (Vec<(&'s str, &'s str)>, O)>
where
    //F: Parser<&'s str, O, E>,
    F: Parser<&'s str, Error = E, Output = O>,
    E: ParseError<&'s str>,
    nom::Err<nom::error::Error<&'s str>>: From<nom::Err<E>>,
{
    move |input: &str| {
        let (input, attrs) = parse_open_tag(tag, input)?;
        let (input, content) = inner.parse(input)?;
        let (input, _) = parse_close_tag(tag, input)?;
        Ok((input, (attrs, content)))
    }
}

pub trait VerifyClass<'s, O> {
    fn verify_class(self, class: &'s str) -> IResult<&'s str, O>;
}

impl<'s, T, O> VerifyClass<'s, O> for T
where
    T: Verify<'s, O>,
{
    fn verify_class(self, class: &'s str) -> IResult<&'s str, O> {
        self.verify("class", class)
    }
}

pub trait Verify<'s, O> {
    fn verify(self, key: &'s str, val: &'s str) -> IResult<&'s str, O>;
}

impl<'s, O> Verify<'s, O> for IResult<&'s str, (Vec<(&'s str, &'s str)>, O)> {
    fn verify(self, key: &'s str, val: &'s str) -> IResult<&'s str, O> {
        self.and_then(|(rem, (attrs, x))| {
            if attrs.attr(key) != Some(val) {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    rem,
                    ErrorKind::Satisfy,
                )))
            } else {
                Ok((rem, x))
            }
        })
    }
}

impl<'s> Verify<'s, ()> for IResult<&'s str, Vec<(&'s str, &'s str)>> {
    fn verify(self, key: &'s str, val: &'s str) -> IResult<&'s str, ()> {
        self.and_then(|(rem, attrs)| {
            if attrs.attr(key) != Some(val) {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    rem,
                    ErrorKind::Satisfy,
                )))
            } else {
                Ok((rem, ()))
            }
        })
    }
}

pub fn kana(input: &str) -> IResult<&str, &str> {
    //take_while(|c: char| c != '<')(input)
    //take_until("<")(input)
    take_while1(|c: char| (c >= 'あ' && c <= '゜') || (c >= 'ァ' && c <= 'ー'))(input)
}

pub fn kanji(input: &str) -> IResult<&str, &str> {
    //take_while(|c: char| c != '<')(input)
    //take_until("<")(input)
    take_while1(|c: char| c >= '一' && c <= '龜')(input)
}

pub fn text(input: &str) -> IResult<&str, &str> {
    //take_while(|c: char| c != '<')(input)
    //take_until("<")(input)
    take_while1(|c: char| c != '<')(input)
}

pub fn h_text(input: &str) -> IResult<&str, &str> {
    //take_while(|c: char| c != '<')(input)
    //take_until("<")(input)
    take_while1(|c: char| {
        (c >= '一' && c <= '龜')
            || (c >= 'あ' && c <= '゜')
            || (c >= 'ァ' && c <= 'ー')
            || (c == '，')
    })(input)
}

pub fn ref_text(input: &str) -> IResult<&str, &str> {
    //take_while(|c: char| c != '<')(input)
    //take_until("<")(input)
    take_while1(|c: char| {
        (c >= '一' && c <= '龜')
            || (c >= 'あ' && c <= '゜')
            || (c >= 'ァ' && c <= 'ー')
            || (c == '☞')
            || (c == '［')
            || (c == '］')
            || (c == '「')
            || (c == '」')
    })(input)
}

pub fn empty_xml_tag<'s>(t: &'s str, input: &'s str) -> IResult<&'s str, Vec<(&'s str, &'s str)>> {
    delimited(char('<'), (parse_identifier, parse_attrs), tag("/>"))
        .parse(input)
        .and_then(|(rem, (tag, attr))| {
            if tag != t {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    input,
                    ErrorKind::Satisfy,
                )))
            } else {
                Ok((rem, attr))
            }
        })
}

fn parse_open_tag<'s>(tag: &'s str, input: &'s str) -> IResult<&'s str, Vec<(&'s str, &'s str)>> {
    delimited(char('<'), (parse_identifier, parse_attrs), char('>'))
        .parse(input)
        .and_then(|(rem, (t, attr))| {
            if tag != t {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    input,
                    ErrorKind::Satisfy,
                )))
            } else {
                Ok((rem, attr))
            }
        })
}

fn parse_close_tag<'s>(t: &'s str, input: &'s str) -> IResult<&'s str, &'s str> {
    delimited(tag("</"), parse_identifier, char('>'))
        .parse(input)
        .and_then(|(rem, parsed)| {
            if parsed != t {
                Err(nom::Err::Error(ParseError::from_error_kind(
                    input,
                    ErrorKind::Satisfy,
                )))
            } else {
                Ok((rem, parsed))
            }
        })
}

fn parse_attr(input: &str) -> IResult<&str, (&str, &str)> {
    (terminated(parse_identifier, char('=')), parse_str).parse(input)
}

fn parse_attrs(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many0(preceded(space0, parse_attr)).parse(input)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(char('"'), take_while(|c: char| c != '"'), char('"')).parse(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_alphabetic() || c == '_' || c == '-')(input)
}

pub trait Attr {
    fn attr(&self, key: &str) -> Option<&str>;
}

impl Attr for Vec<(&str, &str)> {
    fn attr(&self, key: &str) -> Option<&str> {
        self.iter()
            .find_map(|(k, v)| if *k == key { Some(*v) } else { None })
    }
}

impl Attr for &[(&str, &str)] {
    fn attr(&self, key: &str) -> Option<&str> {
        self.iter()
            .find_map(|(k, v)| if *k == key { Some(*v) } else { None })
    }
}

#[cfg(test)]
mod tests {
    use nom::character::complete::alpha0;

    use super::*;
    #[test]
    fn attr() {
        let s = "h_g=\"aあ\"";
        let (rem, (a, b)) = parse_attr(s).unwrap();
        assert!(rem.is_empty());
        assert_eq!(a, "h_g");
        assert_eq!(b, "aあ");
    }

    #[test]
    fn html() {
        let str = "<html class=\"a\" b=\"c_c\">Hello</html>";
        let (rem, (attr, inner)) = xml_tag("html", alpha0)(str).unwrap();
        assert!(rem.is_empty());
        assert_eq!(inner, "Hello");
        assert_eq!(attr.attr("class"), Some("a"));
        assert_eq!(attr.attr("b"), Some("c_c"));
    }
}
