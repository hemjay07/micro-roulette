// src/service.rs
//! MicroRoulette GraphQL Service

#![cfg_attr(target_arch = "wasm32", no_main)]

use async_graphql::{EmptySubscription, Object, Schema, SimpleObject, Request, Response};
use linera_sdk::{
    linera_base_types::WithServiceAbi,
    views::View,
    Service, ServiceRuntime,
};
use micro_roulette::{RouletteAbi, TableStatus};
use micro_roulette::state::RouletteState;

linera_sdk::service!(RouletteService);

/// The MicroRoulette service
pub struct RouletteService {
    state: RouletteState,
}

impl WithServiceAbi for RouletteService {
    type Abi = RouletteAbi;
}

impl Service for RouletteService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = RouletteState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { state }
    }

    async fn handle_query(&self, request: Request) -> Response {
        // Read current state values
        let house_edge_bps = *self.state.house_edge_bps.get();
        let spin_number = *self.state.spin_number.get();
        let status = *self.state.status.get();
        let round_total = *self.state.round_total.get();
        let total_volume = *self.state.total_volume.get();
        let total_payouts = *self.state.total_payouts.get();
        let total_spins = *self.state.total_spins.get();

        // Fairness data
        let server_seed_hash = self.state.server_seed_hash.get().clone();
        let revealed_server_seed = self.state.revealed_server_seed.get().clone();
        let last_client_seed = self.state.last_client_seed.get().clone();
        let last_result = *self.state.last_result.get();

        // Convert TableStatus to string
        let status_str = match status {
            TableStatus::Open => "Open",
            TableStatus::Spinning => "Spinning",
            TableStatus::PayingOut => "PayingOut",
            TableStatus::Closed => "Closed",
        };

        let schema = Schema::build(
            QueryRoot {
                house_edge_bps,
                spin_number,
                status: status_str.to_string(),
                is_betting_open: status == TableStatus::Open,
                round_total: round_total.to_string(),
                total_volume: total_volume.to_string(),
                total_payouts: total_payouts.to_string(),
                total_spins,
                server_seed_hash,
                revealed_server_seed,
                last_client_seed,
                last_result,
            },
            MutationRoot,
            EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}

// GraphQL types
#[derive(SimpleObject)]
struct TableInfo {
    house_edge_bps: u16,
    spin_number: u64,
    status: String,
    is_betting_open: bool,
    round_total: String,
}

#[derive(SimpleObject)]
struct PlatformStats {
    total_volume: String,
    total_payouts: String,
    total_spins: u64,
}

struct QueryRoot {
    house_edge_bps: u16,
    spin_number: u64,
    status: String,
    is_betting_open: bool,
    round_total: String,
    total_volume: String,
    total_payouts: String,
    total_spins: u64,
    server_seed_hash: String,
    revealed_server_seed: String,
    last_client_seed: String,
    last_result: u8,
}

#[derive(SimpleObject)]
struct FairnessInfo {
    server_seed_hash: String,
    revealed_server_seed: String,
    last_client_seed: String,
    last_result: u8,
    can_verify: bool,
}

#[Object]
impl QueryRoot {
    /// Get table information
    async fn table_info(&self) -> TableInfo {
        TableInfo {
            house_edge_bps: self.house_edge_bps,
            spin_number: self.spin_number,
            status: self.status.clone(),
            is_betting_open: self.is_betting_open,
            round_total: self.round_total.clone(),
        }
    }

    /// Get current table status (Open, Spinning, PayingOut, Closed)
    async fn table_status(&self) -> &str {
        &self.status
    }

    /// Check if table is currently accepting bets
    async fn is_betting_open(&self) -> bool {
        self.is_betting_open
    }

    /// Get house edge in basis points
    async fn house_edge_bps(&self) -> u16 {
        self.house_edge_bps
    }

    /// Get current spin number
    async fn spin_number(&self) -> u64 {
        self.spin_number
    }

    /// Get total bets for current round
    async fn round_total(&self) -> &str {
        &self.round_total
    }

    /// Get platform statistics
    async fn platform_stats(&self) -> PlatformStats {
        PlatformStats {
            total_volume: self.total_volume.clone(),
            total_payouts: self.total_payouts.clone(),
            total_spins: self.total_spins,
        }
    }

    /// Get fairness information for verification
    async fn fairness_info(&self) -> FairnessInfo {
        FairnessInfo {
            server_seed_hash: self.server_seed_hash.clone(),
            revealed_server_seed: self.revealed_server_seed.clone(),
            last_client_seed: self.last_client_seed.clone(),
            last_result: self.last_result,
            can_verify: !self.revealed_server_seed.is_empty(),
        }
    }

    /// Get last spin result
    async fn last_result(&self) -> u8 {
        self.last_result
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Placeholder mutation
    async fn noop(&self) -> bool {
        true
    }
}
