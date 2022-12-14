use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId};

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
    reward: u32,
    duolingo_username: String,
    language: String,
    quota_per_day: u32,
    total_xp: u32,
    current_daily_xp: u32,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct DuolingoAccount {
    duolingo_username: String,
    language: String,
    account_id: AccountId,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            challenges: UnorderedMap::new(b"m"),
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

    pub fn get_all_state(&self) -> Vec<(AccountId, ChallengeState)> {
        self.challenges.to_vec()
    }

    pub fn create_challenge(
        &mut self,
        language: String,
        duolingo_username: String,
        quota_per_day: u32,
    ) {
        if let Some(_challenge) = self.challenges.get(&env::predecessor_account_id()) {
            env::panic_str("Challenge already present");
        }
        self.challenges.insert(
            &env::predecessor_account_id(),
            &ChallengeState {
                total_days: 30,
                total_lives: 3,
                days_left: 30,
                lives_left: 3,
                reward: 1,
                quota_per_day,
                total_xp: 0,
                current_daily_xp: 0,
                duolingo_username,
                language,
            },
        );
    }

    pub fn admin_update_challenge(&mut self, update: Vec<(AccountId, u32)>) {
        for i in 0..update.len() {
            let mut previous_val = self.challenges.get(&update[i].0).unwrap();
            let incoming_value = &update[i].1;
            previous_val.current_daily_xp = previous_val
                .current_daily_xp
                .checked_add(*incoming_value)
                .unwrap();
            previous_val.total_xp = previous_val.total_xp.checked_add(*incoming_value).unwrap();
            self.challenges.insert(&update[i].0, &previous_val);
        }
    }

    pub fn cleanup_challenge(&mut self) {
        let challenge = self.get_challenge();
        if challenge.days_left == 0 || challenge.lives_left == 0 {
            self.challenges.remove(&env::predecessor_account_id());
        } else {
            panic_str("Challenge is not finished yet");
        }
    }

    pub fn admin_check_successful_day(&mut self) {
        let all_challenges = self.challenges.to_vec();
        for i in 0..all_challenges.len() {
            if all_challenges[i].1.quota_per_day <= all_challenges[i].1.current_daily_xp {
                self.admin_move_day();
            } else {
                self.admin_fail_day()
            }
        }
    }

    pub fn admin_move_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.current_daily_xp = 0;
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
    pub fn admin_fail_day(&mut self) {
        let mut previous_val = self.challenges.get(&env::predecessor_account_id()).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.lives_left = previous_val.lives_left.checked_sub(1).unwrap();
        previous_val.current_daily_xp = 0;
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
}