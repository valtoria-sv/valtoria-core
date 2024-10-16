use crate::crypto::stealth_address::StealthAddress;
use crate::storage::database::RocksDbStore;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub id: String,
    pub balance: u64,
    pub stealth_addresses: Vec<StealthAddress>, // List of stealth addresses
}

pub struct Wallet {
    pub accounts: HashMap<String, Account>,
    pub store: RocksDbStore,
}

impl Wallet {
    pub fn new(store: RocksDbStore) -> Self {
        Wallet {
            accounts: HashMap::new(),
            store,
        }
    }

    pub fn create_account(&mut self, id: &str) -> Result<(), String> {
        if self.accounts.contains_key(id) {
            return Err(format!("Account with id {} already exists.", id));
        }

        let account = Account {
            id: id.to_string(),
            balance: 0,
            stealth_addresses: Vec::new(),
        };

        self.accounts.insert(id.to_string(), account);
        self.store_account(&account)?;

        Ok(())
    }

    pub fn create_stealth_address(&mut self, account_id: &str) -> Result<StealthAddress, String> {
        let account = self
            .accounts
            .get_mut(account_id)
            .ok_or_else(|| format!("Account with id {} not found.", account_id))?;

        let stealth_address = StealthAddress::new();
        account.stealth_addresses.push(stealth_address.clone());
        self.store_account(account)?;

        Ok(stealth_address)
    }

    pub fn get_balance(&self, id: &str) -> Result<u64, String> {
        self.accounts
            .get(id)
            .map(|account| account.balance)
            .ok_or_else(|| format!("Account with id {} not found.", id))
    }

    pub fn update_balance(&mut self, id: &str, amount: i64) -> Result<(), String> {
        let account = self
            .accounts
            .get_mut(id)
            .ok_or_else(|| format!("Account with id {} not found.", id))?;

        if amount < 0 && account.balance < amount.abs() as u64 {
            return Err(format!("Insufficient balance for account: {}", id));
        }

        account.balance = (account.balance as i64 + amount) as u64;
        self.store_account(account)?;

        Ok(())
    }

    fn store_account(&self, account: &Account) -> Result<(), String> {
        let key = format!("account_{}", account.id);
        let value = serde_json::to_string(account).map_err(|e| e.to_string())?;
        self.store.put(key, value).map_err(|e| e.to_string())
    }

    pub fn load_account(&mut self, id: &str) -> Result<(), String> {
        let key = format!("account_{}", id);
        if let Some(value) = self.store.get(key).map_err(|e| e.to_string())? {
            let account: Account = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
            self.accounts.insert(id.to_string(), account);
        }
        Ok(())
    }
}
