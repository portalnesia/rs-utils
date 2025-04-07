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
    use uuid::{Uuid,Timestamp,NoContext};
    let ts = Timestamp::now(NoContext);
    Uuid::new_v7(ts).to_string()
}

pub const ALPHANUMERIC_CHARS: [char; 62] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub const SAFE_CHARS: [char; 64] = [
    '_','-','0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];


pub fn parse_url(url: String) -> Result<String,ParseError> {
    let parsed = match Url::parse(&url) {
        Ok(url) => url,
        Err(err) => {
            return Err(err)
        }
    };

    let mut query = parsed.query().unwrap().to_string();

    if query != "" {
        query = format!("?{}",query);
    }

    let mut parser = format!("{}{}{}",parsed.host().unwrap(),parsed.path(),query);
    parser = parser.replacen("www.", "", 1);
    Ok(parser)
}

pub fn nanoid_format(chars: &[char],length: usize) -> String {
    use nanoid::nanoid as create_nanoid;

    let created = create_nanoid!(length,chars);

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
        $crate::helper::nanoid_format(&$crate::helper::SAFE_CHARS,15)
    };
    // nanoid with length parameter
    ($size:tt) => {
        $crate::helper::nanoid_format(&$crate::helper::SAFE_CHARS,$size)
    };
    // nanoid with custom characters and default length
    ($chars:expr) => {
        $crate::helper::nanoid_format($chars,15)
    };
    // nanoid with custom characters and length parameter
    ($chars:expr, $size:tt) => {
        $crate::helper::nanoid_format($chars,$size)
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
                .unwrap_or_default() + chars.as_str()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn first_letter_function(words: String, max: usize) -> String {
    let letters = words
        .split_whitespace()
        .filter_map(|word| {
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
        $crate::helper::first_letter_function($words,0)
    };
    ($words:expr,$size:tt) => {
        $crate::helper::first_letter_function($words,$size)
    };
}

/// Slug format string to slugify
///
/// Example: "hello world" => "hello-world"
fn slug(input: &str) -> String {
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

        let str = nanoid!(&['1','2','3','4','5']);
        assert_eq!(str.len(), 15);
        assert!(
            str.chars().all(|c| c.is_numeric()),
            "Generated data contains non-numeric characters: {}",
            str
        );

        let str = nanoid!(&['1','2','3','4','5'],20);
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

        assert_eq!(parsed,"portalnesia.com/news?foo=bar");

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
            ("Hello 123 $World", 2, "H"), // angka/simbol diabaikan
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
}
