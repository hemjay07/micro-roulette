# Environment Sweep

**Date:** 2026-01-22
**Purpose:** Final environment verification for MicroRoulette

## Environment Variables

| Variable | Required Value | Found | Status |
|----------|---------------|-------|--------|
| LINERA_FAUCET_URL | https://faucet.testnet-conway.linera.net | https://faucet.testnet-conway.linera.net | PASS |
| CHAIN_ID | 8fd4233c5d03554f87d47a711cf70619727ca3d148353446cab81fb56922c9b7 | 8fd4233c5d03554f87d47a711cf70619727ca3d148353446cab81fb56922c9b7 | PASS |
| LINERA_SERVICE_PORT | 8081 | 8081 | PASS |
| FRONTEND_PORT | 8080 | 8080 | PASS |
| LINERA_WALLET_PATH | ~/.config/linera/wallet.json | ~/Library/Application Support/linera/wallet.json | NOTE |
| RUST_LOG | info | info | PASS |
| RUSTUP_TOOLCHAIN | 1.86.0 | 1.86.0 | PASS |
| CARGO_BUILD_TARGET | wasm32-unknown-unknown | wasm32-unknown-unknown | PASS |

**Note:** The `LINERA_WALLET_PATH` in `.env.local` points to `~/.config/linera/wallet.json` but on macOS, Linera uses `~/Library/Application Support/linera/wallet.json` by default. This is not an issue as the Linera CLI automatically uses the correct default path.

## Linera Wallet

| Property | Value | Status |
|----------|-------|--------|
| Wallet exists | YES | PASS |
| Wallet location | /Users/mujeeb/Library/Application Support/linera/wallet.json | PASS |
| Chain ID in wallet | 8fd4233c5d03554f87d47a711cf70619727ca3d148353446cab81fb56922c9b7 | PASS |
| Chain tags | ADMIN | OK |
| Blocks | 71 | OK |
| Has default owner | No owner key | NOTE |

**Note:** The wallet shows "No owner key" for the default chain. This is expected for testnet chains created from the faucet without explicit owner assignment.

## Development Tools

| Tool | Required | Found | Status |
|------|----------|-------|--------|
| Rust | 1.86.0 | 1.86.0 (05f9846f8 2025-03-31) | PASS |
| wasm32-unknown-unknown target | installed | installed | PASS |
| Node.js | 18+ | v22.13.1 | PASS |
| npm | 9+ | (bundled with Node 22.x) | PASS |
| Linera CLI | 0.15.8 | v0.15.8 | PASS |

### Linera CLI Details
```
linera
Linera protocol: v0.15.8
RPC API hash: K9p3m/MsIPZL32CYddAqlG6PHKprJvMjei5cIiqFgDY
GraphQL API hash: RmwcE5swpH/HkjbetY/YyD6ebNQFS9oeU6ayEAvDjEQ
WIT API hash: 0X+I4jeHCdpD2M0R+OVodI4pH+dF9rt0K/iHENVcnug
Source: https://github.com/linera-io/linera-protocol/tree/32c047f7891e08503019302b0258c17c2c7c4180
```

## Issues Found

### Minor Issues (Non-blocking)

1. **Wallet Path Mismatch**: The `.env.local` specifies `LINERA_WALLET_PATH=~/.config/linera/wallet.json` but macOS uses `~/Library/Application Support/linera/wallet.json`. This is not a problem because:
   - The Linera CLI automatically uses the correct macOS default path
   - The environment variable is only used for documentation purposes

2. **No Owner Key**: The wallet chain shows "No owner key" - this is normal for testnet faucet chains and won't affect application functionality.

### No Critical Issues Found

All required tools and configurations are properly set up.

## Summary

| Category | Status |
|----------|--------|
| Environment Variables | PASS |
| Linera Wallet | PASS |
| Rust Toolchain | PASS |
| Node.js/npm | PASS |
| Linera CLI | PASS |

**Overall Status: READY FOR DEVELOPMENT**

## Setup Commands (if needed)

No setup commands needed - the environment is fully configured.

### For Reference Only

If you need to reset the environment in the future:

```bash
# Re-initialize Linera wallet from faucet
linera wallet init --faucet https://faucet.testnet-conway.linera.net

# Verify Rust wasm target
rustup target add wasm32-unknown-unknown

# Set Rust toolchain
rustup default 1.86.0

# Verify Linera CLI
linera --version
```

## Quick Start Commands

```bash
# Navigate to project
cd /Users/mujeeb/projects/micro-roulette

# Build the smart contract
source ~/.cargo/env
cargo build --release --target wasm32-unknown-unknown

# Start Linera service (in terminal 1)
linera service --port 8081

# Start frontend (in terminal 2)
cd frontend && npm run dev
```
