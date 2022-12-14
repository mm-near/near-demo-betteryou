use std::collections::HashMap;

use near_sdk::{AccountId, Balance};

#[derive(Debug)]
pub struct Stake {
    backer: AccountId,
    value: Balance,
}

pub struct FundingEngine {
    founder: AccountId,
    initial_stake: Balance,
    backers: Vec<Stake>,
}

impl FundingEngine {
    pub fn new(founder: AccountId, initial_stake: Balance) -> Self {
        Self {
            founder,
            initial_stake,
            backers: Vec::new(),
        }
    }

    /// Funds the commitment.
    pub fn fund(&mut self, backer: AccountId, value: Balance) {
        self.backers.push(Stake { backer, value });
    }

    /// Resolves the commitment and returns payouts to each backer.
    pub fn resolve(&self, success: bool) -> HashMap<AccountId, Balance> {
        let total_stake =
            self.initial_stake + self.backers.iter().map(|b| b.value).sum::<Balance>();
        let mut payouts = HashMap::<AccountId, Balance>::new();
        payouts.insert(self.founder.clone(), if success { total_stake } else { 0 });
        for backing in &self.backers {
            payouts.insert(
                backing.backer.clone(),
                if success { 0 } else { backing.value },
            );
        }
        payouts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_backers() {
        let my_account: AccountId = "me".parse().unwrap();
        let engine = FundingEngine::new(my_account.clone(), 42);
        assert_eq!(engine.resolve(true), [(my_account.clone(), 42)].into());
        assert_eq!(engine.resolve(false), [(my_account.clone(), 0)].into());
    }

    #[test]
    fn single_backer() {
        let my_account: AccountId = "me".parse().unwrap();
        let friend_account: AccountId = "friend".parse().unwrap();
        let mut engine = FundingEngine::new(my_account.clone(), 10);
        engine.fund(friend_account.clone(), 20);
        assert_eq!(
            engine.resolve(true),
            [(my_account.clone(), 30), (friend_account.clone(), 0)].into()
        );
        assert_eq!(
            engine.resolve(false),
            [(my_account.clone(), 0), (friend_account.clone(), 20)].into()
        );
    }
}
