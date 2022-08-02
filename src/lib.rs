/*
 * Example smart contract written in RUST
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://near-docs.io/develop/Contract
 *
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, AccountId, env, PublicKey, Gas};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{}
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    /**
     * Getting Account ids
     */
    pub fn get_wallet(&self) -> (AccountId, AccountId, AccountId) {
        // Getting signer address
        let signer = env::signer_account_id();
        let predecessor = env::predecessor_account_id();

        // Getting smart contract wallet id
        let current = env::current_account_id();

        (signer, predecessor, current)
    }

    /**
     * Getting signer's public key
     */
    pub fn get_pk(&self) -> PublicKey {
        // Code goes here
        let public_key: PublicKey = env::signer_account_pk();
        public_key
    }

    /**
     * Getting gas attached to a transaction call
     */
    pub fn get_gas(&self) -> (Gas, Gas) {

        // Attached gas
        let attached_gas: Gas = env::prepaid_gas();

        // Used gas
        let used_gas: Gas = env::used_gas();
        
        (attached_gas, used_gas)
    }

    /**
     * Getting attached NEAR tokens to a transaction
     */
    pub fn get_deposit(&self) -> u128 {
        // code goes here
        let deposit = env::attached_deposit();
        deposit
    }

    /**
     * Getting storage cost per byte
     */
    pub fn get_storage_info(&self) -> (u128, u64) {

        // Storage cost
        let storage_cost: u128 = env::storage_byte_cost();

        // Storage used
        let storage_used: u64 = env::storage_usage();

        (storage_cost, storage_used)
    }

    /**
     * Getting smart contract account balance
     */
    pub fn get_balance(&self) -> (u128, u128) {

        // Get balance
        let balance: u128 = env::account_balance();

        // Get locked balance
        let locked_balance = env::account_locked_balance();

        (balance, locked_balance)
    }

    /**
     * Getting block timestamp / block height
     */
    pub fn get_block_id(&self) -> (u64, u64) {

        // Get block height
        let block_height: u64 = env::block_height();

        // get block timestamp
        let block_time = env::block_timestamp();

        (block_height, block_time)
    }

    /**
     * Check if account id is valid
     */
    pub fn is_valid_account(&self, account_id: String) -> bool {
        // Code goes here
        let is_valid: bool = env::is_valid_account_id(account_id.as_bytes());
        is_valid
    }
}
