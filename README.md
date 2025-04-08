# rs-utils

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
![Test](https://github.com/portalnesia/rs-utils/actions/workflows/rust.yml/badge.svg)

[crates-badge]: https://img.shields.io/crates/v/pn_utils.svg

[crates-url]: https://crates.io/crates/pn_utils

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[mit-url]: https://github.com/portalnesia/rs-utils/blob/main/LICENSE

This is a utility library for Portalnesia projects. It contains various helper functions and components that are
commonly used across different parts of the application.

## Modules

### Crypto

The `crypto` module provides cryptographic functionalities for securing data. It includes
features encryption, and decryption.

#### Features

* **Encryption/Decryption:**
    * `encrypt`: Encrypts data using AES-256-GCM.
    * `encrypt_json`: Jsonify and encrypts a struct
    * `decrypt`: Decrypts data encrypted with AES-256-GCM.
    * `decrypt_json`: Decrypt data and parse to struct

#### Usage

```rs
use pn_utils::Crypto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    age: usize,
}

fn main() {
    let data = String::from("This is secret data");

    # Initiate crypto with secret key
    let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());

    let encrypted = crypto.encrypt(data.clone()).expect("Failed to encrypt");
    println!("Encrypted data: {}", encrypted);

    let decrypted = crypto.decrypt(encrypted).expect("Failed to decrypt");
    println!("Decrypted data: {}", decrypted);
    
    let encrypted = crypto
        .encrypt_json(&Data {
            name: "Name".to_string(),
            age: 27,
        })
        .expect("Failed to encrypt data");
    println!("Encrypted data: {}", encrypted);

    let decrypted_json: Data = crypto.decrypt_json(encrypted).expect("Failed to decrypt");
    println!("Decrypted data: {:?}", decrypted_json);
}
```

### Password

The `password` module provides functionalities for securely handling passwords. It includes features for hashing
passwords and verifying them against their hashed versions.

#### Features

* **Hashing:**
    * `hash_password`: Securely hashes a password using Argon2.
    * `compare_password`: Verifies a password against its hashed version.

#### Usage

```rs
use pn_utils::password::{compare_password, hash_password};

fn main() {
    let password = "my_secret_password";
    let hashed_password = hash_password(password.to_string()).expect("failed to hashed password");
    println!(
        "password: {}\nhashed password: {}",
        password,
        hashed_password.clone()
    );

    let is_password_valid = compare_password(password.to_string(), hashed_password.clone());
    println!("Is password valid? {}", is_password_valid)
}
```

### Helper

The `helper` module provides a collection of general-purpose utility functions and macros that are useful across various
parts of the application. This module aims to reduce code duplication and provide convenient tools for common tasks.

#### Features

##### String Manipulation

* `truncate`: Truncates a string to a maximum length, adding "..." at the end if the string is longer than `max`.
* `clean`: Cleans a string by removing HTML tags and extra whitespace.
* `clean_truncate`: Combines `clean` and `truncate` to clean a string and then truncate it to a maximum length.
* `ucwords`: Capitalizes the first letter of each word in a sentence.
* `capitalize_first`: Capitalizes the first character of a string.
* `slug`: Converts a string to a URL-friendly slug.
* `first_letter!`: Macro to extract the first letter of each word in a string and return them in uppercase.
* `parse_url`: Parses a URL string and returns a simplified version (host, path, query).

##### ID Generation

* `uuid`: Generates a UUID v7 string.
* `nanoid!`: Macro to generate a nanoid.

##### Check

* `is_url`: Checks if a given string is a valid URL.
* `is_twitter_url`: Checks if a given string is a valid Twitter URL.
* `is_true!`: Macro to check the "truthiness" of a value
* `validate_email`: Validates if a given string is a valid email address.

#### Number Formatter

* `bytes_format!`: Macro to format float64 bytes to human-readable string.
* `format_number!`: Macro to format float64 with a thousand separator.
* `format_short_number`: Format a number into a short format.

### Usage

```rs
use pn_utils::helper::{clean, truncate, uuid, ALPHANUMERIC_CHARS};
use pn_utils::{first_letter, is_true, nanoid};

fn main() {
    // truncate
    let long_text = "This is very very very long text";
    let truncated_text = truncate(long_text.to_string(), 25);
    println!("truncated: {}", truncated_text);

    // clean
    let html = r#"<p>Hello World</p>"#;
    let clean_text = clean(html.to_string());
    println!("Cleaned text: {}", clean_text);

    // uuid
    let uid = uuid();
    println!("uuid: {}", uid);

    // nanoid
    let nid = nanoid!();
    println!("simple nanoid: {}", nid);

    let nid = nanoid!(30);
    println!("nanoid with length: {}", nid);

    let nid = nanoid!(&ALPHANUMERIC_CHARS);
    println!("nanoid with custom characters: {}", nid);

    let nid = nanoid!(&ALPHANUMERIC_CHARS, 30);
    println!("nanoid with custom characters and length: {}", nid);

    let fl = first_letter!("Hello World".to_string());
    println!("first letter: {}", fl);

    let fl = first_letter!("Hello World From Rust".to_string(), 2);
    println!("first letter with max: {}", fl);

    let t = is_true!("false");
    println!("is_true: {}", t);
}
```