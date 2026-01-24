#!/usr/bin/env bash
set -e

cd /Users/mujeeb/projects/micro-roulette
export RUSTUP_TOOLCHAIN=1.86.0

# Generate server seed
INITIAL_SEED=$(openssl rand -hex 32)
SEED_HASH=$(echo -n "$INITIAL_SEED" | shasum -a 256 | cut -d' ' -f1)

echo "📦 Publishing MicroRoulette to blockchain..."
echo "Seed hash: $SEED_HASH"

# Publish application
/Users/mujeeb/.cargo/bin/linera publish-and-create \
    contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm \
    contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm \
    --json-argument "{\"house_edge_bps\": 270, \"min_bet\": \"1000000\", \"max_bet\": \"100000000\", \"initial_server_seed_hash\": \"$SEED_HASH\"}"

# Save seed
echo "$INITIAL_SEED" > .server_seed
chmod 600 .server_seed

echo "✅ Deployment complete!"
echo "Server seed saved to .server_seed"
