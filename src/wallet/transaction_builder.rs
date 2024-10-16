use crate::wallet::account::Wallet;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub id: String,
    pub sender: String,
    pub receiver: String,
    pub amount: u64,
}

pub struct TransactionBuilder<'a> {
    wallet: &'a mut Wallet,
}

impl<'a> TransactionBuilder<'a> {
    pub fn new(wallet: &'a mut Wallet) -> Self {
        TransactionBuilder { wallet }
    }

    pub fn create_transaction(
        &mut self,
        sender: &str,
        receiver: &str,
        amount: u64,
    ) -> Result<Transaction, String> {
        // Validate sender's balance
        let sender_balance = self.wallet.get_balance(sender)?;
        if sender_balance < amount {
            return Err(format!(
                "Insufficient balance in sender's account: {}",
                sender
            ));
        }

        // Create the transaction
        let transaction = Transaction {
            id: format!("tx_{}", uuid::Uuid::new_v4()),
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        };

        // Update balances
        self.wallet.update_balance(sender, -(amount as i64))?;
        self.wallet.update_balance(receiver, amount as i64)?;

        Ok(transaction)
    }
}
