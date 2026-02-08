use solana_sdk::signature::{Keypair, Signer};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::convert::TryInto;
use crate::crypto;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WalletInfo {
    pub address: String,
    pub private_key: Vec<u8>,
}

pub fn generate_batch(count: usize, password: &str) {
    let mut list = load_from_file("wallets.enc", password).unwrap_or_else(|_| Vec::new());
    
    for _ in 0..count {
        let kp = Keypair::new();
        list.push(WalletInfo {
            address: kp.pubkey().to_string(),
            private_key: kp.to_bytes().to_vec(),
        });
    }

    if let Ok(json_string) = serde_json::to_string_pretty(&list) {
        let encrypted_data = crypto::encrypt(json_string.as_bytes(), password);
        let _ = fs::write("wallets.enc", encrypted_data);
        println!("✅ generated. total in the database: {}", list.len());
    }
}

pub fn load_from_file(filename: &str, password: &str) -> io::Result<Vec<WalletInfo>> {
    let encrypted_data = fs::read(filename)?;
    
    match crypto::decrypt(&encrypted_data, password) {
        Ok(decrypted_bytes) => {
            let wallets: Vec<WalletInfo> = serde_json::from_slice(&decrypted_bytes)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
            Ok(wallets)
        }
        Err(_) => Err(io::Error::new(io::ErrorKind::PermissionDenied, "Ошибка расшифровки")),
    }
}

pub fn import_from_txt(path_txt: &str, password: &str) -> io::Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    println!("📍 Мой терминал сейчас в: {:?}", current_dir);
    // 1. Пытаемся загрузить существующие, если файла нет - создаем пустой вектор
    let mut list = load_from_file("wallets.enc", password).unwrap_or_else(|_| Vec::new());
    
    // 2. Читаем текстовик. Если его нет - вот тут реально будет ошибка
    let content = fs::read_to_string(path_txt)
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, format!("Файл {} не найден!", path_txt)))?;
    
    let mut imported_count = 0;

    for line in content.lines() {
        let trim = line.trim();
        if trim.is_empty() { continue; }

        // Декодируем Base58
        if let Ok(key_vec) = bs58::decode(trim).into_vec() {
            // Пытаемся преобразовать в массив 64 байта
            if let Ok(key_array) = key_vec.try_into() {
                let key_array: [u8; 64] = key_array; // Фиксируем тип
                
                if let Ok(kp) = Keypair::from_bytes(&key_array) {
                    let addr = kp.pubkey().to_string();

                    if !list.iter().any(|w| w.address == addr) {
                        list.push(WalletInfo {
                            address: addr.clone(),
                            private_key: key_array.to_vec(),
                        });
                        imported_count += 1;
                        println!("   [+] added: {}...", &addr[..8]);
                    }
                }
            } else {
                println!("   [!] skip: the key must be 64 bytes (Base58)");
            }
        }
    }

    // 3. Сохраняем, если что-то добавили
    if imported_count > 0 {
        let json_data = serde_json::to_string_pretty(&list)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        
        let encrypted_data = crypto::encrypt(json_data.as_bytes(), password);
        fs::write("wallets.enc", encrypted_data)?;
        println!("\n✅ import completed: {}", imported_count);
    } else {
        println!("\nℹ️ no keys have been added");
    }

    Ok(())
}