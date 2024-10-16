use crate::blockchain::{Block, Blockchain};
use crate::consensus::pow::ProofOfWork; // Update import

pub struct Miner {
    pub blockchain: Blockchain,
    pub pow: ProofOfWork,
}

impl Miner {
    pub fn new(blockchain: Blockchain, difficulty: usize) -> Self {
        let pow = ProofOfWork::new(difficulty);
        Miner { blockchain, pow }
    }

    /// Mines a new block and adds it to the blockchain.
    pub fn mine(&mut self, data: String) -> Result<Block, String> {
        let (hash, nonce) = self.pow.mine_block(&data);

        let new_block = Block::new(
            self.blockchain.get_latest_block().index + 1,
            hash,
            data,
            self.blockchain.get_latest_block().hash.clone(),
            nonce,
        );

        self.blockchain
            .add_block(new_block.clone())
            .map_err(|e| e.to_string())?;

        Ok(new_block)
    }
}
