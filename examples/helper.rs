extern crate utils;

use crate::utils::helper::{truncate,clean};

fn main() {
    // truncate
    let long_text = "This is very very very long text";
    let truncated_text = truncate(long_text.to_string(), 25);
    println!("truncated: {}", truncated_text);

    let html = r#"<p>Hello World</p>"#;
    let clean_text = clean(html.to_string());
    println!("Cleaned text: {}",clean_text);
}
