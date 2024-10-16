use valtoria::wallet::{Transaction, Wallet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_creation_and_validation() {
        let wallet = Wallet::new("your_secure_password");

        // Create a transaction
        let transaction = wallet.create_transaction("recipient_address", 100);

        // Validate the transaction
        assert!(
            wallet.validate_transaction(&transaction),
            "Transaction should be valid"
        );
    }

    #[test]
    fn test_insufficient_funds() {
        let wallet = Wallet::new("your_secure_password");

        // Attempt to create a transaction with insufficient funds
        let transaction = wallet.create_transaction("recipient_address", 1000); // Assuming initial balance is less

        // Validate the transaction
        assert!(
            !wallet.validate_transaction(&transaction),
            "Transaction should be invalid due to insufficient funds"
        );
    }
}
