use svg::{
    Document,
    node::element::{Circle, Path, Text, path::Data},
};

const CIRCLE: char = '\u{20dd}';
const VOICED: char = '\u{309a}';
const HALF_WIDTH_DAKUTEN: char = 'ﾞ';
const HALF_WIDTH_HANDAKUTEN: char = 'ﾟ';
const TEXT_STYLE: &'static str = "font-size:25px;font-family:sans-serif;fill:#fff;stroke:#000;stroke-width:2.2px;paint-order:stroke;";

pub fn gen_svg(accent_word: &str) -> String {
    let mut doc = Document::new();
    let mora = str_to_mora(accent_word);
    let mora_len = mora.len();
    let svg_width = (mora_len.saturating_sub(1) * 35) + 32;
    doc = doc.set("width", svg_width);
    doc = doc.set("height", 90);
    doc = doc.set("viewBox", (0, 0, svg_width, 90));

    // draw text
    for (pos, m) in mora
        .iter()
        .filter(|m| m.as_str() != "＼" && m.as_str() != "\u{20dd}" && m.as_str() != "▔")
        .enumerate()
    {
        let x = 16 + (pos * 35);
        println!("draw");
        doc = draw_mora(doc, m, x.saturating_sub(11))
    }

    // draw accent pattern
    // heiban
    if mora.last().unwrap() == "▔" {
        doc = draw_path(doc, 16, 40, PathType::Up, 35);
        doc = draw_circle(doc, 16, 40, false);

        for (i, m) in mora.iter().enumerate().skip(1) {
            let x = 16 + (i * 35);
            if m == "▔" {
                doc = draw_circle(doc, x, 15, true)
            } else {
                doc = draw_path(doc, x, 15, PathType::Straight, 35);
                doc = draw_circle(doc, x, 15, false)
            }
        }
    }
    /* atama daka */
    else if mora.get(1).unwrap() == "＼" {
        doc = draw_path(doc, 16, 15, PathType::Down, 35);
        doc = draw_circle(doc, 16, 15, false);
        let mut last_i = 0;
        for (i, _m) in mora
            .iter()
            .filter(|s| s.as_str() != "＼")
            .enumerate()
            .skip(1)
        {
            let x: usize = 16 + (i * 35);
            last_i = i;
            doc = draw_path(doc, x, 40, PathType::Straight, 35);
            doc = draw_circle(doc, x, 40, false)
        }
        let x: usize = 16 + ((last_i + 1) * 35);
        doc = draw_circle(doc, x, 40, true)
    }
    /* o daka */
    else if mora.last().unwrap() == "＼" {
        doc = draw_path(doc, 16, 40, PathType::Up, 35);
        doc = draw_circle(doc, 16, 40, false);
        let mut last_i = 0;
        for (i, _m) in mora.iter().skip(2).enumerate().skip(1) {
            let x: usize = 16 + (i * 35);
            last_i = i;
            doc = draw_path(doc, x, 15, PathType::Straight, 35);
            doc = draw_circle(doc, x, 15, false)
        }
        let x: usize = 16 + ((last_i + 1) * 35);

        doc = draw_path(doc, x, 15, PathType::Down, 35);

        doc = draw_circle(doc, x, 15, false);

        let x: usize = 16 + ((last_i + 2) * 35);
        doc = draw_circle(doc, x, 40, true)
    }
    /* naka daka */
    else {
        doc = draw_path(doc, 16, 40, PathType::Up, 35);
        doc = draw_circle(doc, 16, 40, false);
        let mut last_i = 0;
        let mut last_i2 = 0;
        for (i, _m) in mora
            .iter()
            .take_while(|s| s.as_str() != "＼")
            .skip(1)
            .enumerate()
            .skip(1)
        {
            let x: usize = 16 + (i * 35);
            last_i = i;
            doc = draw_path(doc, x, 15, PathType::Straight, 35);
            doc = draw_circle(doc, x, 15, false)
        }
        let x: usize = 16 + ((last_i + 1) * 35);
        doc = draw_path(doc, x, 15, PathType::Down, 35);
        doc = draw_circle(doc, x, 15, false);
        last_i = last_i + 1;
        for (i, _m) in mora
            .iter()
            .skip_while(|s| s.as_str() != "＼")
            .skip(1)
            .enumerate()
            .skip(1)
        {
            let x: usize = 16 + ((i + last_i) * 35);
            last_i2 = i;
            doc = draw_path(doc, x, 40, PathType::Straight, 35);
            doc = draw_circle(doc, x, 40, false)
        }
        let x: usize = 16 + ((last_i + last_i2 + 1) * 35);
        doc = draw_path(doc, x, 40, PathType::Straight, 35);

        doc = draw_circle(doc, x, 40, false);

        let x: usize = 16 + ((last_i + last_i2 + 2) * 35);
        doc = draw_circle(doc, x, 40, true)
    }

    println!("{}", doc.to_string());
    doc.to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PathType {
    Straight = 0,
    Up = -25,
    Down = 25,
}

fn draw_path(
    document: Document,
    xpos: usize,
    ypos: usize,
    typ: PathType,
    width: usize,
) -> Document {
    let data = Data::new()
        .move_to((xpos, ypos))
        .line_by((width, typ as isize));
    let inner = Path::new()
        .set("d", data.clone())
        .set("style", "fill:none;stroke:#fff;stroke-width:2.5;");
    let outer = Path::new()
        .set("d", data)
        .set("style", "fill:none;stroke:#000;stroke-width:4.7;");
    document.add(outer).add(inner)
}

pub fn draw_mora(mut doc: Document, mora: &str, xpos: usize) -> Document {
    let mora_len = mora
        .chars()
        .filter(|c| {
            c != &CIRCLE && c != &VOICED && c != &HALF_WIDTH_DAKUTEN && c != &HALF_WIDTH_HANDAKUTEN
        })
        .count();
    let text = if mora_len == 1 {
        Text::new(mora)
            .set("x", xpos)
            .set("y", 77.5)
            .set("style", TEXT_STYLE)
    } else {
        let little = "ぁぅぇぉゃゅょァゥェォャュョ";
        let (l, _) = mora.split_once(|c| little.contains(c)).unwrap();
        Text::new(l)
            .set("x", xpos.saturating_sub(5))
            .set("y", 77.5)
            .set("style", TEXT_STYLE)
    };

    doc = doc.add(text);
    if mora_len == 1 {
        return doc;
    }
    let little = "ぁぅぇぉゃゅょァゥェォャュョ";
    let index = mora.find(|c| little.contains(c)).unwrap();
    let t = Text::new(&mora[index..])
        .set("x", xpos.saturating_add(12))
        .set("y", 77.5)
        .set("style", TEXT_STYLE);

    doc.add(t)
}

fn draw_circle(mut doc: Document, xpos: usize, ypos: usize, is_next: bool) -> Document {
    let c_outer = Circle::new()
        .set("r", 7.2)
        .set("cx", xpos)
        .set("cy", ypos)
        .set("style", "opacity:1;fill:#000;");
    doc = doc.add(c_outer);

    let c = Circle::new()
        .set("r", 5)
        .set("cx", xpos)
        .set("cy", ypos)
        .set("style", "opacity:1;fill:#fff;");
    doc = doc.add(c);

    if is_next {
        let c = Circle::new()
            .set("r", 3.25)
            .set("cx", xpos)
            .set("cy", ypos)
            .set("style", "opacity:1;fill:#000;");
        return doc.add(c);
    }

    doc
}

fn str_to_mora(word: &str) -> Vec<String> {
    let little = "ぁぅぇぉゃゅょァゥェォャュョ";
    let mut mora: Vec<String> = Vec::new();
    for c in word.chars() {
        if little.contains(c) {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else if c == CIRCLE {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else if c == VOICED {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else if c == HALF_WIDTH_DAKUTEN {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else if c == HALF_WIDTH_HANDAKUTEN {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else {
            mora.push(c.to_string())
        }
    }

    mora
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mora() {
        assert_eq!(
            str_to_mora("きく"),
            vec!["き".to_string(), "く".to_string()]
        );
        assert_eq!(
            str_to_mora("キク"),
            vec!["キ".to_string(), "ク".to_string()]
        );
        assert_eq!(
            str_to_mora("㋖ク"),
            vec!["㋖".to_string(), "ク".to_string()]
        );
        assert_eq!(
            str_to_mora("き⃝く"),
            vec!["き⃝".to_string(), "く".to_string()]
        );
        assert_eq!(
            str_to_mora("きょく"),
            vec!["きょ".to_string(), "く".to_string()]
        );
        assert_eq!(
            str_to_mora("キョク"),
            vec!["キョ".to_string(), "ク".to_string()]
        );
    }
}
