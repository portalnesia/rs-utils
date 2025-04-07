extern crate utils;
use utils::helper::{truncate, clean, uuid,ALPHANUMERIC_CHARS};
use utils::{nanoid,first_letter};

fn main() {
    // truncate
    let long_text = "This is very very very long text";
    let truncated_text = truncate(long_text.to_string(), 25);
    println!("truncated: {}", truncated_text);

    // clean
    let html = r#"<p>Hello World</p>"#;
    let clean_text = clean(html.to_string());
    println!("Cleaned text: {}",clean_text);

    // uuid
    let uid = uuid();
    println!("uuid: {}",uid);

    // nanoid
    let nid = nanoid!();
    println!("simple nanoid: {}",nid);

    let nid = nanoid!(30);
    println!("nanoid with length: {}",nid);

    let nid = nanoid!(&ALPHANUMERIC_CHARS);
    println!("nanoid with custom characters: {}",nid);

    let nid = nanoid!(&ALPHANUMERIC_CHARS,30);
    println!("nanoid with custom characters and length: {}",nid);

    let fl = first_letter!("Hello World".to_string());
    println!("first letter: {}",fl);

    let fl = first_letter!("Hello World From Rust".to_string(),2);
    println!("first letter with max: {}",fl);
}
