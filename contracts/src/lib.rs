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
    /// Place a bet (bet_type: 0=Red, 1=Black, 2=Odd, 3=Even, 4=Low, 5=High, 6-8=Dozen, 9-11=Column, 12-48=Straight)
    PlaceBet { bet_type: u8, amount: Amount },
    /// Start the spin (close betting, lock bets)
    StartSpin,
    /// Execute the spin with randomness
    ExecuteSpin { client_seed: String },
    /// Open table for new round
    OpenNewRound,
    /// Clear all current bets
    ClearBets,
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
    /// Payout notification
    Payout {
        player: Owner,
        spin_id: u64,
        amount: Amount,
    },
}

/// Arguments for instantiating the application
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct InstantiationArgument {
    /// House edge in basis points (270 = 2.7%)
    pub house_edge_bps: u16,
}
