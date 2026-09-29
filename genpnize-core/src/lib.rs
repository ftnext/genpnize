pub fn genpnize(text: &str) -> String {
    let lines = chop_text(text, 13);
    lines.join("\n")
}

fn chop_text(text: &str, n: usize) -> Vec<String> {
    let characters: Vec<char> = text.chars().collect();
    let mut lines: Vec<String> = characters
        .chunks(n)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect();
    if lines.len() >= 2 {
        let punctuation = match lines.last().unwrap().as_str() {
            "?" | "？" => Some('?'),
            "!" | "！" => Some('!'),
            _ => None,
        };
        if let Some(mark) = punctuation {
            lines.pop();
            lines.last_mut().unwrap().push(mark);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chop_text() {
        let text = "あいうえおかきくけこさしすせそ";
        assert_eq!(
            chop_text(text, 13),
            vec!["あいうえおかきくけこさしす", "せそ"]
        );
    }

    #[test]
    fn test_should_include_trailing_punctuation() {
        let text = "あいうえおかきくけこさしす？";
        assert_eq!(chop_text(text, 13), vec!["あいうえおかきくけこさしす?"]);
    }
}
