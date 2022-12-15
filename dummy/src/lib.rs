use funding::FundingEngine;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId};

use near_sdk::collections::LookupMap;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    challenges: LookupMap<AccountId, ChallengeState>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct ChallengeState {
    total_days: u32,
    total_lives: u32,
    days_left: u32,
    lives_left: u32,
    funding: FundingEngine,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            challenges: LookupMap::new(b"challenges".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_state_for_user(&self, account_id: AccountId) -> ChallengeState {
        self.challenges.get(&account_id).unwrap()
    }

    pub fn get_challenge(&self) -> ChallengeState {
        self.challenges.get(&env::predecessor_account_id()).unwrap()
    }

    #[payable]
    pub fn create_challenge(&mut self) {
        if self.challenges.contains_key(&env::predecessor_account_id()) {
            env::panic_str("Challenge already present");
        }
        self.challenges.insert(
            &env::predecessor_account_id(),
            &ChallengeState {
                total_days: 30,
                total_lives: 3,
                days_left: 30,
                lives_left: 3,
                funding: FundingEngine::new(env::predecessor_account_id(), env::attached_deposit()),
            },
        );
    }

    #[payable]
    pub fn add_price(&mut self, account_id: AccountId) {
        let mut challenge = self.challenges.get(&account_id).unwrap();
        challenge
            .funding
            .fund(env::predecessor_account_id(), env::attached_deposit());
        self.challenges.insert(&account_id, &challenge);
    }

    pub fn cleanup_challenge(&mut self) {
        let challenge = self.get_challenge();
        if challenge.days_left == 0 || challenge.lives_left == 0 {
            self.challenges.remove(&env::predecessor_account_id());
        } else {
            panic_str("Challenge is not finished yet");
        }
    }

    pub fn admin_move_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
    pub fn admin_fail_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.lives_left = previous_val.lives_left.checked_sub(1).unwrap();
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
}
