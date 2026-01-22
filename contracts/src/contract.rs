// src/contract.rs
//! MicroRoulette Contract Implementation

#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{
    linera_base_types::{Amount, WithContractAbi},
    views::{View, RootView},
    Contract, ContractRuntime,
};
use micro_roulette::{
    InstantiationArgument, Message, Operation, RouletteAbi,
    Bet, BetType, FairnessProof, PlayerBets, PlayerStats, RouletteNumber, SpinResult, TableStatus,
};
use micro_roulette::state::{Owner, RouletteState};

// Contract entry point
linera_sdk::contract!(RouletteContract);

/// The MicroRoulette contract
pub struct RouletteContract {
    state: RouletteState,
    runtime: ContractRuntime<Self>,
}

impl WithContractAbi for RouletteContract {
    type Abi = RouletteAbi;
}

impl Contract for RouletteContract {
    type Message = Message;
    type InstantiationArgument = InstantiationArgument;
    type Parameters = ();
    type EventValue = ();

    /// Load the contract state from storage
    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = RouletteState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self { state, runtime }
    }

    /// Store the contract state
    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }

    /// Initialize the application
    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        self.state.house_edge_bps.set(argument.house_edge_bps);
        self.state.spin_number.set(1);
        self.state.status.set(TableStatus::Open);
        self.state.round_total.set(Amount::ZERO);
        self.state.total_volume.set(Amount::ZERO);
        self.state.total_payouts.set(Amount::ZERO);
        self.state.total_spins.set(0);
        log::info!("MicroRoulette initialized with house edge: {} bps", argument.house_edge_bps);
    }

    /// Execute an operation
    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            Operation::Deposit { amount } => {
                self.deposit(amount).await;
            }
            Operation::Withdraw { amount } => {
                self.withdraw(amount).await;
            }
            Operation::PlaceBet { bet_type, amount } => {
                self.place_bet(bet_type, amount).await;
            }
            Operation::StartSpin => {
                self.start_spin().await;
            }
            Operation::ExecuteSpin { client_seed } => {
                self.execute_spin(client_seed).await;
            }
            Operation::ResolveSpin { server_seed } => {
                self.resolve_spin(server_seed).await;
            }
            Operation::OpenNewRound => {
                self.open_new_round().await;
            }
            Operation::ClearBets => {
                self.clear_bets().await;
            }
        }
    }

    /// Handle incoming messages
    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::SpinResult { spin_id, result, winnings, total_bet } => {
                // This message is received by a player's chain to notify them of the spin result
                log::info!(
                    "SpinResult received: spin_id={}, result={}, winnings={:?}, total_bet={:?}",
                    spin_id, result, winnings, total_bet
                );

                // The message serves as a notification - winnings are already credited
                // by the main contract during execute_spin
            }
            Message::BetConfirmed { player, bet_type, amount, round_total } => {
                // Acknowledge bet was placed successfully
                log::info!(
                    "BetConfirmed: player={:?}, bet_type={}, amount={:?}, round_total={:?}",
                    player, bet_type, amount, round_total
                );

                // This is a confirmation message - the bet is already in state
                // from the original PlaceBet operation
            }
            Message::BetRejected { player, reason, attempted_amount } => {
                // Bet was rejected - notify player
                log::warn!(
                    "BetRejected: player={:?}, reason={}, amount={:?}",
                    player, reason, attempted_amount
                );

                // The bet was rejected, no state change needed
                // The frontend should display the rejection reason to the user
            }
            Message::Payout { player, spin_id, amount } => {
                // Credit winnings to the player
                self.credit(&player, amount).await;
                log::info!("Payout: player={:?}, spin_id={}, amount={:?}", player, spin_id, amount);
            }
            Message::Refund { player, reason, amount } => {
                // Refund bet amount to the player
                self.credit(&player, amount).await;
                log::info!("Refund: player={:?}, reason={}, amount={:?}", player, reason, amount);
            }
        }
    }
}

// ============================================================================
// IMPLEMENTATION METHODS
// ============================================================================

impl RouletteContract {
    /// Get authenticated signer or panic
    fn signer(&self) -> Owner {
        self.runtime
            .authenticated_signer()
            .expect("Operation must be authenticated")
    }

    /// Get player balance
    async fn get_balance(&self, player: &Owner) -> Amount {
        self.state.balances.get(player).await.ok().flatten().unwrap_or_default()
    }

    /// Credit tokens to a player
    async fn credit(&mut self, player: &Owner, amount: Amount) {
        let current = self.get_balance(player).await;
        let _ = self.state.balances.insert(player, current.saturating_add(amount));
    }

    /// Debit tokens from a player, returns true if successful
    async fn debit(&mut self, player: &Owner, amount: Amount) -> bool {
        let current = self.get_balance(player).await;
        if current >= amount {
            let _ = self.state.balances.insert(player, current.saturating_sub(amount));
            true
        } else {
            false
        }
    }

    // ========== PLAYER METHODS ==========

    async fn deposit(&mut self, amount: Amount) {
        let player = self.signer();
        self.credit(&player, amount).await;
        log::info!("Player {:?} deposited {:?}", player, amount);
    }

    async fn withdraw(&mut self, amount: Amount) {
        let player = self.signer();
        let success = self.debit(&player, amount).await;
        assert!(success, "Insufficient balance");
        log::info!("Player {:?} withdrew {:?}", player, amount);
    }

    async fn place_bet(&mut self, bet_type: u8, amount: Amount) {
        // Check table is accepting bets
        let status = *self.state.status.get();
        assert!(status == TableStatus::Open, "Table is not accepting bets (status: {:?})", status);

        let player = self.signer();

        // Validate bet type
        let bt = BetType::new(bet_type).expect("Invalid bet type");
        assert!(bt.is_valid(), "Invalid bet type");

        // Check player has sufficient balance
        let balance = self.get_balance(&player).await;
        assert!(balance >= amount, "Insufficient balance");

        // Create the bet
        let bet = Bet::new(bet_type, amount).expect("Failed to create bet");

        // Debit from player balance
        self.debit(&player, amount).await;

        // Get existing bets or create new
        let mut player_bets = self.state.current_bets
            .get(&player)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(PlayerBets::new);

        // Add bet
        player_bets.add_bet(bet);

        // Update state
        let _ = self.state.current_bets.insert(&player, player_bets);

        // Update round total
        let current_round_total = *self.state.round_total.get();
        self.state.round_total.set(current_round_total.saturating_add(amount));

        // Update player stats
        let mut stats = self.state.player_stats
            .get(&player)
            .await
            .ok()
            .flatten()
            .unwrap_or_else(PlayerStats::new);
        stats.record_bet(amount);
        let _ = self.state.player_stats.insert(&player, stats);

        log::info!("Player {:?} placed bet: type={}, amount={:?}", player, bet_type, amount);
    }

    async fn clear_bets(&mut self) {
        let player = self.signer();
        let status = *self.state.status.get();
        assert!(status == TableStatus::Open, "Cannot clear bets now");

        if let Ok(Some(player_bets)) = self.state.current_bets.get(&player).await {
            // Refund the bets
            self.credit(&player, player_bets.total_amount).await;

            // Update round total
            let current_round_total = *self.state.round_total.get();
            self.state.round_total.set(current_round_total.saturating_sub(player_bets.total_amount));

            // Remove bets
            let _ = self.state.current_bets.remove(&player);

            log::info!("Player {:?} cleared bets", player);
        }
    }

    // ========== GAME METHODS ==========

    /// Start the spin - locks bets and changes status to Spinning
    async fn start_spin(&mut self) {
        let status = *self.state.status.get();
        assert!(status == TableStatus::Open, "Table must be Open to start spin (current: {:?})", status);

        // Lock bets by changing status to Spinning
        self.state.status.set(TableStatus::Spinning);

        log::info!("Spin started, bets locked. Status: Spinning");
    }

    /// Execute the spin with provided randomness
    async fn execute_spin(&mut self, client_seed: String) {
        let status = *self.state.status.get();
        assert!(status == TableStatus::Spinning, "Table must be Spinning to execute (current: {:?})", status);

        let spin_number = *self.state.spin_number.get();

        // Generate server seed (in production, this would be pre-committed)
        let server_seed = format!("server_seed_{}", spin_number);

        // Store seeds for later verification
        self.state.last_client_seed.set(client_seed.clone());

        // Generate result using provable fairness
        let proof = FairnessProof::generate(&server_seed, &client_seed, spin_number);
        let result = RouletteNumber::new(proof.result).unwrap_or_default();

        // Store result for verification
        self.state.last_result.set(result.0);

        log::info!("Spin {} result: {} ({})", spin_number, result.0, result.color());

        // Calculate payouts for each player
        let mut total_payout = Amount::ZERO;

        // Get all players who bet
        let player_keys: Vec<Owner> = self.state.current_bets
            .indices()
            .await
            .map(|iter| iter.collect())
            .unwrap_or_default();

        // Count players before iterating
        let player_count = player_keys.len() as u32;

        for player in player_keys {
            if let Ok(Some(player_bets)) = self.state.current_bets.get(&player).await {
                let winnings = player_bets.calculate_winnings(result);

                // Update player stats
                let mut stats = self.state.player_stats
                    .get(&player)
                    .await
                    .ok()
                    .flatten()
                    .unwrap_or_else(PlayerStats::new);

                if winnings > Amount::ZERO {
                    // Credit winnings to player
                    self.credit(&player, winnings).await;
                    total_payout = total_payout.saturating_add(winnings);

                    // Record win in stats
                    stats.record_win(winnings);

                    log::info!("Player {:?} won {:?}", player, winnings);
                } else {
                    // Record loss in stats
                    stats.record_loss();
                }

                // Save updated stats
                let _ = self.state.player_stats.insert(&player, stats);
            }
        }

        // Create spin result record
        let spin_result = SpinResult {
            spin_id: spin_number,
            result,
            seed_hash: proof.combined_hash.clone(),
            total_bets: *self.state.round_total.get(),
            total_payout,
            player_count,
        };

        // Add to history (keep last 20)
        self.state.spin_history.push_back(spin_result).await.ok();

        // Trim history to 20 entries
        while let Ok(count) = self.state.spin_history.count().await {
            if count <= 20 {
                break;
            }
            self.state.spin_history.delete_front().await.ok();
        }

        // Update global statistics
        let total_volume = *self.state.total_volume.get();
        self.state.total_volume.set(total_volume.saturating_add(*self.state.round_total.get()));

        let total_payouts = *self.state.total_payouts.get();
        self.state.total_payouts.set(total_payouts.saturating_add(total_payout));

        let total_spins = *self.state.total_spins.get();
        self.state.total_spins.set(total_spins + 1);

        // Move to payout state
        self.state.status.set(TableStatus::PayingOut);

        // Increment spin number
        self.state.spin_number.set(spin_number + 1);

        log::info!("Spin complete. Total bets: {:?}, Total payout: {:?}",
            self.state.round_total.get(), total_payout);
    }

    /// Resolve spin by revealing the server seed for verification
    async fn resolve_spin(&mut self, server_seed: String) {
        let status = *self.state.status.get();
        assert!(
            status == TableStatus::PayingOut,
            "Table must be in PayingOut state to resolve (current: {:?})",
            status
        );

        // Store revealed server seed for verification
        self.state.revealed_server_seed.set(server_seed.clone());

        // Verify the result matches (optional verification)
        let client_seed = self.state.last_client_seed.get().clone();
        let spin_number = self.state.spin_number.get().saturating_sub(1); // Previous spin
        let expected_result = *self.state.last_result.get();

        let proof = FairnessProof::generate(&server_seed, &client_seed, spin_number);

        if proof.result == expected_result {
            log::info!(
                "ResolveSpin: Server seed revealed and verified! Seed: {}..., Result: {}",
                &server_seed[..8.min(server_seed.len())],
                expected_result
            );
        } else {
            log::warn!(
                "ResolveSpin: Verification mismatch! Expected: {}, Got: {}",
                expected_result,
                proof.result
            );
        }

        // Transition to open for next round (or let OpenNewRound handle this)
        log::info!("Spin resolved. Ready for new round.");
    }

    /// Open table for a new round
    async fn open_new_round(&mut self) {
        // Clear current bets
        let player_keys: Vec<Owner> = self.state.current_bets
            .indices()
            .await
            .map(|iter| iter.collect())
            .unwrap_or_default();

        for player in player_keys {
            let _ = self.state.current_bets.remove(&player);
        }

        // Reset round total
        self.state.round_total.set(Amount::ZERO);

        // Set status to open
        self.state.status.set(TableStatus::Open);

        log::info!("New round opened, spin #{}", self.state.spin_number.get());
    }
}
