use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::{io::{self, Write}, sync::Arc, str::FromStr};

mod wallets;
mod crypto;
mod sniffer;
mod executor;
mod analyser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rpc_url = "https://api.mainnet-beta.solana.com".to_string();
    let client = Arc::new(RpcClient::new(rpc_url));

    println!(r#"
    
     █████╗ ██████╗ ███████╗███████╗
    ██╔══██╗██╔══██╗██╔════╝██╔════╝
    ███████║██████╔╝█████╗  ███████╗
    ██╔══██║██╔══██╗██╔══╝  ╚════██║
    ██║  ██║██║  ██║███████╗███████║
    ╚═╝  ╚═╝╚═╝  ╚═╝╚══════╝╚══════╝ v2.0 - ULTIMATE
    "#);

    print!("🔑 Enter Master Password: ");
    io::stdout().flush()?;
    let mut password_input = String::new();
    io::stdin().read_line(&mut password_input)?;
    let password = Arc::new(password_input.trim().to_string());

    loop {
        println!("\n--- 🛠 FARM MANAGEMENT PANEL ---");
        println!("1. Generate New Wallets");
        println!("2. Check Balances & Addresses");
        println!("3. Clone Attack (Replay Engine)");
        println!("4. Advanced Diff-Analysis");
        println!("5. Premium Features");
        println!("6. Import from wallets.txt");
        println!("0. Exit");
        print!("\nSelection: ");
        io::stdout().flush()?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "1" => {
                print!("Amount of wallets to generate: ");
                io::stdout().flush()?;
                let mut c = String::new();
                io::stdin().read_line(&mut c)?;
                let count: usize = c.trim().parse().unwrap_or(0);
                if count > 0 {
                    wallets::generate_batch(count, &*password);
                }
            }
            "2" => {
                println!("\n⏳ Scanning Solana Blockchain...");
                match wallets::load_from_file("wallets.enc", &*password) {
                    Ok(list) => {
                        println!("📋 Wallets Loaded: {}", list.len());
                        println!("-------------------------------------------------------------");
                        let mut total_balance = 0.0;
                        for (i, w) in list.iter().enumerate() {
                            if let Ok(pk) = Pubkey::from_str(&w.address) {
                                let lamports = client.get_balance(&pk).await.unwrap_or(0);
                                let sol = lamports as f64 / 1_000_000_000.0;
                                total_balance += sol;
                                println!("[#{:02}] {} | 💎 {:.4} SOL", i + 1, w.address, sol);
                            }
                        }
                        println!("-------------------------------------------------------------");
                        println!("💰 TOTAL FARM BALANCE: {:.4} SOL", total_balance);
                    }
                    Err(_) => println!("❌ Error: Invalid password or wallets.enc not found."),
                }
            }
            "3" => {
                println!("\n🕵️‍♂️ REPLICATION MODE (Transaction Replay Engine)");
                print!("🔗 Insert Success TX Hash: ");
                io::stdout().flush()?;
                let mut hash = String::new();
                io::stdin().read_line(&mut hash)?;
                
                let hash_trim = hash.trim();
                if hash_trim.is_empty() { continue; }
                println!("🔍 Analyzing structure...");
                
                if let Some(template) = sniffer::analyze_transaction(client.clone(), hash_trim).await {
                    println!("\n✅ TEMPLATE READY FOR REPLICATION");
                    println!("🎯 Target Program: {}", template.program_id);
                    println!("📄 Data Payload: {} bytes", template.data.len());
                    
                    print!("🚀 Execute on ALL wallets? (y/n): ");
                    io::stdout().flush()?;
                    let mut confirm = String::new();
                    io::stdin().read_line(&mut confirm)?;
                    
                    if confirm.trim().eq_ignore_ascii_case("y") {
                        println!("⏳ Connecting to Jito Block Engine...");
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        println!("❌ Error: Insufficient permissions for Bundle transaction.");
                        println!("📢 Replay mode is restricted to PREMIUM license holders.");
                    }
                } else {
                    println!("❌ Failed to parse transaction structure (V0/ALT Error).");
                }
            }
            "4" => {
                println!("\n🕵️‍♂️ SELECT MODE:");
                println!("a. Hex calculation & Replication");
                println!("b. Diff-analysis (V0/ALT Support)");
                print!("> ");
                io::stdout().flush()?;
                
                let mut sub = String::new();
                io::stdin().read_line(&mut sub)?;

                if sub.trim() == "b" {
                    println!("\n⚠️  MODULE IN DEVELOPMENT");
                    println!("🔬 Diff-analysis for V0 transactions is in private beta.");
                } else {
                    print!("🔗 Insert TX Hash: ");
                    io::stdout().flush()?;
                    let mut hash = String::new();
                    io::stdin().read_line(&mut hash)?;
                    if let Some(template) = sniffer::analyze_transaction(client.clone(), hash.trim()).await {
                        if let Ok(my_wallets) = wallets::load_from_file("wallets.enc", &*password) {
                            executor::replay_transaction(client.clone(), my_wallets, template).await;
                        }
                    }
                } 
            }
            "5" => {
                println!("\n✨ [ PREMIUM FUNCTIONS - ACCESS DENIED ] ✨");
                println!("--------------------------------------------------");
                println!("🚀 Volume Booster      | Status: [LOCKED] 🔒");
                println!("🛡 Anti-MEV (Jito)      | Status: [LOCKED] 🔒");
                println!("🎯 Liquidity Sniper    | Status: [LOCKED] 🔒");
                println!("⚡️ Lightning Executor   | Status: [LOCKED] 🔒");
                println!("--------------------------------------------------");
                println!("📩 Contact developer for full version: @lflfjjfj"); 
            }
            "6" => {
                println!("📥 Importing from wallets.txt...");
                // Используем уже введенный мастер-пароль
                if let Err(e) = wallets::import_from_txt("wallets.txt", &*password) {
                    println!("❌ Import Error: {}", e);
                    println!("💡 Make sure 'wallets.txt' is in the root folder.");
                }
            }
            "0" => {
                println!("👋 See you soon, Millionaire.");
                break;
            }
            _ => println!("⚠️ Invalid choice."),
        }
    }
    Ok(())
}