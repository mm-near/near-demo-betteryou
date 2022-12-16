use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::panic_str;
use near_sdk::{serde::Serialize, AccountId, Balance};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Debug)]

pub struct Stake {
    backer: AccountId,
    value: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
pub struct FundingEngine {
    founder: AccountId,
    initial_stake: Balance,
    backers: Vec<Stake>,
}

// Don't allow too many backers (are refunds would be hard to fit in gas limit).
const MAX_BACKERS: usize = 100;

impl FundingEngine {
    pub fn new(founder: &AccountId, initial_stake: Balance) -> Self {
        Self {
            founder: founder.clone(),
            initial_stake,
            backers: Vec::new(),
        }
    }

    /// Funds the commitment.
    pub fn fund(&mut self, backer: &AccountId, value: Balance) {
        if self.backers.len() >= MAX_BACKERS {
            panic_str("Maximum amount of backers reached.");
        }
        self.backers.push(Stake {
            backer: backer.clone(),
            value,
        });
    }

    /// Resolves the commitment and returns payouts to each backer.
    pub fn resolve(&self, success: bool) -> HashMap<AccountId, Balance> {
        let mut payouts = HashMap::<AccountId, Balance>::new();
        if success {
            // If challenge was successfull - all money goes to the founder.
            let total_stake =
                self.initial_stake + self.backers.iter().map(|b| b.value).sum::<Balance>();
            payouts.insert(self.founder.clone(), total_stake);
        } else {
            // Return money to backers (the initial_stake will stay with the contract).
            for backing in &self.backers {
                payouts.insert(backing.backer.clone(), backing.value);
            }
        }
        payouts
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    Success,
    Failure,
}

#[derive(Debug)]
pub struct Bet {
    player: AccountId,
    value: Balance,
    outcome: Outcome,
}

pub struct BettingEngine {
    bets: Vec<Bet>,
}

impl BettingEngine {
    pub fn new(founder: &AccountId, initial_stake: Balance) -> Self {
        Self {
            bets: vec![Bet {
                player: founder.clone(),
                value: initial_stake,
                outcome: Outcome::Success,
            }],
        }
    }

    /// Adds a new bet for success.
    pub fn bet_for(&mut self, player: &AccountId, value: Balance) {
        self.bets.push(Bet {
            player: player.clone(),
            value,
            outcome: Outcome::Success,
        });
    }

    /// Adds a new bet for failure.
    pub fn bet_against(&mut self, player: &AccountId, value: Balance) {
        self.bets.push(Bet {
            player: player.clone(),
            value,
            outcome: Outcome::Failure,
        });
    }

    /// Resolves the bet and returns payouts to each player.
    pub fn resolve(&mut self, outcome: Outcome) -> HashMap<AccountId, Balance> {
        let total_bet: Balance = self.bets.iter().map(|b| b.value).sum();
        let outcome_side_bet: Balance = self
            .bets
            .iter()
            .filter_map(|b| {
                if b.outcome == outcome {
                    Some(b.value)
                } else {
                    None
                }
            })
            .sum();

        let mut payouts = HashMap::new();
        for bet in &self.bets {
            payouts.insert(
                bet.player.clone(),
                if bet.outcome == outcome {
                    bet.value * total_bet / outcome_side_bet
                } else {
                    0
                },
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
        let engine = FundingEngine::new(&my_account, 42);
        assert_eq!(engine.resolve(true), [(my_account.clone(), 42)].into());
        assert_eq!(engine.resolve(false), [].into());
    }

    #[test]
    fn single_backer() {
        let my_account: AccountId = "me".parse().unwrap();
        let friend_account: AccountId = "friend".parse().unwrap();
        let mut engine = FundingEngine::new(&my_account, 10);
        engine.fund(&friend_account, 20);
        assert_eq!(engine.resolve(true), [(my_account.clone(), 30)].into());
        assert_eq!(engine.resolve(false), [(friend_account.clone(), 20)].into());
    }

    #[test]
    fn single_player_win() {
        let my_account: AccountId = "me".parse().unwrap();
        let mut engine = BettingEngine::new(&my_account, 42);
        assert_eq!(
            engine.resolve(Outcome::Success),
            [(my_account.clone(), 42)].into()
        );
        assert_eq!(
            engine.resolve(Outcome::Failure),
            [(my_account.clone(), 0)].into()
        );
    }

    #[test]
    fn two_players() {
        let my_account: AccountId = "me".parse().unwrap();
        let friend_account: AccountId = "friend".parse().unwrap();
        let mut engine = BettingEngine::new(&my_account, 10);
        engine.bet_against(&friend_account, 20);
        assert_eq!(
            engine.resolve(Outcome::Success),
            [(my_account.clone(), 30), (friend_account.clone(), 0)].into()
        );
    }
}
