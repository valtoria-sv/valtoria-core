use valtoria::blockchain::{Block, Blockchain, Transaction};
use valtoria::wallet::Wallet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation_and_chain_validation() {
        let mut blockchain = Blockchain::new();

        // Create a wallet to generate transactions
        let wallet = Wallet::new("your_secure_password");

        // Create a transaction
        let transaction = wallet.create_transaction("recipient_address", 50);

        // Add transaction to a new block
        let block = Block::new(
            1,
            vec![transaction.clone()],
            blockchain.get_latest_block_hash(),
        );
        blockchain.add_block(block).expect("Failed to add block");

        // Validate the blockchain
        assert!(
            blockchain.is_chain_valid(),
            "Blockchain should be valid after adding a block"
        );
    }

    #[test]
    fn test_multiple_blocks_and_transaction_processing() {
        let mut blockchain = Blockchain::new();
        let wallet = Wallet::new("your_secure_password");

        // Create multiple transactions and blocks
        for i in 0..5 {
            let transaction = wallet.create_transaction(format!("recipient_{}", i), 10 * (i + 1));
            let block = Block::new(
                i as u32 + 1,
                vec![transaction.clone()],
                blockchain.get_latest_block_hash(),
            );
            blockchain.add_block(block).expect("Failed to add block");
        }

        assert!(
            blockchain.is_chain_valid(),
            "Blockchain should be valid after adding multiple blocks"
        );
    }
}
