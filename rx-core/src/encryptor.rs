use aes::Aes128;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex_literal::hex;
use std::str;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

/// AES 加/解密器
///
/// 对称加密算法，用于加密配置文件中的敏感信息
pub struct AesEncryptor {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl AesEncryptor {
    pub fn new(key: Vec<u8>, iv: Vec<u8>) -> Self {
        Self { key, iv }
    }

    pub fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        let cipher = Aes128Cbc::new_from_slices(&self.key, &self.iv).unwrap();
        cipher.encrypt_vec(plaintext.as_bytes())
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> String {
        let cipher = Aes128Cbc::new_from_slices(&self.key, &self.iv).unwrap();
        let decrypted_ciphertext = cipher.decrypt_vec(ciphertext).unwrap();
        str::from_utf8(&decrypted_ciphertext).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = hex!("000102030405060708090a0b0c0d0e0f").to_vec();
        let iv = hex!("101112131415161718191a1b1c1d1e1f").to_vec();

        let encryptor = AesEncryptor::new(key, iv);

        let plaintext = "Hello, AES encryption!";
        let ciphertext = encryptor.encrypt(plaintext);
        let decrypted_text = encryptor.decrypt(&ciphertext);

        assert_eq!(plaintext, decrypted_text);
    }
}
