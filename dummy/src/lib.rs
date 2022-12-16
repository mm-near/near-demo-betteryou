use funding::FundingEngine;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Promise};

use near_sdk::collections::UnorderedMap;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    challenges: UnorderedMap<AccountId, ChallengeState>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct ChallengeState {
    total_days: u32,
    total_lives: u32,
    days_left: u32,
    lives_left: u32,
    funding: FundingEngine,
    day_status: Vec<bool>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            challenges: UnorderedMap::new(b"challenges".to_vec()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[private]
    pub fn reset(&mut self) {
        self.challenges.clear();
    }

    pub fn get_challenge(&self, account_id: AccountId) -> Option<ChallengeState> {
        self.challenges.get(&account_id)
    }

    #[payable]
    pub fn create_challenge(&mut self) {
        if self
            .challenges
            .get(&env::predecessor_account_id())
            .is_some()
        {
            env::panic_str("Challenge already present");
        }
        self.challenges.insert(
            &env::predecessor_account_id(),
            &ChallengeState {
                total_days: 30,
                total_lives: 3,
                days_left: 30,
                lives_left: 3,
                funding: FundingEngine::new(
                    &env::predecessor_account_id(),
                    env::attached_deposit(),
                ),
                day_status: Vec::new(),
            },
        );
    }

    #[payable]
    pub fn add_prize(&mut self, account_id: AccountId) {
        let mut challenge = self.challenges.get(&account_id).unwrap();
        challenge
            .funding
            .fund(&env::predecessor_account_id(), env::attached_deposit());
        self.challenges.insert(&account_id, &challenge);
    }

    pub fn finish(&mut self) {
        let challenge = self.challenges.get(&env::predecessor_account_id()).unwrap();
        if challenge.days_left == 0 || challenge.lives_left == 0 {
            let prize = challenge.funding.resolve(challenge.days_left == 0);
            for (account_id, tokens) in prize.iter() {
                Promise::new(account_id.clone()).transfer(*tokens);
            }
            self.challenges.remove(&env::predecessor_account_id());
        } else {
            panic_str("Challenge is not finished yet");
        }
    }

    pub fn admin_move_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.day_status.push(true);
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
    pub fn admin_fail_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.lives_left = previous_val.lives_left.checked_sub(1).unwrap();
        previous_val.day_status.push(false);
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }

    pub fn abandon_challenge(&mut self) {
        let caller = env::predecessor_account_id();
        let challenge = self.challenges.get(&caller).unwrap();
        let prize = challenge.funding.resolve(false);
        for (account_id, tokens) in prize.iter() {
            Promise::new(account_id.clone()).transfer(*tokens);
        }
        self.challenges.remove(&caller);
    }
}
