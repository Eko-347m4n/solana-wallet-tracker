use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::str::FromStr;
use solana_sdk::commitment_config::CommitmentConfig;
use spl_token::ID as TOKEN_PROGRAM_ID;
use solana_account_decoder::UiAccountEncoding;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use spl_token::state::Account as TokenAccount;
use bincode::deserialize;

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

///Task 2
//Fungsi ambil token account
fn get_spl_token_accounts(pubkey: &Pubkey, client: &RpcClient) -> Result<(), Box<dyn Error>> {
    let token_accounts = client.get_token_accounts_by_owner(
        pubkey,
        solana_client::rpc_client::TokenAccountsFilter::ProgramId(TOKEN_PROGRAM_ID),
    )?;

    println!("\nSPL Tokens owned by {}:", pubkey);
    for account in token_accounts.value {
        println!("- Token Account: {}", account.pubkey);
    }

    Ok(())
}

//Decode info token
for account in token_accounts.value {
    let data = base64::decode(account.account.data[0].clone())?; // [data, encoding]
    let token_account: TokenAccount = deserialize(&data)?;

    let mint = token_account.mint;
    let amount = token_account.amount;

    println!(
        "- Token Account: {}\n  Mint: {}\n  Balance (raw): {}\n",
        account.pubkey, mint, amount
    );
}