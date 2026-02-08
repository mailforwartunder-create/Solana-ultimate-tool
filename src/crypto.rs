use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use argon2::{Argon2, PasswordHasher};

pub fn encrypt(data: &[u8], password: &str) -> Vec<u8> {
    let salt = "static_salt_for_now"; 
    let mut key_bytes = [0u8; 32];
    let argon2 = Argon2::default();
    let _ = argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key_bytes);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); 
    
    let ciphertext = cipher.encrypt(&nonce, data).expect("Encryption failed");
    let mut result = nonce.to_vec();
    result.extend_from_slice(&ciphertext);
    result
}

pub fn decrypt(encrypted_data: &[u8], password: &str) -> Result<Vec<u8>, String> {
    if encrypted_data.len() < 12 { return Err("Invalid data".into()); }
    
    let mut key_bytes = [0u8; 32];
    let salt = "static_salt_for_now";
    let argon2 = Argon2::default();
    let _ = argon2.hash_password_into(password.as_bytes(), salt.as_bytes(), &mut key_bytes);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&encrypted_data[..12]);
    let ciphertext = &encrypted_data[12..];

    cipher.decrypt(nonce, ciphertext).map_err(|_| "Wrong password".to_string())
}
