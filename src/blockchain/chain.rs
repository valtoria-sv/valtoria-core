use crate::blockchain::block::Block;
use crate::consensus::pow::ProofOfWork;
use crate::storage::RocksDbStore;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub store: RocksDbStore, // RocksDb instance for persistent storage
}

impl Blockchain {
    /// Initializes a new blockchain with a genesis block
    pub fn new(store: RocksDbStore) -> Self {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            store,
        };
        let genesis_block = Blockchain::create_genesis_block();
        blockchain.add_block(genesis_block);
        blockchain
    }

    /// Creates the genesis block (the first block in the chain)
    fn create_genesis_block() -> Block {
        Block::new(0, String::from("0"), vec![String::from("Genesis Block")])
    }

    /// Adds a block to the blockchain and persists it to RocksDb
    pub fn add_block(&mut self, block: Block) -> Result<(), String> {
        if let Some(prev_block) = self.blocks.last() {
            if block.previous_hash != prev_block.hash {
                return Err("Invalid previous hash".to_string());
            }
        }
        self.store_block(&block)?;
        self.blocks.push(block);
        Ok(())
    }

    /// Stores a block in RocksDb
    fn store_block(&self, block: &Block) -> Result<(), String> {
        self.store
            .put(
                block.index.to_string(),
                serde_json::to_string(block).unwrap(),
            )
            .map_err(|e| format!("Failed to store block: {}", e))
    }

    /// Retrieves the last `n` blocks for difficulty adjustment
    pub fn get_last_blocks(&self, n: usize) -> Vec<Block> {
        let len = self.blocks.len();
        if len < n {
            self.blocks.clone()
        } else {
            self.blocks[len - n..].to_vec()
        }
    }

    /// Retrieves the total supply by counting all transactions across blocks
    pub fn total_supply(&self) -> u64 {
        self.blocks
            .iter()
            .flat_map(|block| &block.transactions)
            .count() as u64
    }
}
