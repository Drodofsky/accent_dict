pub fn to_circle(input: &str) -> String {
    let mut circled = String::new();
    for c in input.chars() {
        match c {
            'ア' => circled.push('㋐'),
            'イ' => circled.push('㋑'),
            'ウ' => circled.push('㋒'),
            'エ' => circled.push('㋓'),
            'オ' => circled.push('㋔'),
            'カ' => circled.push('㋕'),
            'キ' => circled.push('㋖'),
            'ク' => circled.push('㋗'),
            'ケ' => circled.push('㋘'),
            'コ' => circled.push('㋙'),
            'サ' => circled.push('㋚'),
            'シ' => circled.push('㋛'),
            'ス' => circled.push('㋜'),
            'セ' => circled.push('㋝'),
            'ソ' => circled.push('㋞'),
            'タ' => circled.push('㋟'),
            'チ' => circled.push('㋠'),
            'ツ' => circled.push('㋡'),
            'テ' => circled.push('㋢'),
            'ト' => circled.push('㋣'),
            'ナ' => circled.push('㋤'),
            'ニ' => circled.push('㋥'),
            'ヌ' => circled.push('㋦'),
            'ネ' => circled.push('㋧'),
            'ノ' => circled.push('㋨'),
            'ハ' => circled.push('㋩'),
            'ヒ' => circled.push('㋪'),
            'フ' => circled.push('㋫'),
            'ヘ' => circled.push('㋬'),
            'ホ' => circled.push('㋭'),
            'マ' => circled.push('㋮'),
            'ミ' => circled.push('㋯'),
            'ム' => circled.push('㋰'),
            'メ' => circled.push('㋱'),
            'モ' => circled.push('㋲'),
            'ヤ' => circled.push('㋳'),
            'ユ' => circled.push('㋴'),
            'ヨ' => circled.push('㋵'),
            'ラ' => circled.push('㋶'),
            'リ' => circled.push('㋷'),
            'ル' => circled.push('㋸'),
            'レ' => circled.push('㋹'),
            'ロ' => circled.push('㋺'),
            'ワ' => circled.push('㋻'),
            _ => {
                circled.push(c);
                circled.push('\u{20DD}');
            }
        }
    }
    circled
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn katakana() {
        assert_eq!(to_circle("アイオキノソルワ"), "㋐㋑㋔㋖㋨㋞㋸㋻")
    }
    #[test]
    fn hiragana() {
        assert_eq!(to_circle("あいおきのそるわ"), "あ⃝い⃝お⃝き⃝の⃝そ⃝る⃝わ⃝")
    }
}
