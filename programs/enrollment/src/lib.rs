mod programs;

use crate::programs::wba_prereq::{CompleteArgs, UpdateArgs, WbaPrereqProgram};
use bs58;
use solana_client::rpc_client::RpcClient;
use solana_program::system_instruction::transfer;
use solana_sdk::{
    message::Message,
    pubkey::Pubkey,
    signature::{read_keypair_file, Keypair, Signer},
    system_program,
    transaction::Transaction,
};
use std::io::{self, BufRead};
use std::str::FromStr;

const RPC_URL: &str = "https://api.devnet.solana.com";

#[cfg(test)]
mod tests {
    use solana_sdk;
    #[test]
    fn keygen() {}
    #[test]
    fn airdop() {}
    #[test]
    fn transfer_sol() {}
    #[test]
    fn transfer_all_to_wba() {}
    #[test]
    fn enroll_to_wba() {}
}

#[test]
fn keygen() {
    // Create a new keypair
    let kp = Keypair::new();
    println!(
        "You've generated a new Solana wallet: {}",
        kp.pubkey().to_string()
    );
    println!("");
    println!("To save your wallet, copy and paste the following into a JSON file:");
    println!("{:?}", kp.to_bytes());
}

#[test]
fn base58_to_wallet() {
    println!("Input your private key as base58:");
    let stdin = io::stdin();
    let base58 = stdin.lock().lines().next().unwrap().unwrap();
    println!("Your wallet file is:");
    let wallet = bs58::decode(base58).into_vec().unwrap();
    println!("{:?}", wallet);
}

#[test]
fn wallet_to_base58() {
    println!("Input your private key as a wallet file byte array:");
    let stdin = io::stdin();
    let wallet = stdin
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    println!("Your private key is:");
    let base58 = bs58::encode(wallet).into_string();
    println!("{:?}", base58);
}

#[test]
fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    let client = RpcClient::new(RPC_URL);
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(s) => {
            println!("Success! Check out your TX here:");
            println!(
                "https://explorer.solana.com/tx/{}?cluster=devnet",
                s.to_string()
            );
        }
        Err(e) => println!("Oops, something went wrong: {}", e.to_string()),
    };
    // First aidrop: https://explorer.solana.com/tx/52L7ds6pzrpg45hJ5gRZ5Am2x62Fb9DhLsamFhBEbWHxJe4F4jXPAho9NGT8V3Y5tkcBvTKQr7xGi9tGh7LoqvHt?cluster=devnet
}

#[test]
fn transfer_to_wba() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    // Define our WBA public key
    let to_pubkey = Pubkey::from_str("EbDgkRikJGQxcWWwFNDKQsEgwdGTopsx1yi3HKY9LKPq").unwrap();
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    // Transfer to WBA wallet: https://explorer.solana.com/tx/uJ5rYNsYZqpAXDioXBH1ve45ufWNwHgoBnPw85Wkzqa6QHp3qfpbyQhYtYSdn61muqsh8KxqvVuyjYtZBU52dhW/?cluster=devnet
}

#[test]
fn transfer_all_to_wba() {
    // Import our keypair
    let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
    // Define our WBA public key
    let to_pubkey = Pubkey::from_str("EbDgkRikJGQxcWWwFNDKQsEgwdGTopsx1yi3HKY9LKPq").unwrap();
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Get balance of dev wallet
    let balance = rpc_client
        .get_balance(&keypair.pubkey())
        .expect("Failed to get balance");

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Create a test transaction to calculate fees
    let message = Message::new_with_blockhash(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
        Some(&keypair.pubkey()),
        &recent_blockhash,
    );

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    let fee = rpc_client
        .get_fee_for_message(&message)
        .expect("Failed to get fee calculator");

    // Get recent blockhash
    let recent_blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Deduct fee from lamports amount and create a TX with correct balance
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash,
    );
    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    // Transfer all to WBA wallet: https://explorer.solana.com/tx/3TRzbjXd1MiRApZ6Zos1EZscgYPGqJJNrXqEYMGKpPAMoLG6mu6cCz7ZqmmNL9WpEEnY9aWutoNHmAn7sUmdcFTs/?cluster=devnet
}

#[test]
fn enroll_to_wba() {
    // Create a Solana devnet connection
    let rpc_client = RpcClient::new(RPC_URL);

    // Let's define our accounts
    let signer = read_keypair_file("wba-wallet.json").expect("Couldn't find wallet file");

    let prereq =
        WbaPrereqProgram::derive_program_address(&[b"prereq", signer.pubkey().to_bytes().as_ref()]);

    // Define our instruction data
    let args = CompleteArgs {
        github: b"chainsona".to_vec(),
    };

    // Get recent blockhash
    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get recent blockhash");

    // Now we can invoke the "complete" function
    let transaction = WbaPrereqProgram::complete(
        &[&signer.pubkey(), &prereq, &system_program::id()],
        &args,
        Some(&signer.pubkey()),
        &[&signer],
        blockhash,
    );
    // Send the transaction
    let signature = rpc_client
        .send_and_confirm_transaction(&transaction)
        .expect("Failed to send transaction");
    // Print our transaction out
    println!(
        "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
        signature
    );
    // Enroll to WBA: https://explorer.solana.com/tx/4fZJoKvjotqgKagVcst4H8ogR26WzXFDJdh5e8xqcHXtYPUg3yWvQt6p1UqaPEKmHs8xSNA8iaRqdrptV1FggruE?cluster=devnet
}
