use std::collections::HashMap;

use near_sdk::Balance;

#[derive(Debug)]
pub struct Stake {
    backer: String,
    value: Balance,
}

#[derive(Default)]
pub struct FundingEngine {
    founder: String,
    initial_stake: Balance,
    backers: Vec<Stake>,
}

impl FundingEngine {
    pub fn new(founder: &str, initial_stake: Balance) -> Self {
        Self {
            founder: founder.to_string(),
            initial_stake,
            backers: Vec::new(),
        }
    }

    /// Funds the commitment.
    pub fn fund(&mut self, backer: &str, value: Balance) {
        self.backers.push(Stake {
            backer: backer.to_string(),
            value,
        });
    }

    /// Resolves the commitment and returns payouts to each backer.
    pub fn resolve(&self, success: bool) -> HashMap<String, Balance> {
        let total_stake =
            self.initial_stake + self.backers.iter().map(|b| b.value).sum::<Balance>();
        let mut payouts = HashMap::<String, Balance>::new();
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
        let engine = FundingEngine::new("me", 42);
        assert_eq!(engine.resolve(true), [("me".to_string(), 42)].into());
        assert_eq!(engine.resolve(false), [("me".to_string(), 0)].into());
    }

    #[test]
    fn single_backer() {
        let mut engine = FundingEngine::new("me", 10);
        engine.fund("friend", 20);
        assert_eq!(
            engine.resolve(true),
            [("me".to_string(), 30), ("friend".to_string(), 0)].into()
        );
        assert_eq!(
            engine.resolve(false),
            [("me".to_string(), 0), ("friend".to_string(), 20)].into()
        );
    }
}
