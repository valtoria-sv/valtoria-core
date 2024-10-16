// Network protocol implementation

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    NewTransaction(String),      // Transaction data
    NewBlock(String),            // Block data
    RequestBlocks,               // Request for blocks
    ResponseBlocks(Vec<String>), // Response with blocks
}

impl Message {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        bincode::deserialize(bytes).unwrap()
    }
}
