// src/lib.rs
//! MicroRoulette - Provably Fair On-Chain Roulette
//!
//! This module defines the Application Binary Interface (ABI) for the roulette application.

#![allow(clippy::large_enum_variant)]

pub mod state;
pub mod types;

use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{AccountOwner, Amount, ContractAbi, ServiceAbi};
use serde::{Deserialize, Serialize};

pub use types::*;

/// Type alias for player/owner addresses
pub type Owner = AccountOwner;

/// The Application Binary Interface for MicroRoulette
pub struct RouletteAbi;

impl ContractAbi for RouletteAbi {
    type Operation = Operation;
    type Response = ();
}

impl ServiceAbi for RouletteAbi {
    type Query = Request;
    type QueryResponse = Response;
}

/// Operations that can be performed on the application
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operation {
    /// Deposit funds to player account
    Deposit { amount: Amount },
    /// Withdraw funds from player account
    Withdraw { amount: Amount },
    /// Place a bet
    PlaceBet { bet_type: BetType, amount: Amount },
    /// Start the spin (close betting, lock bets)
    StartSpin,
    /// Execute the spin with randomness
    ExecuteSpin { client_seed: String },
    /// Resolve spin and reveal server seed for verification
    ResolveSpin { server_seed: String },
    /// Open table for new round
    OpenNewRound,
    /// Clear all current bets
    ClearBets,

    // ========== Admin Operations ==========

    /// Update game settings (admin only)
    UpdateSettings {
        house_edge_bps: Option<u16>,
        min_bet: Option<Amount>,
        max_bet: Option<Amount>,
    },
    /// Pause/unpause the contract (admin only)
    SetPaused { paused: bool },
    /// Withdraw from treasury (admin only)
    WithdrawTreasury { amount: Amount },
    /// Set the next server seed hash (admin only)
    SetServerSeedHash { hash: String },
    /// Close the table permanently (admin only)
    CloseTable,
}

/// Messages sent between chains
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    /// Notify of spin result with details
    SpinResult {
        spin_id: u64,
        result: u8,
        winnings: Amount,
        total_bet: Amount,
    },
    /// Confirm bet was placed
    BetConfirmed {
        player: Owner,
        bet_type: BetType,
        amount: Amount,
        round_total: Amount,
    },
    /// Bet was rejected
    BetRejected {
        player: Owner,
        reason: String,
        attempted_amount: Amount,
    },
    /// Payout notification
    Payout {
        player: Owner,
        spin_id: u64,
        amount: Amount,
    },
    /// Refund notification
    Refund {
        player: Owner,
        reason: String,
        amount: Amount,
    },
}

/// Arguments for instantiating the application
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InstantiationArgument {
    /// House edge in basis points (270 = 2.7%)
    pub house_edge_bps: u16,
}
