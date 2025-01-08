extern crate utils;

use crate::utils::Crypto;

fn main() {
    let data = String::from("Ini adalah data rahasia. Tidak boleh ada yang tahu!");

    let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());

    let encrypted = crypto.encrypt(data.clone()).expect("Failed to encrypt");
    println!("Encrypted data: {}", encrypted);

    let decrypted = crypto.decrypt(encrypted).expect("Failed to decrypt");
    println!("Decrypted data: {}", decrypted);
}
