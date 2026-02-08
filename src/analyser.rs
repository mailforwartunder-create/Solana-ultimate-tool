use std::convert::TryInto;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct DiffResult {
    pub offset: usize,
    pub length: usize,
    pub data_old: Vec<u8>,
    pub data_new: Vec<u8>,
}

pub struct TransactionAnalyser {
    pub last_seen_data: Option<Vec<u8>>,
}

impl TransactionAnalyser {
    pub fn new() -> Self {
        Self { last_seen_data: None }
    }

    // Твоя функция сравнения
    pub fn compare(&mut self, new_data: &[u8]) -> Vec<DiffResult> {
        let mut results = Vec::new();

        if let Some(old_data) = &self.last_seen_data {
            let min_len = old_data.len().min(new_data.len());
            let mut i = 0;

            while i < min_len {
                if old_data[i] != new_data[i] {
                    let start = i;
                    while i < min_len && old_data[i] != new_data[i] {
                        i += 1;
                    }
                    results.push(DiffResult {
                        offset: start,
                        length: i - start,
                        data_old: old_data[start..i].to_vec(),
                        data_new: new_data[start..i].to_vec(),
                    });
                } else {
                    i += 1;
                }
            }
        }

        self.last_seen_data = Some(new_data.to_vec());
        results
    }

    pub fn report_diffs(&self, diffs: &[DiffResult]) {
        if diffs.is_empty() {
            println!("✅ Структура идентична.");
            return;
        }

        println!("\n🔎 ОБНАРУЖЕНЫ ИЗМЕНЕНИЯ В БАЙТАХ:");
        for diff in diffs {
            print!("📍 Смещение: {:0>2} | Длина: {} байт", diff.offset, diff.length);
            
            if diff.length == 8 {
                let val_old = u64::from_le_bytes(diff.data_old.as_slice().try_into().unwrap_or([0; 8]));
                let val_new = u64::from_le_bytes(diff.data_new.as_slice().try_into().unwrap_or([0; 8]));
                print!(" | 💰 СУММА: {} -> {}", val_old, val_new);
            } 
            else if diff.length == 32 {
                print!(" | 🔑 Публичный ключ");
            }
            println!();
        }
    }

    // Твоя НОВАЯ функция для метода 3-х транзакций
pub fn find_amount_offset(&mut self, tx1: &[u8], tx2: &[u8], tx3: &[u8]) -> Option<usize> {
    // Находим минимальную общую длину, чтобы не было паники
    let min_len = tx1.len().min(tx2.len()).min(tx3.len());
    
    // 1. Фиксируем шум (сравниваем первые две транзы 0.25 и 0.25)
    let mut noise_indices = HashSet::new();
    for i in 0..min_len {
        if tx1[i] != tx2[i] {
            noise_indices.insert(i);
        }
    }

    // 2. Ищем сумму (сравниваем вторую 0.25 и третью 0.26)
    // Идем по байтам и ищем последовательность из 8 байт, которая НЕ шум
    let mut i = 0;
    while i <= min_len - 8 {
        // Проверяем, есть ли тут изменения
        let mut changed = false;
        let mut has_noise = false;

        for j in 0..8 {
            if tx2[i + j] != tx3[i + j] {
                changed = true;
            }
            if noise_indices.contains(&(i + j)) {
                has_noise = true;
            }
        }

        // Если байты изменились и это НЕ шум
        if changed && !has_noise {
            let val_old = u64::from_le_bytes(tx2[i..i+8].try_into().unwrap_or([0; 8]));
            let val_new = u64::from_le_bytes(tx3[i..i+8].try_into().unwrap_or([0; 8]));

            // Если это похоже на наши 0.25 -> 0.26 SOL (в лампортах это большая разница)
            if val_old != val_new {
                return Some(i);
            }
        }
        i += 1;
    }

    None
}
}
