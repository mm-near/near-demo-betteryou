use funding::FundingEngine;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::env::panic_str;
use near_sdk::serde::Serialize;
use near_sdk::{env, near_bindgen, AccountId, Promise};

// Full seconds since UNIX_EPOCH.
pub type Timestamp = u64;

// Seconds
pub type Duration = u64;

const DAY: Duration = 86400000000000;

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

    duolingo_username: String,
    language: String,
    quota_per_day: u32,
    total_xp: u32,
    day_start_xp: u32,
    current_daily_xp: u32,
    register_timestamp: Timestamp,
    days_passed: u64,
    funding: FundingEngine,
    day_status: Vec<bool>,
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
    #[private]
    pub fn reset(&mut self) {
        self.challenges.clear();
    }

    pub fn get_state_for_user(&self, account_id: AccountId) -> ChallengeState {
        self.challenges.get(&account_id).unwrap()
    }

    pub fn get_challenge(&self, account_id: AccountId) -> Option<ChallengeState> {
        self.challenges.get(&account_id)
    }

    pub fn get_all_state(&self) -> Vec<(AccountId, ChallengeState)> {
        self.challenges.to_vec()
    }

    #[payable]
    pub fn create_challenge(
        &mut self,
        language: String,
        duolingo_username: String,
        quota_per_day: u32,
        total_days: u32,
        total_lives: u32,
    ) {
        if let Some(_challenge) = self.challenges.get(&env::predecessor_account_id()) {
            env::panic_str("Challenge already present");
        }
        self.challenges.insert(
            &env::predecessor_account_id(),
            &ChallengeState {
                total_days,
                total_lives,
                days_left: total_days,
                lives_left: total_lives,
                quota_per_day,
                total_xp: 0,
                current_daily_xp: 0,
                day_start_xp: 0,
                duolingo_username,
                language,
                register_timestamp: env::block_timestamp(),
                days_passed: 0,
                funding: FundingEngine::new(
                    &env::predecessor_account_id(),
                    env::attached_deposit(),
                ),
                day_status: Vec::new(),
            },
        );
    }
    #[private]
    pub fn admin_update_challenge(&mut self, update: Vec<(AccountId, u32)>) {
        let now: Timestamp = env::block_timestamp();
        for i in 0..update.len() {
            // Check if it's a new day for all of the challenges based on their register timestamp
            let mut previous_val = self.challenges.get(&update[i].0).unwrap();
            let day_start = self
                .challenges
                .get(&update[i].0)
                .unwrap()
                .register_timestamp
                + self
                    .challenges
                    .get(&update[i].0)
                    .unwrap()
                    .days_passed
                    .checked_add(1)
                    .unwrap()
                    * DAY;
            if now > day_start {
                // Is the challenge under the quota for the day or not
                if self.challenges.get(&update[i].0).unwrap().quota_per_day
                    <= self.challenges.get(&update[i].0).unwrap().current_daily_xp
                {
                    self.admin_move_day(&update[i].0, previous_val.total_xp);
                } else {
                    self.admin_fail_day(&update[i].0, previous_val.total_xp);
                }
            }
            let incoming_value = &update[i].1;
            previous_val.total_xp = *incoming_value;
            previous_val.current_daily_xp = *incoming_value - previous_val.day_start_xp;
            self.challenges.insert(&update[i].0, &previous_val);
        }
    }

    pub fn finish(&mut self) {
        let challenge = self.get_challenge(env::predecessor_account_id()).unwrap();
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

    fn admin_move_day(&mut self, account_id: &AccountId, day_start_xp: u32) {
        let mut previous_val = self.challenges.get(account_id).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.days_passed = previous_val.days_passed.checked_add(1).unwrap();
        previous_val.current_daily_xp = 0;
        previous_val.day_start_xp = day_start_xp;
        previous_val.day_status.push(true);
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }
    fn admin_fail_day(&mut self, account_id: &AccountId, day_start_xp: u32) {
        let mut previous_val = self.challenges.get(account_id).unwrap();
        previous_val.days_left = previous_val.days_left.checked_sub(1).unwrap();
        previous_val.days_passed = previous_val.days_passed.checked_add(1).unwrap();
        previous_val.lives_left = previous_val.lives_left.checked_sub(1).unwrap();
        previous_val.current_daily_xp = 0;
        previous_val.day_start_xp = day_start_xp;
        previous_val.day_status.push(false);
        self.challenges
            .insert(&env::predecessor_account_id(), &previous_val);
    }

    #[payable]
    pub fn add_prize(&mut self, account_id: AccountId) {
        let mut challenge = self.challenges.get(&account_id).unwrap();
        challenge
            .funding
            .fund(&env::predecessor_account_id(), env::attached_deposit());
        self.challenges.insert(&account_id, &challenge);
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
