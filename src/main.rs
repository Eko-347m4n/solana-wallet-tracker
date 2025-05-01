use solana_client::{rpc_client::RpcClient, rpc_request::TokenAccountsFilter};
use solana_sdk::pubkey::Pubkey;
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::str::FromStr;
use spl_token::state::Account as TokenAccount;
use base64::engine::general_purpose::STANDARD as BASE64_STANDARD;
use base64::Engine;
use solana_account_decoder::UiAccountData;
use solana_sdk::program_pack::Pack;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <wallet_address>");
        std::process::exit(1);
    }

    let rpc_url = env::var("RPC_URL")?;
    let client = RpcClient::new(rpc_url.clone());
    let pubkey = Pubkey::from_str(&args[1])?;

    let sol_balance = get_sol_balance(&rpc_url, &args[1])?;
    println!("Wallet {} has {} SOL", pubkey, sol_balance);

    get_spl_token_accounts(&pubkey, &client)?;

    Ok(())
}

fn get_sol_balance(rpc_url: &str, pubkey_str: &str) -> Result<f64, Box<dyn Error>> {
    let client = RpcClient::new(rpc_url.to_string());
    let pubkey = Pubkey::from_str(pubkey_str)?;
    let lamports = client.get_balance(&pubkey)?;
    Ok(lamports as f64 / 1_000_000_000.0)
}

fn get_spl_token_accounts(pubkey: &Pubkey, client: &RpcClient) -> Result<(), Box<dyn Error>> {
    let token_accounts = client.get_token_accounts_by_owner(
        pubkey,
        TokenAccountsFilter::ProgramId(spl_token::ID),
    )?;

    println!("\nSPL Tokens owned by {}:", pubkey);

    for account in token_accounts {
        match &account.account.data {
            UiAccountData::Binary(data, _) => {
                let decoded = BASE64_STANDARD.decode(data)?;
                let token_account = TokenAccount::unpack_from_slice(&decoded)?;

                println!(
                    "- Token Account: {}\n  Mint: {}\n  Balance (raw): {}\n",
                    account.pubkey, token_account.mint, token_account.amount
                );
            }
            _ => {
                println!("- Skipped non-binary account: {}", account.pubkey);
            }
        }
    }

    Ok(())
}