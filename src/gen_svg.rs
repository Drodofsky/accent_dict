use svg::{node::element::Text, Document};

const CIRCLE: char = '\u{20dd}';
const VOICED: char = '\u{309a}';

pub fn gen_svg(accent_word: &str) -> String {
    let mut doc = Document::new();
    let mora = str_to_mora(accent_word);
    let voiced_count = mora
        .iter()
        .map(|c| c.chars())
        .flatten()
        .filter(|c| c == &VOICED)
        .count();

    let mora_len = if mora.contains(&"＼".to_string()) {
        mora.len() - 1
    } else {
        mora.len()
    }
    .saturating_sub(voiced_count);
    let svg_width = (mora_len.saturating_sub(1) * 35) + 32;
    doc = doc.set("width", svg_width);
    doc = doc.set("height", 75);
    doc = doc.set("viewBox", (0, 0, svg_width, 75));

    for (pos, m) in mora
        .iter()
        .filter(|m| m.as_str() != "＼" && m.as_str() != "\u{20dd}" && m.as_str() != "▔")
        .enumerate()
    {
        let x = 16 + (pos * 35);
        println!("draw");
        doc = draw_mora(doc, m, x.saturating_sub(11))
    }
    println!("{}", doc.to_string());
    doc.to_string()
}

pub fn draw_mora(mut doc: Document, mora: &str, xpos: usize) -> Document {
    let mora_len = mora
        .chars()
        .filter(|c| c != &CIRCLE && c != &VOICED)
        .count();
    let text = if mora_len == 1 {
        Text::new(mora)
            .set("x", xpos.saturating_add(5))
            .set("y", 67.5)
            .set("style", "font-size:20px;font-family:sans-serif;fill:#fff;")
    } else {
        let little = "ぁぅぇぉゃゅょァゥェォャュョ";
        let (l, _) = mora.split_once(|c| little.contains(c)).unwrap();
        Text::new(l)
            .set("x", xpos.saturating_sub(5))
            .set("y", 67.5)
            .set("style", "font-size:20px;font-family:sans-serif;fill:#fff;")
    };

    doc = doc.add(text);
    if mora_len == 1 {
        return doc;
    }
    let little = "ぁぅぇぉゃゅょァゥェォャュョ";
    let index = mora.find(|c| little.contains(c)).unwrap();
    let t = Text::new(&mora[index..])
        .set("x", xpos.saturating_add(12))
        .set("y", 67.5)
        .set("style", "font-size:20px;font-family:sans-serif;fill:#fff;");

    doc.add(t)
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
