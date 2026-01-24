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

/// Types of bets a player can make
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
        let multiplier = self.bet_type.payout_multiplier() as u128;
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
    /// Number of players who placed bets this spin
    pub player_count: u32,
}

// ============================================================================
// PLAYER STATS
// ============================================================================

/// Statistics for a player
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Total bets placed
    pub total_bets: u64,
    /// Total amount wagered (all time)
    pub total_wagered: Amount,
    /// Total amount won (all time)
    pub total_won: Amount,
    /// Total amount lost (all time)
    pub total_lost: Amount,
    /// Biggest single win
    pub biggest_win: Amount,
    /// Current win/loss streak (positive = wins, negative = losses)
    pub current_streak: i32,
    /// Best winning streak ever
    pub best_streak: i32,
}

impl PlayerStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Update stats after a win
    pub fn record_win(&mut self, amount: Amount) {
        self.total_won = self.total_won.saturating_add(amount);

        // Update biggest win if this is larger
        if amount > self.biggest_win {
            self.biggest_win = amount;
        }

        // Update streak
        if self.current_streak >= 0 {
            self.current_streak += 1;
        } else {
            self.current_streak = 1;
        }

        // Update best streak
        if self.current_streak > self.best_streak {
            self.best_streak = self.current_streak;
        }
    }

    /// Update stats after a loss
    pub fn record_loss(&mut self, amount: Amount) {
        self.total_lost = self.total_lost.saturating_add(amount);

        // Update streak
        if self.current_streak <= 0 {
            self.current_streak -= 1;
        } else {
            self.current_streak = -1;
        }
    }

    /// Record a bet placed
    pub fn record_bet(&mut self, amount: Amount) {
        self.total_bets += 1;
        self.total_wagered = self.total_wagered.saturating_add(amount);
    }
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
    fn test_bet_type_validation() {
        // Valid bet types
        assert!(BetType::Straight(0).is_valid());
        assert!(BetType::Straight(36).is_valid());
        assert!(!BetType::Straight(37).is_valid());

        // Split adjacency
        assert!(BetType::Split(1, 2).is_valid()); // horizontal
        assert!(BetType::Split(1, 4).is_valid()); // vertical
        assert!(!BetType::Split(1, 5).is_valid()); // diagonal
        assert!(!BetType::Split(1, 36).is_valid()); // not adjacent

        // Street validation
        assert!(BetType::Street(1).is_valid()); // valid starting number
        assert!(BetType::Street(4).is_valid()); // valid starting number
        assert!(BetType::Street(7).is_valid()); // valid starting number
        assert!(!BetType::Street(2).is_valid()); // not a street start
        assert!(!BetType::Street(35).is_valid()); // incomplete street (only 35, 36)

        // Dozens and columns
        assert!(BetType::Dozen(1).is_valid());
        assert!(BetType::Dozen(3).is_valid());
        assert!(!BetType::Dozen(4).is_valid());
        assert!(BetType::Column(1).is_valid());
        assert!(!BetType::Column(4).is_valid());
    }

    #[test]
    fn test_bet_type_payouts() {
        // Even money bets pay 1:1
        assert_eq!(BetType::Red.payout_multiplier(), 1);
        assert_eq!(BetType::Black.payout_multiplier(), 1);
        assert_eq!(BetType::Odd.payout_multiplier(), 1);
        assert_eq!(BetType::Even.payout_multiplier(), 1);
        assert_eq!(BetType::Low.payout_multiplier(), 1);
        assert_eq!(BetType::High.payout_multiplier(), 1);

        // Dozens and columns pay 2:1
        assert_eq!(BetType::Dozen(1).payout_multiplier(), 2);
        assert_eq!(BetType::Dozen(2).payout_multiplier(), 2);
        assert_eq!(BetType::Dozen(3).payout_multiplier(), 2);
        assert_eq!(BetType::Column(1).payout_multiplier(), 2);
        assert_eq!(BetType::Column(2).payout_multiplier(), 2);
        assert_eq!(BetType::Column(3).payout_multiplier(), 2);

        // Inside bets
        assert_eq!(BetType::Straight(17).payout_multiplier(), 35);
        assert_eq!(BetType::Split(1, 2).payout_multiplier(), 17);
        assert_eq!(BetType::Street(1).payout_multiplier(), 11);
        assert_eq!(BetType::Corner(1, 2, 4, 5).payout_multiplier(), 8);
        assert_eq!(BetType::SixLine(1).payout_multiplier(), 5);
    }

    #[test]
    fn test_bet_type_winners() {
        let red_17 = RouletteNumber::new(17).unwrap();
        let black_20 = RouletteNumber::new(20).unwrap();
        let zero = RouletteNumber::new(0).unwrap();

        // Red/Black (17 is black in European roulette)
        assert!(BetType::Black.is_winner(red_17));
        assert!(!BetType::Red.is_winner(red_17));
        assert!(BetType::Black.is_winner(black_20));
        assert!(!BetType::Red.is_winner(zero));
        assert!(!BetType::Black.is_winner(zero));

        // Odd/Even
        assert!(BetType::Odd.is_winner(red_17));
        assert!(!BetType::Even.is_winner(red_17));
        assert!(BetType::Even.is_winner(black_20));

        // Straight
        assert!(BetType::Straight(17).is_winner(red_17));
        assert!(!BetType::Straight(18).is_winner(red_17));
        assert!(BetType::Straight(0).is_winner(zero));

        // Split
        assert!(BetType::Split(17, 20).is_winner(red_17));
        assert!(BetType::Split(17, 20).is_winner(black_20));
        assert!(!BetType::Split(17, 20).is_winner(zero));

        // Street
        assert!(BetType::Street(1).is_winner(RouletteNumber::new(1).unwrap()));
        assert!(BetType::Street(1).is_winner(RouletteNumber::new(2).unwrap()));
        assert!(!BetType::Street(1).is_winner(RouletteNumber::new(4).unwrap()));

        // Corner
        let corner_bet = BetType::Corner(1, 2, 4, 5);
        assert!(corner_bet.is_winner(RouletteNumber::new(1).unwrap()));
        assert!(corner_bet.is_winner(RouletteNumber::new(2).unwrap()));
        assert!(corner_bet.is_winner(RouletteNumber::new(4).unwrap()));
        assert!(corner_bet.is_winner(RouletteNumber::new(5).unwrap()));
        assert!(!corner_bet.is_winner(RouletteNumber::new(3).unwrap()));

        // SixLine
        let sixline_bet = BetType::SixLine(1);
        assert!(sixline_bet.is_winner(RouletteNumber::new(1).unwrap()));
        assert!(sixline_bet.is_winner(RouletteNumber::new(6).unwrap()));
        assert!(!sixline_bet.is_winner(RouletteNumber::new(7).unwrap()));
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
