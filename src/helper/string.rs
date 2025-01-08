
/// Truncate string
///
/// ## Example
///
/// ```
/// let result = utils::helper::truncate("lorem ipsum lorem ipsum lorem ipsum".to_string(),18);
/// // result == lorem ipsum lor...
/// ```
pub fn truncate(text: String, max: usize) -> String {
    if max > text.len() {
        text
    } else {
        let max_size = max - 3;
        let str = format!("{}...", &text[..max_size]);
        str
    }
}

/// Clean string into unformatted HTML
///
/// ## Example
///
/// ```
/// let result = utils::helper::clean("XSS<script>attack</script>".to_string());
/// // result == "XSS"
///```
pub fn clean(text: String) -> String {
    use sanitize_html::sanitize_str;
    use sanitize_html::rules::predefined::DEFAULT;
    use regex::Regex;

    let cleaned = sanitize_str(&DEFAULT,text.as_str()).unwrap_or("".to_string());
    let re = Regex::new(r"^\s+|\s+$").unwrap();
    re.replace_all(&cleaned, "").to_string()
}

/// Clean text data and truncate
pub fn clean_truncate(text: String, max: usize) -> String {
    let cleaned = clean(text);
    truncate(cleaned,max)
}

/// Generate uuid
pub fn uuid() -> String {
    use uuid::Uuid;

    Uuid::new_v4().to_string()
}

/// Generate nanoid
///
/// Default length: 15
pub fn nanoid(length: Option<usize>) -> String {
    use nanoid::nanoid as create_nanoid;

    let len = length.unwrap_or(15);

    let created = create_nanoid!(len);
    created
}

/// Capitalize each first character in sentences
///
/// ## Example
///
/// ```
/// utils::helper::ucwords("hello world"); // Hello World
/// ```
pub fn ucwords(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .map(|c| c.to_uppercase().collect::<String>())
                .unwrap_or_default() + chars.as_str()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_long() {
        let long_text = "This is very very very long text";
        let truncated_text = truncate(long_text.to_string(), 25);
        assert_eq!(truncated_text, "This is very very very...".to_string());
    }

    #[test]
    fn test_truncate_short() {
        let long_text = "This is short text";
        let truncated_text = truncate(long_text.to_string(), 20);
        assert_eq!(truncated_text, long_text.to_string());
    }

    #[test]
    fn test_clean() {
        let html = r#"<p>Hello World</p>"#;
        let clean_text = clean(html.to_string());
        assert_eq!(clean_text, "Hello World".to_string());
    }

    #[test]
    fn test_clean_and_truncate() {
        let html = r#"<p>Hello World This is Long Text</p>"#;
        let clean_text = clean_truncate(html.to_string(),19);
        assert_eq!(clean_text, "Hello World This...".to_string());
    }

    #[test]
    fn test_ucwords() {
        let text = "hello world this is text";
        let result = ucwords(text);
        assert_eq!(result, "Hello World This Is Text".to_string());
    }
}
