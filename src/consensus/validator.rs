use crate::blockchain::block::Block;
use crate::crypto::keys::PublicKey;
use crate::crypto::ring_signature::RingSignature;
use crate::storage::database::Database;

pub struct Validator;

impl Validator {
    /// Validate a block's transactions by verifying ring signatures
    pub fn validate_block_signatures(block: &Block, db: &Database) -> Result<bool, String> {
        for transaction in block.transactions() {
            let ring_signature = transaction.ring_signature();
            let public_keys: Vec<PublicKey> = transaction
                .inputs()
                .iter()
                .map(|input| input.public_key())
                .collect();

            if !RingSignature::verify(&ring_signature, &public_keys) {
                return Err("Invalid transaction signature".to_string());
            }
        }
        Ok(true)
    }

    /// Validate transaction uniqueness, prevent double-spending
    pub fn validate_transaction_uniqueness(block: &Block, db: &Database) -> Result<bool, String> {
        for transaction in block.transactions() {
            if db.transaction_exists(&transaction.id()) {
                return Err("Double spending detected".to_string());
            }
        }
        Ok(true)
    }
}
