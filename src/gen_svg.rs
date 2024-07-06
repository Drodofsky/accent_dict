use svg::Document;

pub fn gen_svg(accent_word: &str) -> String {
    let doc = Document::new();
    let mora = str_to_mora(accent_word);

    doc.to_string()
}

fn str_to_mora(word: &str) -> Vec<String> {
    let little = "ぁぅぇぉゃゅょァゥェォャュョ";
    let mut mora: Vec<String> = Vec::new();
    for c in word.chars() {
        if little.contains(c) {
            let l = mora.len().saturating_sub(1);
            mora[l].push(c)
        } else if c == '\u{20dd}' {
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
