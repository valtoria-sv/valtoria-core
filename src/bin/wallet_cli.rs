fn main() {
    println!("Monero-like wallet CLI starting...");
}

use crate::wallet::Wallet;
use log::{error, info};
use std::env; // Assuming you have a Wallet struct in your wallet module

fn main() {
    // Initialize logging
    env_logger::init();

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <command> [args]", args[0]);
        eprintln!("Commands: send <amount> <address>, balance");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "send" => {
            if args.len() != 4 {
                eprintln!("Usage: {} send <amount> <address>", args[0]);
                return;
            }

            let amount: u64 = args[2].parse().expect("Invalid amount");
            let address = &args[3];

            let wallet = Wallet::new(); // Create a new wallet instance
            match wallet.send_transaction(amount, address) {
                Ok(_) => info!("Successfully sent {} to {}", amount, address),
                Err(e) => error!("Failed to send transaction: {}", e),
            }
        }
        "balance" => {
            let wallet = Wallet::new(); // Create a new wallet instance
            match wallet.get_balance() {
                Ok(balance) => info!("Your balance is: {}", balance),
                Err(e) => error!("Failed to retrieve balance: {}", e),
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
