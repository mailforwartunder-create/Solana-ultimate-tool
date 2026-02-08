use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signer::Signer,
    transaction::Transaction,
};
use crate::sniffer::TransactionTemplate;
use crate::wallets::WalletInfo;
use std::sync::Arc;
use std::str::FromStr;

pub async fn replay_transaction(
    client: Arc<RpcClient>,
    wallets: Vec<WalletInfo>,
    template: TransactionTemplate,
) {
    println!("🚀 starting (Replay Attack) on {} wallets(WIP).", wallets.len());

    // Получаем свежий blockhash (нужен для отправки транзакции)
    let latest_blockhash = match client.get_latest_blockhash().await {
        Ok(bh) => bh,
        Err(e) => {
            println!("❌ network error: {}", e);
            return;
        }
    };

    for wallet_info in wallets {
        // Восстанавливаем ключи кошелька
        let keypair = solana_sdk::signature::Keypair::from_bytes(&wallet_info.private_key).unwrap();
        let my_pubkey = keypair.pubkey();

        println!("⚡ wallet processing: {}", my_pubkey);

        // --- МАГИЯ ПОДМЕНЫ ---
        // Мы берем список аккаунтов из чужой транзакции
        // И заменяем первый аккаунт (обычно это signer/плательщик) на НАШ кошелек
        let mut account_metas = Vec::new();
        for (i, acc) in template.accounts.iter().enumerate() {
            if i == 0 {
                // Подставляем СЕБЯ вместо жертвы
                account_metas.push(AccountMeta::new(my_pubkey, true)); 
            } else {
                // Остальные аккаунты (пулы, токены системы) оставляем как есть
                // Важно: тут нужно знать, какие writeable, но для теста ставим false/true эвристически
                // Для production нужно парсить точнее. Пока ставим new(writeable) для всех, кроме системных.
                account_metas.push(AccountMeta::new(*acc, false)); 
            }
        }

        // Собираем инструкцию
        let instruction = Instruction {
            program_id: template.program_id,
            accounts: account_metas,
            data: template.data.clone(),
        };

        // Собираем транзакцию
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&my_pubkey),
            &[&keypair],
            latest_blockhash,
        );

        // Отправляем
        match client.send_and_confirm_transaction(&tx).await {
            Ok(sig) => println!("✅ success! Hash: {}", sig),
            Err(e) => println!("❌ sending error: {}", e),
        }
        
        // Пауза, чтобы не спамить
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }
}