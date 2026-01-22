#!/usr/bin/env bash
#
# MicroRoulette - Development Environment Setup Script
# =====================================================
# This script sets up and starts the MicroRoulette development environment.
# It installs dependencies, builds contracts, deploys to Conway testnet,
# and starts the frontend.
#
# Usage: ./init.sh [--build-only] [--no-deploy]
#

set -eu

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Configuration
LINERA_FAUCET_URL="${LINERA_FAUCET_URL:-https://faucet.testnet-conway.linera.net}"
FRONTEND_PORT="${FRONTEND_PORT:-8080}"
API_PORT="${API_PORT:-8081}"

# Parse arguments
BUILD_ONLY=false
NO_DEPLOY=false
for arg in "$@"; do
    case $arg in
        --build-only)
            BUILD_ONLY=true
            ;;
        --no-deploy)
            NO_DEPLOY=true
            ;;
    esac
done

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_command() {
    if ! command -v "$1" &> /dev/null; then
        log_error "$1 is required but not installed."
        return 1
    fi
}

# Banner
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}║   🎰 MicroRoulette - Provably Fair On-Chain Roulette      ║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 1: Check prerequisites
log_info "Checking prerequisites..."

MISSING_DEPS=()

if ! check_command "rustc"; then
    MISSING_DEPS+=("rust")
fi

if ! check_command "cargo"; then
    MISSING_DEPS+=("cargo")
fi

if ! check_command "node"; then
    MISSING_DEPS+=("nodejs")
fi

if ! check_command "npm"; then
    MISSING_DEPS+=("npm")
fi

if [ ${#MISSING_DEPS[@]} -ne 0 ]; then
    log_error "Missing dependencies: ${MISSING_DEPS[*]}"
    echo ""
    echo "Please install the missing dependencies:"
    echo "  - Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  - Node.js: https://nodejs.org/"
    echo ""
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | grep -oE '1\.[0-9]+\.[0-9]+')
log_info "Rust version: $RUST_VERSION"
if [[ "$RUST_VERSION" != "1.86.0" ]]; then
    log_warn "Expected Rust 1.86.0, found $RUST_VERSION"
    log_info "The rust-toolchain.toml will enforce the correct version for builds"
fi

# Check for wasm32-unknown-unknown target
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    log_info "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

log_success "Prerequisites check complete"

# Step 2: Create project structure if needed
log_info "Setting up project structure..."

mkdir -p contracts/src
mkdir -p frontend/src/components
mkdir -p frontend/src/composables
mkdir -p frontend/src/utils

log_success "Project structure ready"

# Step 3: Install Linera CLI if not present
if ! command -v linera &> /dev/null; then
    log_info "Installing Linera CLI..."
    cargo install linera-service@0.15.8 --locked 2>&1 || {
        log_warn "Could not install linera-service, you may need to install it manually"
    }
fi

# Step 4: Build contracts
log_info "Building smart contracts..."
if [ -f "contracts/Cargo.toml" ]; then
    cd contracts

    # Ensure we use the correct Rust version
    export RUSTUP_TOOLCHAIN=1.86.0

    cargo build --release --target wasm32-unknown-unknown 2>&1 || {
        log_error "Contract build failed!"
        log_info "Please check the contracts/ directory for errors"
        exit 1
    }

    cd ..
    log_success "Contracts built successfully"

    # Verify WASM files exist
    if [ -f "contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm" ]; then
        log_success "Contract WASM: contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm"
    fi
    if [ -f "contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm" ]; then
        log_success "Service WASM: contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm"
    fi
else
    log_warn "contracts/Cargo.toml not found, skipping contract build"
    log_info "You need to create the contract files first"
fi

if [ "$BUILD_ONLY" = true ]; then
    log_success "Build completed (--build-only mode)"
    exit 0
fi

# Step 5: Deploy to Conway testnet
if [ "$NO_DEPLOY" = false ]; then
    log_info "Deploying to Conway testnet..."

    # Initialize wallet
    log_info "Initializing Linera wallet..."
    linera wallet init --faucet="$LINERA_FAUCET_URL" 2>&1 || {
        log_warn "Wallet may already be initialized"
    }

    # Request a chain
    log_info "Requesting chain from faucet..."
    linera wallet request-chain --faucet="$LINERA_FAUCET_URL" 2>&1 || {
        log_warn "Chain request may have failed"
    }

    # Get Chain ID
    CHAIN_ID=$(linera wallet show 2>&1 | grep -oE '[a-f0-9]{64}' | head -1) || CHAIN_ID=""

    if [ -n "$CHAIN_ID" ]; then
        log_success "Chain ID: $CHAIN_ID"
    else
        log_warn "Could not determine Chain ID"
    fi

    # Generate server seed for provable fairness
    INITIAL_SEED=$(openssl rand -hex 32 2>/dev/null || cat /dev/urandom | head -c 32 | xxd -p)
    SEED_HASH=$(echo -n "$INITIAL_SEED" | shasum -a 256 | cut -d' ' -f1)
    log_info "Initial server seed hash: ${SEED_HASH:0:16}..."

    # Store seed securely
    echo "$INITIAL_SEED" > .server_seed
    chmod 600 .server_seed 2>/dev/null || true

    # Deploy the application
    if [ -f "contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm" ] && \
       [ -f "contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm" ]; then

        log_info "Publishing and creating application..."
        APP_ID=$(linera publish-and-create \
            contracts/target/wasm32-unknown-unknown/release/micro_roulette_contract.wasm \
            contracts/target/wasm32-unknown-unknown/release/micro_roulette_service.wasm \
            --json-argument "{
                \"house_edge_bps\": 270,
                \"min_bet\": \"1000000\",
                \"max_bet\": \"100000000\",
                \"initial_server_seed_hash\": \"$SEED_HASH\"
            }" 2>&1 | grep -oE '[a-f0-9]{64}' | tail -1) || APP_ID=""

        if [ -n "$APP_ID" ]; then
            log_success "Application ID: $APP_ID"
            echo "APP_ID=$APP_ID" > .deployment
            echo "CHAIN_ID=$CHAIN_ID" >> .deployment
            echo "SEED_HASH=$SEED_HASH" >> .deployment
        else
            log_warn "Could not deploy application (contracts may need to be created first)"
        fi
    else
        log_warn "WASM files not found, skipping deployment"
    fi
fi

# Step 6: Install frontend dependencies
log_info "Installing frontend dependencies..."
if [ -f "frontend/package.json" ]; then
    cd frontend
    npm install 2>&1
    cd ..
    log_success "Frontend dependencies installed"
else
    log_warn "frontend/package.json not found, skipping npm install"
    log_info "You need to create the frontend files first"
fi

# Step 7: Create .env file for frontend
if [ -f ".deployment" ]; then
    source .deployment

    cat > frontend/.env << EOF
VITE_LINERA_FAUCET_URL=$LINERA_FAUCET_URL
VITE_APP_ID=${APP_ID:-}
VITE_CHAIN_ID=${CHAIN_ID:-}
EOF
    log_success "Frontend .env created"
fi

# Step 8: Start services
log_info "Starting services..."

# Cleanup function
cleanup() {
    log_info "Shutting down services..."
    [ -n "${LINERA_PID:-}" ] && kill $LINERA_PID 2>/dev/null || true
    [ -n "${FRONTEND_PID:-}" ] && kill $FRONTEND_PID 2>/dev/null || true
    exit 0
}
trap cleanup SIGINT SIGTERM

# Start Linera service
if command -v linera &> /dev/null; then
    log_info "Starting Linera service on port $API_PORT..."
    linera service --port $API_PORT &
    LINERA_PID=$!
    sleep 3
fi

# Start frontend
if [ -f "frontend/package.json" ]; then
    log_info "Starting frontend on port $FRONTEND_PORT..."
    cd frontend
    npm run dev -- --host 0.0.0.0 --port $FRONTEND_PORT &
    FRONTEND_PID=$!
    cd ..
    sleep 2
fi

# Print summary
echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}║            🎰 MicroRoulette is running! 🎰               ║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}╠════════════════════════════════════════════════════════════╣${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}║${NC}  Frontend:   ${BLUE}http://localhost:$FRONTEND_PORT${NC}                      ${GREEN}║${NC}"
echo -e "${GREEN}║${NC}  API:        ${BLUE}http://localhost:$API_PORT${NC}                       ${GREEN}║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
[ -n "${CHAIN_ID:-}" ] && echo -e "${GREEN}║${NC}  Chain ID:   ${YELLOW}${CHAIN_ID:0:16}...${NC}                     ${GREEN}║${NC}"
[ -n "${APP_ID:-}" ] && echo -e "${GREEN}║${NC}  App ID:     ${YELLOW}${APP_ID:0:16}...${NC}                     ${GREEN}║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}║${NC}  Network:    ${BLUE}Conway Testnet${NC}                            ${GREEN}║${NC}"
echo -e "${GREEN}║                                                            ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════════════════╝${NC}"
echo ""
echo "Press Ctrl+C to stop all services"
echo ""

# Wait for processes
wait
