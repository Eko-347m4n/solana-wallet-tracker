use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use dotenv::dotenv;
use std::env;
use std::error::Error;

/// Fungsi utama: jalankan CLI
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Baca file .env

    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <WALLET_ADDRESS>");
        std::process::exit(1);
    }

    let wallet_address = &args[1];
    let sol = get_sol_balance(&rpc_url, wallet_address)?;
    println!("Wallet {} has {:.9} SOL", wallet_address, sol);

    Ok(())
}

/// Fungsi untuk ambil saldo dan konversi dari lamports ke SOL
fn get_sol_balance(rpc_url: &str, pubkey_str: &str) -> Result<f64, Box<dyn Error>> {
    let client = RpcClient::new(rpc_url.to_string());
    let pubkey = Pubkey::from_str(pubkey_str)?;
    let lamports = client.get_balance(&pubkey)?;

    // 1 SOL = 1_000_000_000 lamports
    Ok(lamports as f64 / 1_000_000_000.0)
}
