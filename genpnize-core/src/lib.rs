pub fn chop_text(text: &str, n: usize) -> Vec<String> {
    let characters: Vec<char> = text.chars().collect();
    characters
        .chunks(n)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect()
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
}
