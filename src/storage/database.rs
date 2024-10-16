use crate::blockchain::block::Block;
use rocksdb::{Error, Options, WriteBatch, DB};
use std::sync::{Arc, Mutex};

/// A wrapper around RocksDB for high-performance key-value storage with caching and batch processing.
#[derive(Debug, Clone)]
pub struct RocksDbStore {
    db: Arc<Mutex<DB>>,
}

impl RocksDbStore {
    /// Creates a new instance of RocksDbStore and opens the database at the specified path.
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path).expect("Failed to open RocksDB");

        RocksDbStore {
            db: Arc::new(Mutex::new(db)),
        }
    }

    // Transaction-related methods omitted for brevity...

    /// Store a block in RocksDB using the block hash or height as the key.
    pub fn store_block(&self, block: &Block) -> Result<(), String> {
        let block_key = format!("block_{}", block.hash);
        let serialized_block =
            bincode::serialize(&block).map_err(|_| "Serialization Error".to_string())?;
        self.store_key(&block_key, &serialized_block)
            .map_err(|_| "Failed to store block".to_string())
    }

    /// Retrieve a block from RocksDB using the block hash or height.
    pub fn get_block(&self, block_hash: &str) -> Option<Block> {
        let db_key = format!("block_{}", block_hash);
        match self.get_key(&db_key) {
            Some(serialized_block) => bincode::deserialize(&serialized_block).ok(),
            None => None,
        }
    }

    /// Batch store multiple blocks in RocksDB for efficient processing.
    pub fn batch_store_blocks(&self, blocks: Vec<&Block>) -> Result<(), Error> {
        let mut batch = WriteBatch::default();
        let db = self.db.lock().unwrap();

        for block in blocks {
            let block_key = format!("block_{}", block.hash);
            let serialized_block = bincode::serialize(block).expect("Failed to serialize block");
            batch.put(block_key.as_bytes(), &serialized_block);
        }

        db.write(batch)
    }

    /// Batch retrieve multiple blocks from RocksDB using a vector of block hashes.
    pub fn get_batch_blocks(&self, block_hashes: Vec<&str>) -> Vec<Option<Block>> {
        let db = self.db.lock().unwrap();
        block_hashes
            .iter()
            .map(|block_hash| {
                let db_key = format!("block_{}", block_hash);
                db.get(db_key.as_bytes())
                    .ok()
                    .flatten()
                    .and_then(|serialized_block| bincode::deserialize(&serialized_block).ok())
            })
            .collect()
    }
}
