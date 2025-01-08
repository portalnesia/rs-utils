use aes::Aes256;
use cbc::{Decryptor, Encryptor};
use cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex;
use rand::Rng;
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
    /// let crypto = utils::Crypto::new("this is secret key".to_string());
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
    /// let crypto = utils::Crypto::new("this is secret key".to_string());
    /// let encrypted_data = crypto.encrypt("hidden text".to_string()).expect("error when encrypting data");
    /// println!("{}",encrypted_data);
    /// ```
    pub fn encrypt(&self, data: String) -> Result<String, Box<dyn Error>> {
        let plain_text = data.as_bytes();

        // if self.key.len() != 16 {
        //     return Err("Key must be 16 bytes long.".into());
        // }

        // Generate IV secara acak
        let mut iv = [0u8; 16];
        rand::thread_rng().fill(&mut iv);

        // Inisialisasi encryptor
        let mut encryptor = Encryptor::<Aes256>::new_from_slices(&self.key, &iv).unwrap();

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

    /// Decrypt string data
    ///
    /// ## Example
    ///
    /// ```
    /// let crypto = utils::Crypto::new("this is secret key".to_string());
    /// let encrypted_data = "0923gnj92bnwio9GJWIFWB"; // this is just an example
    /// let decrypted_data = crypto.decrypt(encrypted_data.to_string());
    /// println!("{}",decrypted_data);
    /// ```
    pub fn decrypt(&self, encrypted: String) -> Result<String, Box<dyn Error>> {
        if encrypted.is_empty() {
            return Ok(String::new());
        }

        // Split encrypted data menjadi IV dan ciphertext
        let parts: Vec<&str> = encrypted.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid encrypted format. Expected ':' separator.".into());
        }

        // Gabungkan kembali bagian encrypted
        let encrypted_data = format!("{}{}", parts[0], parts[1]);
        let cipher_text = hex::decode(encrypted_data)?;

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
        let mut decryptor = Decryptor::<Aes256>::new_from_slices(&self.key, iv).unwrap();

        // Buat buffer untuk dekripsi
        let mut buffer = cipher_text.to_vec();

        // Proses blok-blok secara manual
        for chunk in buffer.chunks_mut(16) {
            decryptor.decrypt_block_mut(chunk.into());
        }

        // Hapus padding PKCS7
        let unpadded = unpad_pkcs7(&buffer)?;
        let decrypted_text = String::from_utf8(unpadded)?;
        Ok(decrypted_text)
    }
}

fn pad_pkcs7(data: &[u8], block_size: usize) -> Vec<u8> {
    let padding_len = block_size - (data.len() % block_size);
    let mut padded_data = data.to_vec();
    padded_data.extend(vec![padding_len as u8; padding_len]);
    padded_data
}

fn unpad_pkcs7(data: &[u8]) -> Result<Vec<u8>, &'static str> {
    if let Some(&padding_len) = data.last() {
        if padding_len as usize > data.len() {
            return Err("Invalid padding");
        }
        let pad_start = data.len() - padding_len as usize;
        if data[pad_start..].iter().all(|&byte| byte == padding_len) {
            return Ok(data[..pad_start].to_vec());
        }
    }
    Err("Invalid padding")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encryption() {
        let crypto = Crypto::new("c67106b30d41345119309c05d1c4ab28".to_string());
        let _encrypted = crypto
            .encrypt(String::from("halo ini data"))
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
}
