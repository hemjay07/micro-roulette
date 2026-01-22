#!/usr/bin/env bash
set -eu

echo "🎰 Starting MicroRoulette deployment..."

# Ensure we're using the right Rust version
export RUSTUP_TOOLCHAIN=1.86.0

# Setup NVM if available
export NVM_DIR="$HOME/.nvm"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

# Linera configuration
export LINERA_FAUCET_URL=https://faucet.testnet-conway.linera.net

# Initialize wallet
echo "🔑 Initializing wallet..."
linera wallet init --faucet="$LINERA_FAUCET_URL" || true

# Request chain
echo "⛓️ Requesting chain..."
linera wallet request-chain --faucet="$LINERA_FAUCET_URL" || true

# Build contracts
echo "🔨 Building contracts..."
cd contracts
cargo build --release --target wasm32-unknown-unknown
cd ..

# Get chain ID
CHAIN_ID=$(linera wallet show | grep -oP 'Chain ID: \K[a-f0-9]+' | head -1)
echo "Chain ID: $CHAIN_ID"

# Generate initial server seed
INITIAL_SEED=$(openssl rand -hex 32)
SEED_HASH=$(echo -n "$INITIAL_SEED" | sha256sum | cut -d' ' -f1)
echo "Initial seed hash: $SEED_HASH"

# Publish application
echo "📦 Publishing application..."
APP_ID=$(linera publish-and-create \
    contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm \
    contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm \
    --json-argument "{
        \"house_edge_bps\": 270,
        \"min_bet\": \"1000000\",
        \"max_bet\": \"100000000\",
        \"initial_server_seed_hash\": \"$SEED_HASH\"
    }" 2>&1 | grep -oP 'Application ID: \K[a-f0-9]+' || echo "")

if [ -z "$APP_ID" ]; then
    echo "⚠️ Could not parse Application ID, checking wallet..."
    APP_ID=$(linera wallet show | grep -oP 'Application: \K[a-f0-9]+' | head -1)
fi

echo "✅ Application ID: $APP_ID"

# Store server seed securely
echo "$INITIAL_SEED" > .server_seed
chmod 600 .server_seed

# Start Linera service
echo "🌐 Starting Linera service on port 8081..."
linera service --port 8081 &
LINERA_PID=$!

# Wait for service to start
sleep 5

# Setup frontend
echo "🎨 Setting up frontend..."
cd frontend
echo "VITE_LINERA_FAUCET_URL=$LINERA_FAUCET_URL" > .env
echo "VITE_APP_ID=$APP_ID" >> .env
echo "VITE_CHAIN_ID=$CHAIN_ID" >> .env

npm install
echo "🖥️ Starting frontend on port 8080..."
npm run dev -- --host 0.0.0.0 --port 8080 &
FRONTEND_PID=$!

echo ""
echo "=========================================="
echo "🎰 MicroRoulette is running!"
echo "=========================================="
echo "Frontend:   http://localhost:8080"
echo "API:        http://localhost:8081"
echo "Chain ID:   $CHAIN_ID"
echo "App ID:     $APP_ID"
echo "=========================================="
echo ""

# Cleanup on exit
trap "kill $LINERA_PID $FRONTEND_PID 2>/dev/null" EXIT

# Wait for processes
wait
