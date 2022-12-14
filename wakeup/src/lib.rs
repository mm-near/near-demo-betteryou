use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::collections::{LookupMap};
use near_sdk::serde;


// Full seconds since UNIX_EPOCH.
pub type Timestamp = u64;

// Seconds
pub type Duration = u64;

const DAY : Duration = 24*60*60;
const MAX_DAYS : u64 = 1000;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    challenges: LookupMap<AccountId,ChallengeState>,
}

#[derive(serde::Serialize, serde::Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChallengeConfig {
    lives: u64,
    days: u64,
    first_day: Timestamp,
    timeout: Duration,
}

impl ChallengeConfig {
    fn validate(&self) {
        if self.days>MAX_DAYS {
            panic!("got {} days, want <= {MAX_DAYS}",self.days);
        }
        if self.lives>self.days {
            panic!("got {} lives, want <= {MAX_DAYS}",self.lives);
        }
    }
}

#[derive(serde::Serialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ChallengeState {
    config: ChallengeConfig,
    days_passed: u64,
    lives_used: u64,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            challenges: LookupMap::new(b"c"),
        }
    }

    pub fn create_challenge(&mut self, config: ChallengeConfig) {
        // TODO: token allocation
        config.validate();
        let caller = env::predecessor_account_id();
        if self.challenges.contains_key(&caller) {
            panic!("challenge already exists");
        }
        self.challenges.insert(&caller, &ChallengeState{
            config,
            days_passed: 0,
            lives_used: 0,
        });
    }

    pub fn get_challenge(&self, account_id: AccountId) -> Option<ChallengeState> {
        self.challenges.get(&account_id)
    }

    pub fn register_wakeup(&mut self) {
        let caller = env::predecessor_account_id();
        let mut ch = self.challenges.get(&caller).expect("challenge not found");

        let now : Timestamp = env::block_timestamp();
        loop {
            // Has challenge finished?
            if ch.lives_used > ch.config.lives || ch.days_passed >= ch.config.days {
                break;
            }
            // Has a new day started?
            let day_start = ch.config.first_day + ch.days_passed*DAY;
            if now < day_start {
                break;
            }
            // Progress the challenge state by 1 day.
            ch.days_passed += 1;
            if day_start + ch.config.timeout < now {
                ch.lives_used += 1;
            }
        }
        // Update the challenge state.
        self.challenges.insert(&caller,&ch);
    }

    pub fn delete_challenge(&mut self) {
        let caller = env::predecessor_account_id();
        self.challenges.remove(&caller).expect("challenge not found");    
    }
}
