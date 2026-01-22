// src/types.rs
//! Core types for MicroRoulette
//!
//! Defines all the domain types including bets, results, and player data.

use linera_sdk::base::{Amount, Owner, Timestamp};
use serde::{Deserialize, Serialize};

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
    /// Column 1: 1,4,7,10,13,16,19,22,25,28,31,34
    /// Column 2: 2,5,8,11,14,17,20,23,26,29,32,35
    /// Column 3: 3,6,9,12,15,18,21,24,27,30,33,36
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
// BET TYPES
// ============================================================================

/// Types of bets a player can make
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BetType {
    // Inside bets (higher payout, lower odds)
    /// Single number (0-36), pays 35:1
    Straight(u8),
    /// Two adjacent numbers, pays 17:1
    Split(u8, u8),
    /// Row of 3 (street), pays 11:1. Value is the starting number (1, 4, 7, etc.)
    Street(u8),
    /// Four numbers (corner), pays 8:1
    Corner(u8, u8, u8, u8),
    /// Two rows (6 numbers), pays 5:1. Value is the starting row (1, 4, 7, etc.)
    SixLine(u8),

    // Outside bets (lower payout, higher odds)
    /// Red numbers, pays 1:1
    Red,
    /// Black numbers, pays 1:1
    Black,
    /// Odd numbers, pays 1:1
    Odd,
    /// Even numbers, pays 1:1
    Even,
    /// Low (1-18), pays 1:1
    Low,
    /// High (19-36), pays 1:1
    High,
    /// Dozen (1=1-12, 2=13-24, 3=25-36), pays 2:1
    Dozen(u8),
    /// Column (1, 2, or 3), pays 2:1
    Column(u8),
}

impl BetType {
    /// Returns the payout multiplier for this bet type (not including original bet)
    pub fn payout_multiplier(&self) -> u64 {
        match self {
            BetType::Straight(_) => 35,
            BetType::Split(_, _) => 17,
            BetType::Street(_) => 11,
            BetType::Corner(_, _, _, _) => 8,
            BetType::SixLine(_) => 5,
            BetType::Red | BetType::Black => 1,
            BetType::Odd | BetType::Even => 1,
            BetType::Low | BetType::High => 1,
            BetType::Dozen(_) | BetType::Column(_) => 2,
        }
    }

    /// Check if this bet wins for a given number
    pub fn is_winner(&self, result: RouletteNumber) -> bool {
        match self {
            BetType::Straight(n) => result.0 == *n,
            BetType::Split(a, b) => result.0 == *a || result.0 == *b,
            BetType::Street(start) => {
                let s = *start;
                result.0 >= s && result.0 < s + 3 && !result.is_zero()
            }
            BetType::Corner(a, b, c, d) => {
                result.0 == *a || result.0 == *b || result.0 == *c || result.0 == *d
            }
            BetType::SixLine(start) => {
                let s = *start;
                result.0 >= s && result.0 < s + 6 && !result.is_zero()
            }
            BetType::Red => result.is_red(),
            BetType::Black => result.is_black(),
            BetType::Odd => result.is_odd(),
            BetType::Even => result.is_even(),
            BetType::Low => result.is_low(),
            BetType::High => result.is_high(),
            BetType::Dozen(d) => result.dozen() == Some(*d),
            BetType::Column(c) => result.column() == Some(*c),
        }
    }

    /// Validate that the bet type parameters are valid
    pub fn is_valid(&self) -> bool {
        match self {
            BetType::Straight(n) => *n <= 36,
            BetType::Split(a, b) => *a <= 36 && *b <= 36 && Self::are_adjacent(*a, *b),
            BetType::Street(start) => *start >= 1 && *start <= 34 && (*start - 1) % 3 == 0,
            BetType::Corner(a, b, c, d) => {
                *a <= 36 && *b <= 36 && *c <= 36 && *d <= 36 && *a > 0
            }
            BetType::SixLine(start) => *start >= 1 && *start <= 31 && (*start - 1) % 3 == 0,
            BetType::Dozen(d) => *d >= 1 && *d <= 3,
            BetType::Column(c) => *c >= 1 && *c <= 3,
            _ => true,
        }
    }

    /// Check if two numbers are adjacent on the betting board
    fn are_adjacent(a: u8, b: u8) -> bool {
        if a == 0 || b == 0 {
            return false;
        }
        let diff = if a > b { a - b } else { b - a };
        // Adjacent horizontally (diff = 1) or vertically (diff = 3)
        diff == 1 || diff == 3
    }

    /// Get a display name for this bet type
    pub fn display_name(&self) -> String {
        match self {
            BetType::Straight(n) => format!("Straight {}", n),
            BetType::Split(a, b) => format!("Split {}/{}", a, b),
            BetType::Street(s) => format!("Street {}-{}", s, s + 2),
            BetType::Corner(a, b, c, d) => format!("Corner {}/{}/{}/{}", a, b, c, d),
            BetType::SixLine(s) => format!("Six Line {}-{}", s, s + 5),
            BetType::Red => "Red".to_string(),
            BetType::Black => "Black".to_string(),
            BetType::Odd => "Odd".to_string(),
            BetType::Even => "Even".to_string(),
            BetType::Low => "1-18".to_string(),
            BetType::High => "19-36".to_string(),
            BetType::Dozen(d) => format!("Dozen {}", d),
            BetType::Column(c) => format!("Column {}", c),
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
    /// Create a new bet, returning None if invalid
    pub fn new(bet_type: BetType, amount: Amount) -> Option<Self> {
        if bet_type.is_valid() && amount > Amount::ZERO {
            Some(Self { bet_type, amount })
        } else {
            None
        }
    }

    /// Calculate total payout if this bet wins (original bet + winnings)
    pub fn calculate_payout(&self) -> Amount {
        let multiplier = self.bet_type.payout_multiplier();
        let winnings = Amount::from_tokens(self.amount.as_tokens() * multiplier);
        self.amount.saturating_add(winnings)
    }
}

// ============================================================================
// PLAYER BETS
// ============================================================================

/// Collection of bets from a single player for one spin
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerBets {
    /// Player's address
    pub player: Owner,
    /// List of bets
    pub bets: Vec<Bet>,
    /// Total amount wagered
    pub total_amount: Amount,
}

impl PlayerBets {
    /// Create a new empty bet collection for a player
    pub fn new(player: Owner) -> Self {
        Self {
            player,
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

    /// Calculate maximum potential payout (if all bets win)
    pub fn max_potential_payout(&self) -> Amount {
        let mut total = Amount::ZERO;
        for bet in &self.bets {
            total = total.saturating_add(bet.calculate_payout());
        }
        total
    }
}

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
// SPIN RESULT
// ============================================================================

/// Result of a single spin
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpinResult {
    /// Unique spin identifier
    pub spin_id: u64,
    /// The winning number
    pub result: RouletteNumber,
    /// When the spin occurred
    pub timestamp: Timestamp,
    /// Hash of the seed used (for provable fairness)
    pub seed_hash: String,
    /// Total amount bet this spin
    pub total_bets: Amount,
    /// Total amount paid out
    pub total_payout: Amount,
    /// Number of players who bet
    pub player_count: u32,
}

// ============================================================================
// PLAYER STATS
// ============================================================================

/// Lifetime statistics for a player
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Total number of spins participated in
    pub total_spins: u64,
    /// Total amount wagered
    pub total_wagered: Amount,
    /// Total amount won
    pub total_won: Amount,
    /// Total amount lost
    pub total_lost: Amount,
    /// Biggest single win
    pub biggest_win: Amount,
    /// Current streak (positive = winning, negative = losing)
    pub current_streak: i32,
    /// Best winning streak
    pub best_streak: i32,
}

// ============================================================================
// PROVABLE FAIRNESS
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

        // Result is first byte mod 37 (consistent algorithm)
        let computed_result = (hash[0] as u64 % 37) as u8;
        computed_result == self.result
    }

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

    /// Hash a server seed (for commitment before spin)
    pub fn hash_seed(seed: &str) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        hex::encode(hasher.finalize())
    }
}

// ============================================================================
// TABLE CONFIG
// ============================================================================

/// Configuration for a roulette table
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableConfig {
    /// Minimum bet amount
    pub min_bet: Amount,
    /// Maximum bet per position
    pub max_bet: Amount,
    /// Maximum total bet per spin per player
    pub max_total_bet: Amount,
    /// Betting time in seconds before auto-spin
    pub betting_time_seconds: u64,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            min_bet: Amount::from_tokens(1_000_000),        // 1 token
            max_bet: Amount::from_tokens(100_000_000),     // 100 tokens
            max_total_bet: Amount::from_tokens(500_000_000), // 500 tokens
            betting_time_seconds: 30,
        }
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // RouletteNumber tests
    #[test]
    fn test_roulette_number_new_valid() {
        for n in 0..=36 {
            assert!(RouletteNumber::new(n).is_some());
        }
    }

    #[test]
    fn test_roulette_number_new_invalid() {
        assert!(RouletteNumber::new(37).is_none());
        assert!(RouletteNumber::new(100).is_none());
    }

    #[test]
    fn test_roulette_number_is_zero() {
        assert!(RouletteNumber(0).is_zero());
        assert!(!RouletteNumber(1).is_zero());
    }

    #[test]
    fn test_roulette_number_is_red() {
        let red_numbers = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
        for n in red_numbers {
            assert!(RouletteNumber(n).is_red(), "Number {} should be red", n);
        }
        assert!(!RouletteNumber(0).is_red());
        assert!(!RouletteNumber(2).is_red());
    }

    #[test]
    fn test_roulette_number_is_black() {
        let black_numbers = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];
        for n in black_numbers {
            assert!(RouletteNumber(n).is_black(), "Number {} should be black", n);
        }
        assert!(!RouletteNumber(0).is_black());
        assert!(!RouletteNumber(1).is_black());
    }

    #[test]
    fn test_roulette_number_odd_even() {
        assert!(RouletteNumber(1).is_odd());
        assert!(RouletteNumber(3).is_odd());
        assert!(RouletteNumber(2).is_even());
        assert!(RouletteNumber(4).is_even());
        // Zero is neither
        assert!(!RouletteNumber(0).is_odd());
        assert!(!RouletteNumber(0).is_even());
    }

    #[test]
    fn test_roulette_number_low_high() {
        assert!(RouletteNumber(1).is_low());
        assert!(RouletteNumber(18).is_low());
        assert!(!RouletteNumber(19).is_low());
        assert!(RouletteNumber(19).is_high());
        assert!(RouletteNumber(36).is_high());
        assert!(!RouletteNumber(18).is_high());
        // Zero is neither
        assert!(!RouletteNumber(0).is_low());
        assert!(!RouletteNumber(0).is_high());
    }

    #[test]
    fn test_roulette_number_dozen() {
        assert_eq!(RouletteNumber(1).dozen(), Some(1));
        assert_eq!(RouletteNumber(12).dozen(), Some(1));
        assert_eq!(RouletteNumber(13).dozen(), Some(2));
        assert_eq!(RouletteNumber(24).dozen(), Some(2));
        assert_eq!(RouletteNumber(25).dozen(), Some(3));
        assert_eq!(RouletteNumber(36).dozen(), Some(3));
        assert_eq!(RouletteNumber(0).dozen(), None);
    }

    #[test]
    fn test_roulette_number_column() {
        assert_eq!(RouletteNumber(1).column(), Some(1));
        assert_eq!(RouletteNumber(4).column(), Some(1));
        assert_eq!(RouletteNumber(2).column(), Some(2));
        assert_eq!(RouletteNumber(5).column(), Some(2));
        assert_eq!(RouletteNumber(3).column(), Some(3));
        assert_eq!(RouletteNumber(6).column(), Some(3));
        assert_eq!(RouletteNumber(0).column(), None);
    }

    // BetType tests
    #[test]
    fn test_bet_type_straight_payout() {
        assert_eq!(BetType::Straight(7).payout_multiplier(), 35);
    }

    #[test]
    fn test_bet_type_split_payout() {
        assert_eq!(BetType::Split(1, 2).payout_multiplier(), 17);
    }

    #[test]
    fn test_bet_type_street_payout() {
        assert_eq!(BetType::Street(1).payout_multiplier(), 11);
    }

    #[test]
    fn test_bet_type_corner_payout() {
        assert_eq!(BetType::Corner(1, 2, 4, 5).payout_multiplier(), 8);
    }

    #[test]
    fn test_bet_type_sixline_payout() {
        assert_eq!(BetType::SixLine(1).payout_multiplier(), 5);
    }

    #[test]
    fn test_bet_type_outside_payouts() {
        assert_eq!(BetType::Red.payout_multiplier(), 1);
        assert_eq!(BetType::Black.payout_multiplier(), 1);
        assert_eq!(BetType::Odd.payout_multiplier(), 1);
        assert_eq!(BetType::Even.payout_multiplier(), 1);
        assert_eq!(BetType::Low.payout_multiplier(), 1);
        assert_eq!(BetType::High.payout_multiplier(), 1);
        assert_eq!(BetType::Dozen(1).payout_multiplier(), 2);
        assert_eq!(BetType::Column(1).payout_multiplier(), 2);
    }

    #[test]
    fn test_bet_type_is_winner_straight() {
        assert!(BetType::Straight(7).is_winner(RouletteNumber(7)));
        assert!(!BetType::Straight(7).is_winner(RouletteNumber(8)));
    }

    #[test]
    fn test_bet_type_is_winner_red() {
        assert!(BetType::Red.is_winner(RouletteNumber(1)));  // Red
        assert!(!BetType::Red.is_winner(RouletteNumber(2))); // Black
        assert!(!BetType::Red.is_winner(RouletteNumber(0))); // Green
    }

    #[test]
    fn test_bet_type_is_winner_black() {
        assert!(BetType::Black.is_winner(RouletteNumber(2)));  // Black
        assert!(!BetType::Black.is_winner(RouletteNumber(1))); // Red
        assert!(!BetType::Black.is_winner(RouletteNumber(0))); // Green
    }

    #[test]
    fn test_bet_type_is_winner_dozen() {
        assert!(BetType::Dozen(1).is_winner(RouletteNumber(5)));
        assert!(!BetType::Dozen(1).is_winner(RouletteNumber(15)));
    }

    #[test]
    fn test_bet_type_is_winner_split() {
        assert!(BetType::Split(1, 2).is_winner(RouletteNumber(1)));
        assert!(BetType::Split(1, 2).is_winner(RouletteNumber(2)));
        assert!(!BetType::Split(1, 2).is_winner(RouletteNumber(3)));
    }

    #[test]
    fn test_bet_type_is_winner_street() {
        // Street 1 covers 1, 2, 3
        assert!(BetType::Street(1).is_winner(RouletteNumber(1)));
        assert!(BetType::Street(1).is_winner(RouletteNumber(2)));
        assert!(BetType::Street(1).is_winner(RouletteNumber(3)));
        assert!(!BetType::Street(1).is_winner(RouletteNumber(4)));
        assert!(!BetType::Street(1).is_winner(RouletteNumber(0)));
    }

    #[test]
    fn test_bet_type_is_winner_corner() {
        assert!(BetType::Corner(1, 2, 4, 5).is_winner(RouletteNumber(1)));
        assert!(BetType::Corner(1, 2, 4, 5).is_winner(RouletteNumber(2)));
        assert!(BetType::Corner(1, 2, 4, 5).is_winner(RouletteNumber(4)));
        assert!(BetType::Corner(1, 2, 4, 5).is_winner(RouletteNumber(5)));
        assert!(!BetType::Corner(1, 2, 4, 5).is_winner(RouletteNumber(3)));
    }

    #[test]
    fn test_bet_type_is_winner_sixline() {
        // Six Line 1 covers 1-6
        assert!(BetType::SixLine(1).is_winner(RouletteNumber(1)));
        assert!(BetType::SixLine(1).is_winner(RouletteNumber(6)));
        assert!(!BetType::SixLine(1).is_winner(RouletteNumber(7)));
        assert!(!BetType::SixLine(1).is_winner(RouletteNumber(0)));
    }

    #[test]
    fn test_bet_type_is_winner_odd_even() {
        assert!(BetType::Odd.is_winner(RouletteNumber(1)));
        assert!(BetType::Odd.is_winner(RouletteNumber(3)));
        assert!(!BetType::Odd.is_winner(RouletteNumber(2)));
        assert!(!BetType::Odd.is_winner(RouletteNumber(0)));

        assert!(BetType::Even.is_winner(RouletteNumber(2)));
        assert!(BetType::Even.is_winner(RouletteNumber(4)));
        assert!(!BetType::Even.is_winner(RouletteNumber(1)));
        assert!(!BetType::Even.is_winner(RouletteNumber(0)));
    }

    #[test]
    fn test_bet_type_is_winner_low_high() {
        assert!(BetType::Low.is_winner(RouletteNumber(1)));
        assert!(BetType::Low.is_winner(RouletteNumber(18)));
        assert!(!BetType::Low.is_winner(RouletteNumber(19)));
        assert!(!BetType::Low.is_winner(RouletteNumber(0)));

        assert!(BetType::High.is_winner(RouletteNumber(19)));
        assert!(BetType::High.is_winner(RouletteNumber(36)));
        assert!(!BetType::High.is_winner(RouletteNumber(18)));
        assert!(!BetType::High.is_winner(RouletteNumber(0)));
    }

    #[test]
    fn test_bet_type_is_winner_column() {
        // Column 1: 1,4,7... Column 2: 2,5,8... Column 3: 3,6,9...
        assert!(BetType::Column(1).is_winner(RouletteNumber(1)));
        assert!(BetType::Column(1).is_winner(RouletteNumber(4)));
        assert!(!BetType::Column(1).is_winner(RouletteNumber(2)));

        assert!(BetType::Column(2).is_winner(RouletteNumber(2)));
        assert!(BetType::Column(2).is_winner(RouletteNumber(5)));

        assert!(BetType::Column(3).is_winner(RouletteNumber(3)));
        assert!(BetType::Column(3).is_winner(RouletteNumber(6)));
    }

    // FairnessProof tests
    #[test]
    fn test_fairness_proof_generate_and_verify() {
        let proof = FairnessProof::generate("server_seed_123", "client_seed_456", 1);
        assert!(!proof.server_seed.is_empty());
        assert!(!proof.client_seed.is_empty());
        assert!(!proof.combined_hash.is_empty());
        assert!(proof.result <= 36);
        assert!(proof.verify());
    }

    #[test]
    fn test_fairness_proof_tampered_hash_fails() {
        let mut proof = FairnessProof::generate("server", "client", 1);
        proof.combined_hash = "tampered_hash".to_string();
        assert!(!proof.verify());
    }

    #[test]
    fn test_fairness_proof_tampered_result_fails() {
        let mut proof = FairnessProof::generate("server", "client", 1);
        proof.result = (proof.result + 1) % 37;
        assert!(!proof.verify());
    }

    #[test]
    fn test_fairness_proof_hash_seed() {
        let hash = FairnessProof::hash_seed("test");
        assert_eq!(hash.len(), 64); // SHA256 produces 64 hex chars
    }

    #[test]
    fn test_fairness_proof_consistent() {
        // Same inputs should produce same outputs
        let proof1 = FairnessProof::generate("seed1", "seed2", 42);
        let proof2 = FairnessProof::generate("seed1", "seed2", 42);
        assert_eq!(proof1.result, proof2.result);
        assert_eq!(proof1.combined_hash, proof2.combined_hash);
    }
}
