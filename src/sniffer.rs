use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_transaction_status::{UiTransactionEncoding, UiInstruction, option_serializer::OptionSerializer, UiParsedInstruction, UiMessage};
use std::sync::Arc;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct TransactionTemplate {
    pub program_id: Pubkey,
    pub accounts: Vec<Pubkey>,
    pub data: Vec<u8>,
}

pub async fn analyze_transaction(client: Arc<RpcClient>, sig_str: &str) -> Option<TransactionTemplate> {
    let signature = Signature::from_str(sig_str.trim()).ok()?;

    let config = solana_client::rpc_config::RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::JsonParsed),
        commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    match client.get_transaction_with_config(&signature, config).await {
        Ok(tx_res) => {
            println!("✅ transaction received.");

            if let solana_transaction_status::EncodedTransaction::Json(ui_tx) = tx_res.transaction.transaction {
                
                match ui_tx.message {
                    UiMessage::Raw(raw_msg) => {
                        let account_keys = raw_msg.account_keys;
                        // Просто берем самую длинную инструкцию, это обычно Swap
                        let mut best_ix: Option<TransactionTemplate> = None;
                        let mut max_len = 0;

                        for (i, ix) in raw_msg.instructions.iter().enumerate() {
                            let data = bs58::decode(&ix.data).into_vec().unwrap_or_default();
                            let hex_str = hex::encode(&data);
                            
                            println!("📝 instruction #{}: HEX = {}", i, hex_str);

                            if data.len() > max_len {
                                max_len = data.len();
                                let p_id = Pubkey::from_str(&account_keys[ix.program_id_index as usize]).unwrap_or_default();
                                let accounts = ix.accounts.iter()
                                    .map(|&idx| Pubkey::from_str(&account_keys[idx as usize]).unwrap_or_default())
                                    .collect();
                                best_ix = Some(TransactionTemplate { program_id: p_id, accounts, data });
                            }
                        }
                        return best_ix;
                    },
                    UiMessage::Parsed(parsed_msg) => {
                        let mut best_ix: Option<TransactionTemplate> = None;
                        let mut max_len = 0;
                        
                        for (i, ix) in parsed_msg.instructions.iter().enumerate() {
                            if let UiInstruction::Parsed(UiParsedInstruction::PartiallyDecoded(decoded)) = ix {
                                let data = bs58::decode(&decoded.data).into_vec().unwrap_or_default();
                                let hex_str = hex::encode(&data);
                                
                                println!("📝 instruction #{} (Parsed): HEX = {}", i, hex_str);

                                if data.len() > max_len {
                                    max_len = data.len();
                                    let accounts = decoded.accounts.iter()
                                        .map(|k| Pubkey::from_str(k).unwrap_or_default())
                                        .collect();
                                    
                                    best_ix = Some(TransactionTemplate {
                                        program_id: Pubkey::from_str(&decoded.program_id).unwrap_or_default(),
                                        accounts,
                                        data,
                                    });
                                }
                            }
                        }
                        return best_ix;
                    }
                }
            }
            None
        },
        Err(e) => {
            println!("❌ rpc error: {}", e);
            None
        }
    }
}