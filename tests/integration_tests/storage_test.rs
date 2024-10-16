use tempfile::tempdir;
use valtoria::storage::{BlockData, Database, TransactionData};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_storage() {
        // Create a temporary directory for the database
        let dir = tempdir().expect("Failed to create temp dir");
        let db = Database::open(dir.path()).expect("Failed to open database");

        // Create and store a block
        let block_data = BlockData {
            index: 1,
            previous_hash: "genesis_hash".to_string(),
            transactions: vec![],
            nonce: 0,
            hash: "block_hash".to_string(),
        };

        // Store the block
        db.put_block(&block_data).expect("Failed to store block");

        // Retrieve the block
        let retrieved_block = db
            .get_block(block_data.index)
            .expect("Failed to retrieve block");

        // Validate that the retrieved block matches the original
        assert_eq!(retrieved_block.index, block_data.index);
        assert_eq!(retrieved_block.previous_hash, block_data.previous_hash);
        assert_eq!(retrieved_block.hash, block_data.hash);
    }

    #[test]
    fn test_transaction_storage() {
        // Create a temporary directory for the database
        let dir = tempdir().expect("Failed to create temp dir");
        let db = Database::open(dir.path()).expect("Failed to open database");

        // Create and store a transaction
        let transaction_data = TransactionData {
            id: 1,
            from: "sender_address".to_string(),
            to: "recipient_address".to_string(),
            amount: 50,
            timestamp: 1234567890,
        };

        // Store the transaction
        db.put_transaction(&transaction_data)
            .expect("Failed to store transaction");

        // Retrieve the transaction
        let retrieved_transaction = db
            .get_transaction(transaction_data.id)
            .expect("Failed to retrieve transaction");

        // Validate that the retrieved transaction matches the original
        assert_eq!(retrieved_transaction.id, transaction_data.id);
        assert_eq!(retrieved_transaction.from, transaction_data.from);
        assert_eq!(retrieved_transaction.to, transaction_data.to);
        assert_eq!(retrieved_transaction.amount, transaction_data.amount);
    }

    #[test]
    fn test_batch_storage() {
        // Create a temporary directory for the database
        let dir = tempdir().expect("Failed to create temp dir");
        let db = Database::open(dir.path()).expect("Failed to open database");

        // Create a batch of blocks
        let mut blocks = vec![];
        for i in 0..5 {
            let block_data = BlockData {
                index: i + 1,
                previous_hash: "genesis_hash".to_string(),
                transactions: vec![],
                nonce: 0,
                hash: format!("block_hash_{}", i + 1),
            };
            blocks.push(block_data);
        }

        // Store the batch of blocks
        db.put_blocks_batch(&blocks)
            .expect("Failed to store batch of blocks");

        // Retrieve and validate each block in the batch
        for block in blocks {
            let retrieved_block = db.get_block(block.index).expect("Failed to retrieve block");
            assert_eq!(retrieved_block.index, block.index);
            assert_eq!(retrieved_block.hash, block.hash);
        }
    }
}
