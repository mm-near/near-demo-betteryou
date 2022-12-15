use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::unordered_map::UnorderedMap;
use near_sdk::serde;
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, Promise};
use std::time::Duration;

// Full seconds since UNIX_EPOCH.
pub type TimestampSeconds = u64;

// Seconds
pub type DurationSeconds = u64;

const DAY: DurationSeconds = 24 * 60 * 60;
const MAX_DAYS: u64 = 1000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    challenges: UnorderedMap<AccountId, ChallengeState>,
}

#[derive(serde::Serialize, serde::Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChallengeConfig {
    lives: u64,
    days: u64,
    first_day: TimestampSeconds,
    timeout: DurationSeconds,
    // TESTONLY: set custom day length.
    day_length: Option<DurationSeconds>,
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
    days_passed: u64,
    lives_used: u64,
    prize: Balance,
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
                self.config.first_day + self.days_passed * self.config.day_length.unwrap_or(DAY);
            if now < day_start {
                log!("Finished processing all days till 'now'");
                break;
            }

            // Progress the challenge state by 1 day.
            self.days_passed += 1;
            if day_start + self.config.timeout >= now {
                log!("Day {} completed!", self.days_passed);
            } else {
                log!("Day {} missing, lost a life!", self.days_passed);
                self.lives_used += 1;
            }
        }
    }

    fn final_prize(&self) -> Option<Balance> {
        if self.lives_used >= self.config.lives {
            return Some(0);
        }
        if self.days_passed < self.config.days {
            return None;
        }
        Some(self.prize)
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            challenges: UnorderedMap::new(b"v"),
        }
    }

    pub fn reset(&mut self) {
        if env::predecessor_account_id() != env::current_account_id() {
            panic!("reset() can be called by admin only");
        }
        self.challenges.clear();
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
                days_passed: 0,
                lives_used: 0,
                prize: env::attached_deposit(),
            },
        );
    }

    pub fn get_challenge(&self, account_id: AccountId) -> Option<ChallengeState> {
        self.challenges.get(&account_id)
    }

    pub fn update_challenge(&mut self) {
        let caller = env::predecessor_account_id();
        let mut ch = self.challenges.get(&caller).expect("challenge not found");
        let now = Duration::from_nanos(env::block_timestamp());
        ch.update(now.as_secs());
        self.challenges.insert(&caller, &ch);
    }

    #[payable]
    pub fn add_prize(&mut self, account_id: AccountId) {
        let mut ch = self
            .challenges
            .get(&account_id)
            .expect("challenge not found");
        ch.prize += env::attached_deposit();
        self.challenges.insert(&account_id, &ch);
    }

    pub fn abandon_challenge(&mut self) {
        let caller = env::predecessor_account_id();
        self.challenges.remove(&caller);
    }

    pub fn finish_challenge(&mut self) -> Option<Balance> {
        let caller = env::predecessor_account_id();
        let mut ch = self.challenges.get(&caller).expect("challenge not found");
        ch.update(env::block_timestamp());
        let prize = ch.final_prize();
        if let Some(prize) = prize {
            log!("Challenge finished");
            Promise::new(caller.clone()).transfer(prize);
            self.challenges.remove(&caller);
        } else {
            log!("The challenge hasn't finished yet!");
        }
        prize
    }
}
