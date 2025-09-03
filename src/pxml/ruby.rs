use nom::IResult;

use super::*;

fn parse_rb(input: &str) -> IResult<&str, Rb> {
    xml_tag("rb", text)(input).map(|(rem, (_attr, rb))| (rem, Rb(rb.into())))
}

fn parse_rt(input: &str) -> IResult<&str, Rt> {
    xml_tag("rt", text)(input).map(|(rem, (_attr, rt))| (rem, Rt(rt.into())))
}

pub fn parse_ruby(input: &str) -> IResult<&str, Ruby> {
    xml_tag("ruby", (parse_rb, parse_rt))(input).map(|(rem, (_attr, (rb, rt)))| (rem, Ruby(rb, rt)))
}

#[cfg(test)]
mod test {
    use crate::pxml::parse_hw;

    use super::*;
    #[test]
    fn ruby() {
        let s = "<span class=\"hw\">【<ruby><rb>悪</rb><rt>あく</rt></ruby><ruby><rb>辣</rb><rt>らつ</rt></ruby>】</span>";
        let (_rem, _res) = parse_hw(s).unwrap();
    }
    #[test]
    fn rub2() {
        let s = "<ruby><rb>悪</rb><rt>あく</rt></ruby>";
        let (_rem, _res) = parse_ruby(s).unwrap();
    }
}
