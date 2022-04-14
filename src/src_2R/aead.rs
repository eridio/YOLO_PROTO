use aes_gcm_siv::{Key, Aes256GcmSiv, Nonce};
use aes_gcm_siv::aead::{NewAead, AeadInPlace};
use rand_core::{OsRng, RngCore};

pub fn encrypt(mk: &[u8; 32], plaintext: &[u8], associated_data: &[u8]) -> (Vec<u8>, [u8; 12]) {
    let key = Key::from_slice(mk);
    let cipher = Aes256GcmSiv::new(key);
    let mut nonce_data = [0_u8; 12];
    OsRng::fill_bytes(&mut OsRng, &mut nonce_data);
    let nonce = Nonce::from_slice(&nonce_data);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(plaintext);

    cipher.encrypt_in_place(nonce, associated_data, &mut buffer)
        .expect("Encryption failed");
    (buffer, nonce_data)
}

pub fn decrypt(mk: &[u8; 32], ciphertext: &[u8], associated_data: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
    let key = Key::from_slice(mk);
    let cipher = Aes256GcmSiv::new(key);

    let nonce = Nonce::from_slice(nonce);
    let mut buffer = Vec::new();
    buffer.extend_from_slice(ciphertext);
    cipher.decrypt_in_place(nonce, associated_data, &mut buffer).expect("Decryption failure {}");
    buffer
}
