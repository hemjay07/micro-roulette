// src/state.rs
//! State management using Linera Views

use crate::types::{TableStatus, PlayerBets, SpinResult, PlayerStats};
use linera_sdk::linera_base_types::{AccountOwner, Amount};
use linera_sdk::views::{MapView, QueueView, RegisterView, RootView, ViewStorageContext};

/// Type alias for player/owner addresses
pub type Owner = AccountOwner;

/// Main application state for MicroRoulette
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct RouletteState {
    /// House edge in basis points (270 = 2.7%)
    pub house_edge_bps: RegisterView<u16>,

    /// Current spin number
    pub spin_number: RegisterView<u64>,

    /// Player balances
    pub balances: MapView<Owner, Amount>,

    /// Current table status (Open, Spinning, PayingOut, Closed)
    pub status: RegisterView<TableStatus>,

    /// Current bets by player
    pub current_bets: MapView<Owner, PlayerBets>,

    /// Total amount bet this round
    pub round_total: RegisterView<Amount>,

    /// Recent spin results (last 20)
    pub spin_history: QueueView<SpinResult>,

    /// Total volume (all time)
    pub total_volume: RegisterView<Amount>,

    /// Total payouts (all time)
    pub total_payouts: RegisterView<Amount>,

    /// Total spins (all time)
    pub total_spins: RegisterView<u64>,

    // ========== Provable Fairness ==========

    /// Committed server seed hash (shown before spin)
    pub server_seed_hash: RegisterView<String>,

    /// Revealed server seed (shown after spin for verification)
    pub revealed_server_seed: RegisterView<String>,

    /// Client seed used in last spin
    pub last_client_seed: RegisterView<String>,

    /// Last spin result number (0-36)
    pub last_result: RegisterView<u8>,

    // ========== Player Statistics ==========

    /// Per-player statistics
    pub player_stats: MapView<Owner, PlayerStats>,
}
