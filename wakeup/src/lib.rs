use funding::FundingEngine;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::unordered_map::UnorderedMap;
use near_sdk::env::panic_str;
use near_sdk::serde;
use near_sdk::{env, log, near_bindgen, AccountId, Promise};
use std::time::Duration;

// Full seconds since UNIX_EPOCH.
pub type TimestampSeconds = u64;

// Seconds
pub type DurationSeconds = u64;

const DAY: DurationSeconds = 24 * 60 * 60;
const MAX_DAYS: u64 = 1000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    challenges: UnorderedMap<AccountId, ChallengeState>,
    // TESTONLY: set custom day length.
    day_length: Option<DurationSeconds>,
}

#[derive(serde::Serialize, serde::Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChallengeConfig {
    lives: u64,
    days: u64,
    first_day: TimestampSeconds,
    timeout: DurationSeconds,
}

impl ChallengeConfig {
    fn validate(&self) {
        if self.days > MAX_DAYS {
            panic!("got {} days, want <= {MAX_DAYS}", self.days);
        }
        if self.lives > MAX_DAYS {
            panic!("got {} lives, want <= {MAX_DAYS}", self.lives);
        }
    }
}

#[derive(serde::Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChallengeState {
    config: ChallengeConfig,
    // TEST ONLY
    day_length: Option<DurationSeconds>,
    days_passed: u64,
    lives_used: u64,
    funding: FundingEngine,
    day_status: Vec<bool>,
}

impl ChallengeState {
    fn update(&mut self, now: TimestampSeconds) {
        loop {
            log!(
                "update challenge loop iter now: {}, days passed: {} lives used: {}",
                now,
                self.days_passed,
                self.lives_used
            );
            // Has challenge finished?
            if self.lives_used > self.config.lives || self.days_passed >= self.config.days {
                log!("The challenge has finished.");
                break;
            }
            // Has a new day started?
            let day_start =
                self.config.first_day + self.days_passed * self.day_length.unwrap_or(DAY);
            if now < day_start {
                log!("Finished processing all days till 'now'");
                break;
            }

            // Progress the challenge state by 1 day.
            self.days_passed += 1;
            if day_start + self.config.timeout >= now {
                self.day_status.push(true);
                log!("Day {} completed!", self.days_passed);
            } else {
                log!("Day {} missing, lost a life!", self.days_passed);
                self.lives_used += 1;
                self.day_status.push(false);
            }
        }
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            challenges: UnorderedMap::new(b"v"),
            day_length: None,
        }
    }
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn reset(&mut self) {
        self.challenges.clear();
    }
    #[private]
    pub fn set_day_length(&mut self, day_length: DurationSeconds) {
        self.day_length = Some(day_length);
    }

    #[payable]
    pub fn create_challenge(&mut self, config: ChallengeConfig) {
        config.validate();
        let caller = env::predecessor_account_id();
        if self.challenges.get(&caller).is_some() {
            panic!("challenge already exists");
        }
        self.challenges.insert(
            &caller,
            &ChallengeState {
                config,
                day_length: self.day_length,
                days_passed: 0,
                lives_used: 0,
                funding: FundingEngine::new(
                    &env::predecessor_account_id(),
                    env::attached_deposit(),
                ),
                day_status: Vec::new(),
            },
        );
    }

    pub fn get_challenge(&self, account_id: AccountId) -> Option<ChallengeState> {
        self.challenges.get(&account_id)
    }

    // Must be called at the right hour of the day.
    pub fn update_challenge(&mut self) {
        let caller = env::predecessor_account_id();
        let mut ch = self.challenges.get(&caller).expect("challenge not found");
        let now = Duration::from_nanos(env::block_timestamp());
        ch.update(now.as_secs());
        self.challenges.insert(&caller, &ch);
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

    pub fn finish(&mut self) {
        let challenge = self.get_challenge(env::predecessor_account_id()).unwrap();
        if challenge.lives_used > challenge.config.lives
            || challenge.days_passed >= challenge.config.days
        {
            let prize = challenge
                .funding
                .resolve(challenge.days_passed >= challenge.config.days);
            for (account_id, tokens) in prize.iter() {
                Promise::new(account_id.clone()).transfer(*tokens);
            }
            self.challenges.remove(&env::predecessor_account_id());
        } else {
            panic_str("Challenge is not finished yet");
        }
    }
}
