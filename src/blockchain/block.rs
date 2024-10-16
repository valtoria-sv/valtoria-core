use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub transactions: Vec<String>, // Simplified transactions as strings for now
}

impl Block {
    /// Creates a new block
    pub fn new(index: u64, previous_hash: String, transactions: Vec<String>) -> Self {
        let timestamp = current_timestamp();
        let mut block = Block {
            index,
            timestamp,
            previous_hash,
            hash: String::new(),
            nonce: 0,
            transactions,
        };
        block.hash = block.calculate_hash(); // Generate initial hash
        block
    }

    /// Calculates the hash of the block based on its contents
    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!(
            "{}{}{}{}{}",
            self.index,
            self.timestamp,
            self.previous_hash,
            self.nonce,
            self.transactions.join(",")
        );
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Increments the nonce and recalculates the hash
    pub fn increment_nonce(&mut self) {
        self.nonce += 1;
        self.hash = self.calculate_hash();
    }

    /// Returns the current timestamp in seconds
    pub fn timestamp() -> u64 {
        current_timestamp()
    }
}

/// Helper function to get the current timestamp in seconds
fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    duration.as_secs()
}
