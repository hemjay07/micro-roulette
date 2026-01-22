// src/types.rs
//! Core types for MicroRoulette

use linera_sdk::linera_base_types::Amount;
use serde::{Deserialize, Serialize};

// ============================================================================
// TABLE STATUS
// ============================================================================

/// Current status of the roulette table
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum TableStatus {
    /// Table is accepting bets
    #[default]
    Open,
    /// Wheel is spinning (bets locked)
    Spinning,
    /// Distributing winnings
    PayingOut,
    /// Table is closed
    Closed,
}

// ============================================================================
// ROULETTE NUMBER
// ============================================================================

/// A roulette number (0-36 for European roulette)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
pub struct RouletteNumber(pub u8);

impl RouletteNumber {
    /// Create a new roulette number, returning None if invalid
    pub fn new(n: u8) -> Option<Self> {
        if n <= 36 {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Check if this is zero (green)
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Check if this is a red number
    /// Red numbers: 1,3,5,7,9,12,14,16,18,19,21,23,25,27,30,32,34,36
    pub fn is_red(&self) -> bool {
        matches!(
            self.0,
            1 | 3 | 5 | 7 | 9 | 12 | 14 | 16 | 18 | 19 | 21 | 23 | 25 | 27 | 30 | 32 | 34 | 36
        )
    }

    /// Check if this is a black number
    pub fn is_black(&self) -> bool {
        !self.is_zero() && !self.is_red()
    }

    /// Check if this is an odd number (excludes zero)
    pub fn is_odd(&self) -> bool {
        !self.is_zero() && self.0 % 2 == 1
    }

    /// Check if this is an even number (excludes zero)
    pub fn is_even(&self) -> bool {
        !self.is_zero() && self.0 % 2 == 0
    }

    /// Check if this is in the low range (1-18)
    pub fn is_low(&self) -> bool {
        self.0 >= 1 && self.0 <= 18
    }

    /// Check if this is in the high range (19-36)
    pub fn is_high(&self) -> bool {
        self.0 >= 19 && self.0 <= 36
    }

    /// Get the dozen (1, 2, or 3) for this number
    pub fn dozen(&self) -> Option<u8> {
        match self.0 {
            1..=12 => Some(1),
            13..=24 => Some(2),
            25..=36 => Some(3),
            _ => None,
        }
    }

    /// Get the column (1, 2, or 3) for this number
    pub fn column(&self) -> Option<u8> {
        if self.is_zero() {
            None
        } else {
            Some(((self.0 - 1) % 3) + 1)
        }
    }

    /// Get color as string
    pub fn color(&self) -> &'static str {
        if self.is_zero() {
            "green"
        } else if self.is_red() {
            "red"
        } else {
            "black"
        }
    }
}

// ============================================================================
// BET TYPE
// ============================================================================

/// Types of bets a player can make (encoded as u8 for simplicity)
/// 0 = Red, 1 = Black, 2 = Odd, 3 = Even, 4 = Low (1-18), 5 = High (19-36)
/// 6 = Dozen 1, 7 = Dozen 2, 8 = Dozen 3
/// 9 = Column 1, 10 = Column 2, 11 = Column 3
/// 12-48 = Straight bet on number (bet_type - 12)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BetType(pub u8);

impl BetType {
    pub const RED: u8 = 0;
    pub const BLACK: u8 = 1;
    pub const ODD: u8 = 2;
    pub const EVEN: u8 = 3;
    pub const LOW: u8 = 4;
    pub const HIGH: u8 = 5;
    pub const DOZEN_1: u8 = 6;
    pub const DOZEN_2: u8 = 7;
    pub const DOZEN_3: u8 = 8;
    pub const COLUMN_1: u8 = 9;
    pub const COLUMN_2: u8 = 10;
    pub const COLUMN_3: u8 = 11;
    pub const STRAIGHT_OFFSET: u8 = 12;

    /// Create a new bet type
    pub fn new(value: u8) -> Option<Self> {
        if value <= 48 {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Check if this bet type is valid
    pub fn is_valid(&self) -> bool {
        self.0 <= 48
    }

    /// Returns the payout multiplier for this bet type (not including original bet)
    pub fn payout_multiplier(&self) -> u64 {
        match self.0 {
            0..=5 => 1,   // Red, Black, Odd, Even, Low, High - pays 1:1
            6..=8 => 2,   // Dozens - pays 2:1
            9..=11 => 2,  // Columns - pays 2:1
            12..=48 => 35, // Straight - pays 35:1
            _ => 0,
        }
    }

    /// Check if this bet wins for a given number
    pub fn is_winner(&self, result: RouletteNumber) -> bool {
        match self.0 {
            0 => result.is_red(),
            1 => result.is_black(),
            2 => result.is_odd(),
            3 => result.is_even(),
            4 => result.is_low(),
            5 => result.is_high(),
            6 => result.dozen() == Some(1),
            7 => result.dozen() == Some(2),
            8 => result.dozen() == Some(3),
            9 => result.column() == Some(1),
            10 => result.column() == Some(2),
            11 => result.column() == Some(3),
            n if n >= 12 && n <= 48 => result.0 == (n - 12),
            _ => false,
        }
    }

    /// Get display name for this bet type
    pub fn display_name(&self) -> String {
        match self.0 {
            0 => "Red".to_string(),
            1 => "Black".to_string(),
            2 => "Odd".to_string(),
            3 => "Even".to_string(),
            4 => "1-18".to_string(),
            5 => "19-36".to_string(),
            6 => "1st Dozen".to_string(),
            7 => "2nd Dozen".to_string(),
            8 => "3rd Dozen".to_string(),
            9 => "Column 1".to_string(),
            10 => "Column 2".to_string(),
            11 => "Column 3".to_string(),
            n if n >= 12 && n <= 48 => format!("{}", n - 12),
            _ => "Invalid".to_string(),
        }
    }
}

// ============================================================================
// BET
// ============================================================================

/// A single bet placed by a player
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bet {
    /// Type of bet
    pub bet_type: BetType,
    /// Amount wagered
    pub amount: Amount,
}

impl Bet {
    /// Create a new bet
    pub fn new(bet_type: u8, amount: Amount) -> Option<Self> {
        let bt = BetType::new(bet_type)?;
        if bt.is_valid() && amount > Amount::ZERO {
            Some(Self { bet_type: bt, amount })
        } else {
            None
        }
    }

    /// Calculate total payout if this bet wins (original bet + winnings)
    pub fn calculate_payout(&self) -> Amount {
        let multiplier = self.bet_type.payout_multiplier();
        let winnings_value = self.amount.saturating_mul(multiplier);
        self.amount.saturating_add(winnings_value)
    }
}

// ============================================================================
// PLAYER BETS
// ============================================================================

/// Collection of bets from a single player for one spin
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerBets {
    /// List of bets
    pub bets: Vec<Bet>,
    /// Total amount wagered
    pub total_amount: Amount,
}

impl PlayerBets {
    /// Create a new empty bet collection
    pub fn new() -> Self {
        Self {
            bets: Vec::new(),
            total_amount: Amount::ZERO,
        }
    }

    /// Add a bet to the collection
    pub fn add_bet(&mut self, bet: Bet) {
        self.total_amount = self.total_amount.saturating_add(bet.amount);
        self.bets.push(bet);
    }

    /// Clear all bets
    pub fn clear(&mut self) {
        self.bets.clear();
        self.total_amount = Amount::ZERO;
    }

    /// Calculate total winnings for a given result
    pub fn calculate_winnings(&self, result: RouletteNumber) -> Amount {
        let mut total = Amount::ZERO;
        for bet in &self.bets {
            if bet.bet_type.is_winner(result) {
                total = total.saturating_add(bet.calculate_payout());
            }
        }
        total
    }
}

// ============================================================================
// SPIN RESULT
// ============================================================================

/// Result of a single spin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpinResult {
    /// Unique spin identifier
    pub spin_id: u64,
    /// The winning number
    pub result: RouletteNumber,
    /// Hash of the seed used (for provable fairness)
    pub seed_hash: String,
    /// Total amount bet this spin
    pub total_bets: Amount,
    /// Total amount paid out
    pub total_payout: Amount,
}

// ============================================================================
// FAIRNESS PROOF
// ============================================================================

/// Provable fairness data for verification
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FairnessProof {
    /// Server seed (revealed after spin)
    pub server_seed: String,
    /// Client seed (from block hash or player)
    pub client_seed: String,
    /// Nonce (spin number)
    pub nonce: u64,
    /// Combined hash
    pub combined_hash: String,
    /// Resulting number (0-36)
    pub result: u8,
}

impl FairnessProof {
    /// Generate a new fairness proof
    pub fn generate(server_seed: &str, client_seed: &str, nonce: u64) -> Self {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(server_seed.as_bytes());
        hasher.update(client_seed.as_bytes());
        hasher.update(nonce.to_le_bytes());
        let hash = hasher.finalize();

        let combined_hash = hex::encode(&hash);
        let result = (hash[0] as u64 % 37) as u8;

        Self {
            server_seed: server_seed.to_string(),
            client_seed: client_seed.to_string(),
            nonce,
            combined_hash,
            result,
        }
    }

    /// Verify that the result matches the seeds
    pub fn verify(&self) -> bool {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(self.server_seed.as_bytes());
        hasher.update(self.client_seed.as_bytes());
        hasher.update(self.nonce.to_le_bytes());
        let hash = hasher.finalize();

        let computed_hash = hex::encode(&hash);
        if computed_hash != self.combined_hash {
            return false;
        }

        let computed_result = (hash[0] as u64 % 37) as u8;
        computed_result == self.result
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // TableStatus tests
    #[test]
    fn test_table_status_default() {
        let status = TableStatus::default();
        assert_eq!(status, TableStatus::Open);
    }

    #[test]
    fn test_table_status_transitions() {
        let status = TableStatus::Open;
        assert_eq!(status, TableStatus::Open);

        let spinning = TableStatus::Spinning;
        assert_eq!(spinning, TableStatus::Spinning);
        assert_ne!(spinning, TableStatus::Open);

        let paying_out = TableStatus::PayingOut;
        assert_eq!(paying_out, TableStatus::PayingOut);

        let closed = TableStatus::Closed;
        assert_eq!(closed, TableStatus::Closed);
    }

    // RouletteNumber tests
    #[test]
    fn test_roulette_number_creation() {
        assert!(RouletteNumber::new(0).is_some());
        assert!(RouletteNumber::new(36).is_some());
        assert!(RouletteNumber::new(37).is_none());
    }

    #[test]
    fn test_roulette_number_zero() {
        let zero = RouletteNumber::new(0).unwrap();
        assert!(zero.is_zero());
        assert!(!zero.is_red());
        assert!(!zero.is_black());
        assert_eq!(zero.color(), "green");
    }

    #[test]
    fn test_roulette_number_red() {
        let red_numbers = vec![1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
        for n in red_numbers {
            let num = RouletteNumber::new(n).unwrap();
            assert!(num.is_red(), "Number {} should be red", n);
            assert!(!num.is_black(), "Number {} should not be black", n);
            assert_eq!(num.color(), "red", "Number {} should have color red", n);
        }
    }

    #[test]
    fn test_roulette_number_black() {
        let black_numbers = vec![2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
        for n in black_numbers {
            let num = RouletteNumber::new(n).unwrap();
            assert!(num.is_black(), "Number {} should be black", n);
            assert!(!num.is_red(), "Number {} should not be red", n);
            assert_eq!(num.color(), "black", "Number {} should have color black", n);
        }
    }

    #[test]
    fn test_roulette_number_odd_even() {
        let zero = RouletteNumber::new(0).unwrap();
        assert!(!zero.is_odd());
        assert!(!zero.is_even());

        let one = RouletteNumber::new(1).unwrap();
        assert!(one.is_odd());
        assert!(!one.is_even());

        let two = RouletteNumber::new(2).unwrap();
        assert!(!two.is_odd());
        assert!(two.is_even());
    }

    #[test]
    fn test_roulette_number_low_high() {
        let zero = RouletteNumber::new(0).unwrap();
        assert!(!zero.is_low());
        assert!(!zero.is_high());

        let one = RouletteNumber::new(1).unwrap();
        assert!(one.is_low());
        assert!(!one.is_high());

        let eighteen = RouletteNumber::new(18).unwrap();
        assert!(eighteen.is_low());
        assert!(!eighteen.is_high());

        let nineteen = RouletteNumber::new(19).unwrap();
        assert!(!nineteen.is_low());
        assert!(nineteen.is_high());

        let thirty_six = RouletteNumber::new(36).unwrap();
        assert!(!thirty_six.is_low());
        assert!(thirty_six.is_high());
    }

    #[test]
    fn test_roulette_number_dozens() {
        let zero = RouletteNumber::new(0).unwrap();
        assert_eq!(zero.dozen(), None);

        let one = RouletteNumber::new(1).unwrap();
        assert_eq!(one.dozen(), Some(1));

        let twelve = RouletteNumber::new(12).unwrap();
        assert_eq!(twelve.dozen(), Some(1));

        let thirteen = RouletteNumber::new(13).unwrap();
        assert_eq!(thirteen.dozen(), Some(2));

        let twenty_four = RouletteNumber::new(24).unwrap();
        assert_eq!(twenty_four.dozen(), Some(2));

        let twenty_five = RouletteNumber::new(25).unwrap();
        assert_eq!(twenty_five.dozen(), Some(3));

        let thirty_six = RouletteNumber::new(36).unwrap();
        assert_eq!(thirty_six.dozen(), Some(3));
    }

    #[test]
    fn test_roulette_number_columns() {
        let zero = RouletteNumber::new(0).unwrap();
        assert_eq!(zero.column(), None);

        // Column 1: 1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34
        for n in [1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34] {
            let num = RouletteNumber::new(n).unwrap();
            assert_eq!(num.column(), Some(1), "Number {} should be in column 1", n);
        }

        // Column 2: 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35
        for n in [2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35] {
            let num = RouletteNumber::new(n).unwrap();
            assert_eq!(num.column(), Some(2), "Number {} should be in column 2", n);
        }

        // Column 3: 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36
        for n in [3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36] {
            let num = RouletteNumber::new(n).unwrap();
            assert_eq!(num.column(), Some(3), "Number {} should be in column 3", n);
        }
    }

    // BetType tests
    #[test]
    fn test_bet_type_creation() {
        assert!(BetType::new(0).is_some());
        assert!(BetType::new(48).is_some());
        assert!(BetType::new(49).is_none());
    }

    #[test]
    fn test_bet_type_payouts() {
        // Even money bets pay 1:1
        assert_eq!(BetType::new(BetType::RED).unwrap().payout_multiplier(), 1);
        assert_eq!(BetType::new(BetType::BLACK).unwrap().payout_multiplier(), 1);
        assert_eq!(BetType::new(BetType::ODD).unwrap().payout_multiplier(), 1);
        assert_eq!(BetType::new(BetType::EVEN).unwrap().payout_multiplier(), 1);
        assert_eq!(BetType::new(BetType::LOW).unwrap().payout_multiplier(), 1);
        assert_eq!(BetType::new(BetType::HIGH).unwrap().payout_multiplier(), 1);

        // Dozens and columns pay 2:1
        assert_eq!(BetType::new(BetType::DOZEN_1).unwrap().payout_multiplier(), 2);
        assert_eq!(BetType::new(BetType::DOZEN_2).unwrap().payout_multiplier(), 2);
        assert_eq!(BetType::new(BetType::DOZEN_3).unwrap().payout_multiplier(), 2);
        assert_eq!(BetType::new(BetType::COLUMN_1).unwrap().payout_multiplier(), 2);
        assert_eq!(BetType::new(BetType::COLUMN_2).unwrap().payout_multiplier(), 2);
        assert_eq!(BetType::new(BetType::COLUMN_3).unwrap().payout_multiplier(), 2);

        // Straight bets pay 35:1
        assert_eq!(BetType::new(BetType::STRAIGHT_OFFSET).unwrap().payout_multiplier(), 35);
        assert_eq!(BetType::new(BetType::STRAIGHT_OFFSET + 17).unwrap().payout_multiplier(), 35);
    }

    #[test]
    fn test_bet_type_winners() {
        let red_17 = RouletteNumber::new(17).unwrap();
        let black_20 = RouletteNumber::new(20).unwrap();
        let zero = RouletteNumber::new(0).unwrap();

        // Red/Black
        assert!(BetType::new(BetType::BLACK).unwrap().is_winner(red_17)); // 17 is actually black!
        assert!(!BetType::new(BetType::RED).unwrap().is_winner(red_17));
        assert!(BetType::new(BetType::BLACK).unwrap().is_winner(black_20));
        assert!(!BetType::new(BetType::RED).unwrap().is_winner(zero));
        assert!(!BetType::new(BetType::BLACK).unwrap().is_winner(zero));

        // Odd/Even
        assert!(BetType::new(BetType::ODD).unwrap().is_winner(red_17));
        assert!(!BetType::new(BetType::EVEN).unwrap().is_winner(red_17));
        assert!(BetType::new(BetType::EVEN).unwrap().is_winner(black_20));

        // Straight
        assert!(BetType::new(BetType::STRAIGHT_OFFSET + 17).unwrap().is_winner(red_17));
        assert!(!BetType::new(BetType::STRAIGHT_OFFSET + 18).unwrap().is_winner(red_17));
        assert!(BetType::new(BetType::STRAIGHT_OFFSET).unwrap().is_winner(zero)); // Bet on 0
    }

    // FairnessProof tests
    #[test]
    fn test_fairness_proof_generation() {
        let proof = FairnessProof::generate("server_seed_123", "client_seed_abc", 1);
        assert!(!proof.combined_hash.is_empty());
        assert!(proof.result <= 36);
    }

    #[test]
    fn test_fairness_proof_verification() {
        let proof = FairnessProof::generate("server_seed_123", "client_seed_abc", 1);
        assert!(proof.verify(), "Generated proof should verify");
    }

    #[test]
    fn test_fairness_proof_consistency() {
        let proof1 = FairnessProof::generate("same_seed", "same_client", 42);
        let proof2 = FairnessProof::generate("same_seed", "same_client", 42);
        assert_eq!(proof1.combined_hash, proof2.combined_hash);
        assert_eq!(proof1.result, proof2.result);
    }

    #[test]
    fn test_fairness_proof_different_nonce() {
        let proof1 = FairnessProof::generate("seed", "client", 1);
        let proof2 = FairnessProof::generate("seed", "client", 2);
        assert_ne!(proof1.combined_hash, proof2.combined_hash);
    }
}
