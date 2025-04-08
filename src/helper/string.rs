/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

use regex::Regex;
use url::{ParseError, Url};

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
    use regex::Regex;
    use sanitize_html::rules::predefined::DEFAULT;
    use sanitize_html::sanitize_str;

    let cleaned = sanitize_str(&DEFAULT, text.as_str()).unwrap_or("".to_string());
    let re = Regex::new(r"^\s+|\s+$").unwrap();
    re.replace_all(&cleaned, "").to_string()
}

/// Clean text data and truncate
///
/// ## Example
///
/// ```
/// let result = utils::helper::clean_truncate("XSS<script>attack</script>".to_string(), 5);
/// // result == "XSS..."
///
/// let result = utils::helper::clean_truncate("XSS<script>attack</script>".to_string(), 10);
/// // result == "XSS"
/// ```
/// Clean text data and truncate
pub fn clean_truncate(text: String, max: usize) -> String {
    let cleaned = clean(text);
    truncate(cleaned, max)
}

/// Generate uuid
///
/// ## Example
///
/// ```
/// use utils::helper;
///
/// let id = helper::uuid();
///
/// println!("UUID: {}", id);
/// ```
pub fn uuid() -> String {
    use uuid::{NoContext, Timestamp, Uuid};
    let ts = Timestamp::now(NoContext);
    Uuid::new_v7(ts).to_string()
}

/// An array of alphanumeric characters.
///
/// This constant contains all uppercase and lowercase letters, as well as the digits 0-9.
/// It is commonly used for generating random strings or identifiers.
pub const ALPHANUMERIC_CHARS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];

/// An array of safe characters.
///
/// This constant contains alphanumeric characters, as well as the underscore and hyphen.
/// It is commonly used for generating random strings or identifiers that are safe for use in URLs or file names.
pub const SAFE_CHARS: [char; 64] = [
    '_', '-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// Parses a URL string and returns a simplified version of the URL.
///
/// This function takes a URL string as input and returns a simplified version of the URL.
/// The simplified URL consists of the host, path, and query parameters, with the "www." prefix removed.
///
/// # Arguments
///
/// * `url` - The URL string to parse.
///
pub fn parse_url(url: String) -> Result<String, ParseError> {
    let parsed = match Url::parse(&url) {
        Ok(url) => url,
        Err(err) => return Err(err),
    };

    let mut query = parsed.query().unwrap().to_string();

    if !query.is_empty() {
        query = format!("?{}", query);
    }

    let mut parser = format!("{}{}{}", parsed.host().unwrap(), parsed.path(), query);
    parser = parser.replacen("www.", "", 1);
    Ok(parser)
}

pub fn nanoid_format(chars: &[char], length: usize) -> String {
    use nanoid::nanoid as create_nanoid;

    let created = create_nanoid!(length, chars);

    created
}
/// Generate nanoid
///
/// ### 1. Simple nanoid
///
/// Example
///
/// ```
/// let result = utils::nanoid!();
/// ```
///
/// ### 2. nanoid with length parameter
///
/// Default chars: alphanumeric, -, _
///
/// Example
///
/// ```
/// let result = utils::nanoid!(30);
///```
///
/// ### 3. nanoid with custom characters and default length
///
/// Default length: 15
///
/// Example
///
/// ```
/// let result = utils::nanoid!(&['1','2','3','4','5']);
///
/// let result = utils::nanoid!(&utils::helper::ALPHANUMERIC_CHARS);
///```
///
/// ### 4. nanoid with custom characters and length parameter
///
/// ## Example
///
/// ```
/// let result = utils::nanoid!(&utils::helper::ALPHANUMERIC_CHARS,30);
///```
#[macro_export]
macro_rules! nanoid {
    // simple nanoid
    () => {
        $crate::helper::nanoid_format(&$crate::helper::SAFE_CHARS, 15)
    };
    // nanoid with length parameter
    ($size:tt) => {
        $crate::helper::nanoid_format(&$crate::helper::SAFE_CHARS, $size)
    };
    // nanoid with custom characters and default length
    ($chars:expr) => {
        $crate::helper::nanoid_format($chars, 15)
    };
    // nanoid with custom characters and length parameter
    ($chars:expr, $size:tt) => {
        $crate::helper::nanoid_format($chars, $size)
    };
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
                .unwrap_or_default()
                + chars.as_str()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn first_letter_function(words: String, max: usize) -> String {
    let letters = words.split_whitespace().filter_map(|word| {
        word.chars()
            .next()
            .filter(|c| c.is_alphabetic())
            .map(|c| c.to_ascii_uppercase())
    });

    if max == 0 {
        letters.collect()
    } else {
        letters.take(max).collect()
    }
}

/// FirstLetter parse string to first letter uppercase
///
/// Example
///
/// ```
/// let letter = utils::first_letter!("Hello world".to_string()); // HW
///
/// let letter = utils::first_letter!("Hello world from rust".to_string(),2); // HW
/// ```
#[macro_export]
macro_rules! first_letter {
    ($words:expr) => {
        $crate::helper::first_letter_function($words, 0)
    };
    ($words:expr,$size:tt) => {
        $crate::helper::first_letter_function($words, $size)
    };
}

/// Slug format string to slugify
///
/// Example: "hello world" => "hello-world"
pub fn slug(input: &str) -> String {
    input
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c)
            } else if c.is_whitespace() || c == '-' || c == '_' {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Checks if a given string is a valid URL.
///
/// # Arguments
///
/// * `url` - The URL string to check.
///
/// # Example
///
/// ```
/// use utils::helper::is_url;
///
/// assert_eq!(is_url("https://portalnesia.com".to_string()), true);
/// assert_eq!(is_url("invalid-url".to_string()), false);
/// ```
pub fn is_url(url: String) -> bool {
    Url::parse(&url).is_ok()
}

/// Checks if a given string is a valid Twitter URL.
///
/// # Arguments
///
/// * `url` - The URL string to check.
///
pub fn is_twitter_url(url: String) -> bool {
    if !is_url(url.clone()) {
        return false;
    }

    let re = Regex::new(r"^https?://(www.)?twitter\.com").unwrap();
    re.is_match(url.as_str())
}

/// Capitalizes the first character of a string.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
///
/// # Example
///
/// ```
/// assert_eq!(utils::helper::capitalize_first("hello".to_string()), "Hello");
/// ```

pub fn capitalize_first(s: String) -> String {
    let trimmed = s.trim_start();
    let mut chars = trimmed.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            // to_uppercase() bisa menghasilkan lebih dari satu char (Unicode)
            let mut result = first.to_uppercase().collect::<String>();
            result.push_str(chars.as_str());
            result
        }
    }
}

/// Validates if a given string is a valid email address.
///
/// # Arguments
///
/// * `email` - The email string to validate.
///
/// # Example
///
/// ```
/// assert_eq!(utils::helper::validate_email("support@portalnesia.com".to_string()), true);
/// assert_eq!(utils::helper::validate_email("invalid-email".to_string()), false);
/// ```
pub fn validate_email(email: String) -> bool {
    let re = Regex::new(r"^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,3})+$").unwrap();
    re.is_match(email.as_str())
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
        let clean_text = clean_truncate(html.to_string(), 19);
        assert_eq!(clean_text, "Hello World This...".to_string());
    }

    #[test]
    fn test_uuid() {
        let uuid = uuid();
        assert_eq!(uuid.len(), 36);
    }

    #[test]
    fn test_nanoid() {
        let str = nanoid!();
        assert_eq!(str.len(), 15);

        let str = nanoid!(30);
        assert_eq!(str.len(), 30);

        let str = nanoid!(&['1', '2', '3', '4', '5']);
        assert_eq!(str.len(), 15);
        assert!(
            str.chars().all(|c| c.is_numeric()),
            "Generated data contains non-numeric characters: {}",
            str
        );

        let str = nanoid!(&['1', '2', '3', '4', '5'], 20);
        assert_eq!(str.len(), 20);
        assert!(
            str.chars().all(|c| c.is_numeric()),
            "Generated data contains non-numeric characters: {}",
            str
        );
    }

    #[test]
    fn test_ucwords() {
        let text = "hello world this is text";
        let result = ucwords(text);
        assert_eq!(result, "Hello World This Is Text".to_string());
    }

    #[test]
    fn test_parse_url() {
        let url = "https://www.portalnesia.com/news?foo=bar".to_string();

        let parsed = parse_url(url).expect("Failed parser url");

        assert_eq!(parsed, "portalnesia.com/news?foo=bar");

        assert!(parse_url("error url".to_string()).is_err());
        assert!(parse_url("https://err. https://".to_string()).is_err());
    }

    #[test]
    fn test_first_letter() {
        let cases = vec![
            ("hello world", 0, "HW"),
            ("Rust language", 0, "RL"),
            ("   hello   rust   world  ", 0, "HRW"),
            ("", 0, ""),
            ("golang", 0, "G"),
            ("123 apples $banana", 0, "A"),
            ("#rust code!", 0, "C"),
            ("Hello World From Rust", 2, "HW"),
            ("Hello World From Rust", 3, "HWF"),
            ("Hello World From Rust", 5, "HWFR"), // lebih dari jumlah kata yang valid
            ("Hello 123 $World", 2, "H"),         // angka/simbol diabaikan
        ];

        for (input, max, expected) in cases {
            let result = first_letter!(input.to_string(), max);
            assert_eq!(
                result, expected,
                "first_letter({:?}, {}) should be {:?}, got {:?}",
                input, max, expected, result
            );
        }
    }

    #[test]
    fn test_slug() {
        let cases = vec![
            ("Hello World", "hello-world"),
            ("Rust is awesome!", "rust-is-awesome"),
            ("  Hello---Rust___World!! ", "hello-rust-world"),
            ("Satu dua tiga empat", "satu-dua-tiga-empat"),
            ("Clean & Simple", "clean-simple"),
            ("Symbols #should @be $gone!", "symbols-should-be-gone"),
            ("MiXeD CaSe and123Numbers", "mixed-case-and123numbers"),
            ("--Already--Sluggy--", "already-sluggy"),
            ("", ""),
        ];

        for (input, expected) in cases {
            let result = slug(input);
            assert_eq!(
                result, expected,
                "slug({:?}) should be {:?}, got {:?}",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_is_url() {
        let cases = vec![("https://", false), ("https://portalnesia.com", true)];
        for (input, expected) in cases {
            let result = is_url(input.to_string());
            assert_eq!(
                result, expected,
                "is_url({:?}) should be {:?}, got {:?}",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_is_twitter_url() {
        let cases = vec![
            ("http://portalnesia.com/twitter.com/contact", false),
            ("http://portalnesia.com/contact", false),
            ("https://twitter.com/Portalnesia1", true),
        ];
        for (input, expected) in cases {
            let result = is_twitter_url(input.to_string());
            assert_eq!(
                result, expected,
                "is_url({:?}) should be {:?}, got {:?}",
                input, expected, result
            );
        }
    }

    #[test]
    fn test_capitalize_first() {
        let cases = vec![
            ("hello world".to_string(), "Hello world"),
            ("Hello world".to_string(), "Hello world"),
            ("rust".to_string(), "Rust"),
            ("r".to_string(), "R"),
            ("".to_string(), ""),
            // unicode
            ("äpfel sind lecker".to_string(), "Äpfel sind lecker"),
            // non-letter first char tetap dipertahankan
            ("123abc".to_string(), "123abc"),
            (" multiple spaces".to_string(), "Multiple spaces"),
        ];

        for (input, expected) in cases {
            let got = capitalize_first(input.clone());
            assert_eq!(
                got, expected,
                "capitalize_first({:?}) should be {:?}, got {:?}",
                input, expected, got
            );
        }
    }

    #[test]
    fn test_validate_email() {
        let cases = vec![
            ("support@portalnesia".to_string(), false),
            ("support@portalnesia.com".to_string(), true),
            ("  support@portalnesia.com".to_string(), false),
            ("support@portalnesia.com  ".to_string(), false),
        ];
        for (input, expected) in cases {
            let got = validate_email(input.clone());
            assert_eq!(
                got, expected,
                "validate_email({:?}) should be {:?}, got {:?}",
                input, expected, got
            );
        }
    }
}
