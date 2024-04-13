/*
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use ring::rand::{SecureRandom, SystemRandom};

fn main() {
    let key_bytes = [0; 32]; // Replace this with your actual key bytes
    let key = UnboundKey::new(&CHACHA20_POLY1305, &key_bytes).unwrap();
    let nonce = Nonce::assume_unique_for_key([0; 12]); // Replace this with your actual nonce
    let key = LessSafeKey::new(key);

    let plaintext = b"Hello, world!";
    let mut in_out = plaintext.to_vec();
    in_out.resize(in_out.len() + key.algorithm().tag_len(), 0);
    let ciphertext_len = key.seal_in_place_separate_tag(nonce, Aad::empty(), &mut in_out).unwrap();
    in_out.truncate(ciphertext_len);
    println!("Ciphertext: {:?}", in_out);

    let mut in_out = in_out;
    in_out.resize(in_out.len() + key.algorithm().tag_len(), 0);
    let plaintext = key.open_in_place(nonce, Aad::empty(), &mut in_out).unwrap();
    println!("Decrypted plaintext: {:?}", plaintext);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn search_test() {

        let key = "howell";
        let iv = "1409";
        let pass = "Howell.net.cn1409";

        //let a = encrypt(key.as_bytes(), iv.as_bytes(), pass.as_bytes());
        //println!("a: {:?}", &a)

        //assert_eq!(v.binary_search(&6), Ok(16));
    }
}*/
