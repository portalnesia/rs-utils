/*
 * Copyright (c) Portalnesia - All Rights Reserved
 * Unauthorized copying of this file, via any medium is strictly prohibited
 * Proprietary and confidential
 * Written by Putu Aditya <aditya@portalnesia.com>
 */

use aes::Aes256;
use cbc::{Decryptor, Encryptor};
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use rand::rngs::OsRng;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Crypto instance
pub struct Crypto {
    key: Vec<u8>,
}

impl Crypto {
    /// Create new crypto instance with secret string
    ///
    /// ## Example
    ///
    /// ```
    /// let crypto = pn_utils::Crypto::new("this is secret key".to_string());
    /// ```
    pub fn new(secret: String) -> Self {
        let key = secret.as_bytes().to_vec();
        Crypto { key }
    }

    /// Encrypt string data
    ///
    /// ## Example
    ///
    /// ```
    /// let crypto = pn_utils::Crypto::new("this is secret key".to_string());
    /// let encrypted_data = crypto.encrypt("hidden text".to_string()).unwrap_or("failed".to_string());
    /// println!("{}",encrypted_data);
    /// ```
    pub fn encrypt(&self, data: String) -> Result<String, Box<dyn Error>> {
        let plain_text = data.as_bytes();

        // if self.key.len() != 16 {
        //     return Err("Key must be 16 bytes long.".into());
        // }

        // Generate IV secara acak
        let mut iv = [0u8; 16];
        // rand::rng().fill(&mut iv);
        let mut rng = OsRng;
        match rng.try_fill_bytes(&mut iv) {
            Ok(_) => {}
            Err(e) => return Err(e.to_string().into()),
        };

        // Inisialisasi encryptor
        let mut encryptor = match Encryptor::<Aes256>::new_from_slices(&self.key, &iv) {
            Ok(data) => data,
            Err(e) => return Err(e.to_string().into()),
        };

        // Tambahkan padding PKCS7
        let mut buffer = pad_pkcs7(plain_text, 16);

        // Proses blok-blok secara manual
        for chunk in buffer.chunks_mut(16) {
            encryptor.encrypt_block_mut(chunk.into());
        }

        // Gabungkan IV dengan ciphertext
        let mut result = Vec::new();
        result.extend_from_slice(&iv);
        result.extend_from_slice(&buffer);

        // Encode hasil ke dalam format hex
        let encrypted_text = hex::encode(result);
        let formatted = format!("{}:{}", &encrypted_text[..32], &encrypted_text[32..]);
        Ok(formatted)
    }

    /// Encrypt struct data
    ///
    /// Jsonify the data and encrypt
    pub fn encrypt_json<T>(&self, data: &T) -> Result<String, Box<dyn Error>>
    where
        T: ?Sized + Serialize,
    {
        use serde_json::to_string;

        let data_string = match to_string(data) {
            Ok(dt) => dt,
            Err(e) => return Err(e.into()),
        };

        self.encrypt(data_string)
    }

    /// Decrypt string data
    ///
    /// ## Example
    ///
    /// ```
    /// let crypto = pn_utils::Crypto::new("this is secret key".to_string());
    /// let encrypted_data = "0923gnj92bnwio9GJWIFWB"; // this is just an example
    /// let decrypted_data = crypto.decrypt(encrypted_data.to_string());
    /// println!("{}",decrypted_data.unwrap_or("failed".to_string()));
    /// ```
    pub fn decrypt(&self, encrypted: String) -> Result<String, Box<dyn Error>> {
        if encrypted.is_empty() {
            return Err("data is empty".into());
        }

        // Split encrypted data menjadi IV dan ciphertext
        let parts: Vec<&str> = encrypted.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid encrypted format. Expected ':' separator.".into());
        }

        // Gabungkan kembali bagian encrypted
        let encrypted_data = format!("{}{}", parts[0], parts[1]);
        let cipher_text = match hex::decode(encrypted_data) {
            Ok(data) => data,
            Err(e) => return Err(format!("Failed decode hex: {}", e))?,
        };

        if cipher_text.len() < 16 {
            return Err("Ciphertext too short.".into());
        }

        // Pisahkan IV dan ciphertext
        let iv = &cipher_text[..16];
        let cipher_text = &cipher_text[16..];

        // Pastikan ciphertext panjangnya kelipatan 16 (ukuran blok AES)
        if cipher_text.len() % 16 != 0 {
            return Err("Ciphertext is not a multiple of the block size.".into());
        }

        // Inisialisasi decryptor
        let mut decryptor = match Decryptor::<Aes256>::new_from_slices(&self.key, iv) {
            Ok(dt) => dt,
            Err(e) => return Err(e.to_string().into()),
        };

        // Buat buffer untuk dekripsi
        let mut buffer = cipher_text.to_vec();

        // Proses blok-blok secara manual
        for chunk in buffer.chunks_mut(16) {
            decryptor.decrypt_block_mut(chunk.into());
        }

        // Hapus padding PKCS7
        let unpadded = match unpad_pkcs7(&buffer) {
            Ok(dt) => dt,
            Err(e) => return Err(format!("Failed unpadded: {}", e))?,
        };
        let decrypted_text = match String::from_utf8(unpadded) {
            Ok(dt) => dt,
            Err(e) => return Err(format!("Failed to convert to string: {}", e))?,
        };
        Ok(decrypted_text)
    }

    /// Decrypt string to struct
    ///
    /// Decrypt data and parse to struct
    pub fn decrypt_json<T>(&self, data: String) -> Result<T, Box<dyn Error>>
    where
        T: for<'a> Deserialize<'a>,
    {
        use serde_json::from_str;

        let decrypted = self.decrypt(data)?;

        let data: T = match from_str(&decrypted) {
            Ok(data) => data,
            Err(err) => return Err(err.into()),
        };

        Ok(data)
    }
}

fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded_data = data.to_vec();
    padded_data.extend(vec![padding_len as u8; padding_len]);
    padded_data
}

fn unpad_pkcs7(data: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if let Some(&padding_len) = data.last() {
        if padding_len as usize > data.len() {
            return Err("Invalid padding".into());
        }
        let pad_start = data.len() - padding_len as usize;
        if data[pad_start..].iter().all(|&byte| byte == padding_len) {
            return Ok(data[..pad_start].to_vec());
        }
    }
    Err("Invalid padding".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct IData {
        name: String,
        age: usize,
    }

    #[test]
    fn encryption() {
        let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());
        let _encrypted = crypto
            .encrypt(String::from("halo ini data"))
            .expect("Failed to encrypt");
    }

    #[test]
    fn encryption_json() {
        let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());

        let _encrypted = crypto
            .encrypt_json(&IData {
                name: String::from("Putu"),
                age: 10,
            })
            .expect("Failed to encrypt");
    }

    #[test]
    fn decryption() {
        let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());

        let data = String::from("halo ini data");
        let encrypted = crypto.encrypt(data.clone()).expect("Failed to encrypt");

        let decrypted = crypto.decrypt(encrypted).expect("Failed to decrypt");
        assert_eq!(data, decrypted)
    }

    #[test]
    fn decryption_json() {
        let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());

        let origin_data = IData {
            name: String::from("Putu"),
            age: 10,
        };
        let _encrypted = crypto
            .encrypt_json(&origin_data)
            .expect("Failed to encrypt");
        let _decrypted: IData = crypto.decrypt_json(_encrypted).expect("Failet to decrypt");

        assert_eq!(origin_data.name, _decrypted.name);
        assert_eq!(origin_data.age, _decrypted.age);
    }
}
