#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    Success,
    Failure,
}

#[derive(Debug)]
pub struct Bet {
    player_id: String,
    value: u32,
    outcome: Outcome,
}

#[derive(Default)]
pub struct BettingEngine {
    bets: Vec<Bet>,
}

impl BettingEngine {
    /// Adds a new bet for success.
    pub fn bet_for(&mut self, player_id: &str, value: u32) {
        self.bets.push(Bet {
            player_id: player_id.to_string(),
            value,
            outcome: Outcome::Success,
        });
    }

    /// Adds a new bet for failure.
    pub fn bet_against(&mut self, player_id: &str, value: u32) {
        self.bets.push(Bet {
            player_id: player_id.to_string(),
            value,
            outcome: Outcome::Failure,
        });
    }

    /// Resolves the bet and returns payouts to each player.
    pub fn resolve(&mut self, outcome: Outcome) -> Vec<(String, u32)> {
        let total_bet: u32 = self.bets.iter().map(|b| b.value).sum();
        let outcome_side_bet: u32 = self
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

        let mut payouts = Vec::new();
        for bet in &self.bets {
            let payout = if bet.outcome == outcome {
                bet.value * total_bet / outcome_side_bet
            } else {
                0
            };
            payouts.push((bet.player_id.clone(), payout));
        }
        payouts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_player_win() {
        let mut engine = BettingEngine::default();
        engine.bet_for("owner", 42);
        let payouts = engine.resolve(Outcome::Success);
        assert_eq!(payouts, vec![("owner".to_string(), 42)]);
    }

    #[test]
    fn single_player_lose() {
        let mut engine = BettingEngine::default();
        engine.bet_for("owner", 42);
        let payouts = engine.resolve(Outcome::Failure);
        assert_eq!(payouts, vec![("owner".to_string(), 0)]);
    }

    #[test]
    fn two_players() {
        let mut engine = BettingEngine::default();
        engine.bet_for("owner", 10);
        engine.bet_against("friend", 20);
        let payouts = engine.resolve(Outcome::Success);
        assert_eq!(
            payouts,
            vec![("owner".to_string(), 30), ("friend".to_string(), 0)]
        );
    }
}
