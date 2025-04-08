/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

extern crate pn_utils;

use crate::pn_utils::Crypto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    age: usize,
}

fn main() {
    let data = String::from("Ini adalah data rahasia. Tidak boleh ada yang tahu!");

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
