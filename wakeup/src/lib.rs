use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PublicKey};

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new_contract(owner_id: AccountId) -> Self {
        Self { owner_id }
    }
}
