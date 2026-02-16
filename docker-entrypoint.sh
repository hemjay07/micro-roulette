#!/usr/bin/env bash
set -eu

echo ""
echo "========================================"
echo "  MicroRoulette - Docker Startup"
echo "========================================"
echo ""

# Configuration
export LINERA_FAUCET_URL="${LINERA_FAUCET_URL:-https://faucet.testnet-conway.linera.net}"

# Initialize wallet
echo "[1/5] Initializing Linera wallet..."
linera wallet init --faucet="$LINERA_FAUCET_URL" 2>/dev/null || echo "Wallet already initialized"

# Request chain
echo "[2/5] Requesting chain from faucet..."
linera wallet request-chain --faucet="$LINERA_FAUCET_URL" 2>/dev/null || echo "Chain already exists"

# Get chain ID
CHAIN_ID=$(linera wallet show 2>/dev/null | grep -oP 'Chain ID: \K[a-f0-9]+' | head -1 || echo "")
if [ -z "$CHAIN_ID" ]; then
    CHAIN_ID=$(linera wallet show 2>/dev/null | grep -E '^[a-f0-9]{64}' | head -1 || echo "unknown")
fi
echo "Chain ID: $CHAIN_ID"

# Generate server seed if needed
if [ ! -f .server_seed ]; then
    INITIAL_SEED=$(openssl rand -hex 32)
    echo "$INITIAL_SEED" > .server_seed
    chmod 600 .server_seed
fi
SEED_HASH=$(cat .server_seed | sha256sum | cut -d' ' -f1)
echo "Seed hash: $SEED_HASH"

# Publish application using pre-built WASM
echo "[3/5] Publishing application to blockchain..."
WASM_CONTRACT="contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm"
WASM_SERVICE="contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm"

if [ ! -f "$WASM_CONTRACT" ] || [ ! -f "$WASM_SERVICE" ]; then
    echo "ERROR: WASM files not found. Build failed?"
    exit 1
fi

APP_ID=$(linera publish-and-create \
    "$WASM_CONTRACT" \
    "$WASM_SERVICE" \
    --json-argument "{
        \"house_edge_bps\": 270,
        \"min_bet\": \"1000000\",
        \"max_bet\": \"100000000\",
        \"initial_server_seed_hash\": \"$SEED_HASH\"
    }" 2>&1 | grep -oE '[a-f0-9]{64}' | tail -1 || echo "")

if [ -z "$APP_ID" ]; then
    echo "Checking for existing application..."
    APP_ID=$(linera wallet show 2>/dev/null | grep -oE 'Application.*[a-f0-9]{64}' | grep -oE '[a-f0-9]{64}' | head -1 || echo "unknown")
fi
echo "Application ID: $APP_ID"

# Start Linera GraphQL service
echo "[4/5] Starting Linera GraphQL service on port 8082..."
linera service --port 8082 &
LINERA_PID=$!
sleep 3

# Start static file server for frontend
echo "[5/5] Starting frontend on port 8080..."
cd frontend/dist
npx serve -s -l 8080 &
FRONTEND_PID=$!

echo ""
echo "========================================"
echo "  MicroRoulette is LIVE!"
echo "========================================"
echo "  Frontend:   http://localhost:8080"
echo "  GraphQL:    http://localhost:8082"
echo "  Chain ID:   $CHAIN_ID"
echo "  App ID:     $APP_ID"
echo "========================================"
echo ""

# Cleanup on exit
trap "kill $LINERA_PID $FRONTEND_PID 2>/dev/null || true" EXIT

# Wait
wait
