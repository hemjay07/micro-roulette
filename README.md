# MicroRoulette

🎰 **Every Spin On-Chain - Provably Fair Roulette on Linera**

MicroRoulette is a classic European roulette game running entirely on Linera's microchain architecture. Every spin is provably fair, every bet is recorded on-chain, and payouts settle in under one second.

## Features

- 🎯 **Full European Roulette** - 37 numbers (0-36), all standard bet types
- ⚡ **Sub-second Settlement** - Leverages Linera's microchain speed
- 🔒 **Provably Fair** - Verifiable randomness using SHA256 seeds
- 📊 **Live Statistics** - Hot/cold numbers, spin history
- 💰 **Multiple Bet Types** - Straight, split, red/black, dozens, columns
- 🔗 **Fully On-Chain** - Running on Linera Conway testnet
- 🌐 **linera-web Integration** - Direct browser-to-blockchain connection

## Quick Start

### Using Docker (Recommended)

```bash
# Clone the repository
git clone <repository-url>
cd micro-roulette

# Start everything with Docker
docker compose up --build
```

Then open http://localhost:8080

### Manual Setup

```bash
# Make init script executable
chmod +x init.sh

# Run the init script
./init.sh
```

## Requirements

- **Rust 1.86.0** (pinned via rust-toolchain.toml - CRITICAL)
- **Node.js 20+**
- **Linera CLI** (installed automatically by init.sh)

## Project Structure

```
micro-roulette/
├── rust-toolchain.toml      # Pins Rust to 1.86.0 (prevents WASM opcode errors)
├── contracts/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # ABI definitions
│       ├── contract.rs      # Contract implementation
│       ├── service.rs       # GraphQL service
│       ├── types.rs         # Core types (Bet, SpinResult, etc.)
│       ├── state.rs         # Linera Views state
│       └── operations.rs    # Operations and Messages
├── frontend/
│   ├── package.json
│   ├── vite.config.js       # WASM configuration
│   └── src/
│       ├── App.vue
│       ├── components/      # Vue components
│       ├── composables/     # Vue composables
│       └── utils/           # Helper utilities
├── init.sh                  # Development setup script
├── Dockerfile
└── docker-compose.yml
```

## Bet Types & Payouts

| Bet Type | Payout | Description |
|----------|--------|-------------|
| Straight | 35:1 | Single number (0-36) |
| Split | 17:1 | Two adjacent numbers |
| Street | 11:1 | Row of 3 numbers |
| Corner | 8:1 | Four adjacent numbers |
| Six Line | 5:1 | Two rows (6 numbers) |
| Red/Black | 1:1 | Color bet |
| Odd/Even | 1:1 | Parity bet |
| Low/High | 1:1 | 1-18 or 19-36 |
| Dozen | 2:1 | 1-12, 13-24, or 25-36 |
| Column | 2:1 | Column of 12 numbers |

## Provable Fairness

MicroRoulette uses a commit-reveal scheme for provable fairness:

1. **Before Spin**: Server seed hash is committed (shown in UI)
2. **During Bet**: Players see commitment but not actual seed
3. **On Spin**: Client seed combined with server seed
4. **After Spin**: Server seed revealed for verification

**Algorithm:**
```
result = SHA256(server_seed + client_seed + nonce)[0] mod 37
```

You can verify any spin in the Fairness Verifier section of the app.

## Linera Features Used

- ✅ **Microchains** - Each game runs on its own microchain for isolation
- ✅ **Linera SDK v0.15.8** - Smart contracts compiled to WASM
- ✅ **Linera Views** - MapView, RegisterView, QueueView for persistent state
- ✅ **GraphQL Service** - Query and mutate state via async-graphql
- ✅ **Cross-chain Messages** - SpinResult, BetConfirmed, Payout, Refund
- ✅ **linera-web (@linera/client)** - Browser-native blockchain connection
- ✅ **Conway Testnet** - Deployed to production testnet
- ✅ **Sub-second Finality** - Fast settlement for real-time gameplay

## Technical Stack

### Backend (Smart Contract)
- **Linera SDK**: v0.15.8
- **State Management**: Linera Views (MapView, RegisterView, QueueView)
- **Target**: wasm32-unknown-unknown

### Frontend
- **Framework**: Vue.js 3.4+
- **Blockchain Client**: @linera/client (linera-web)
- **Build Tool**: Vite 5.x
- **Styling**: Tailwind CSS

### Testnet
- **Network**: Conway
- **Faucet**: https://faucet.testnet-conway.linera.net

## Development

### Building Contracts Only

```bash
./init.sh --build-only
```

### Skipping Deployment

```bash
./init.sh --no-deploy
```

### Environment Variables

```bash
LINERA_FAUCET_URL=https://faucet.testnet-conway.linera.net
FRONTEND_PORT=8080
API_PORT=8081
```

## API Reference

### GraphQL Queries

```graphql
# Get chain ID (critical for judges!)
query { chainId }

# Get table configuration
query { config { minBet maxBet houseEdgeBps } }

# Get current status
query { tableStatus { status spinNumber isBettingOpen } }

# Get spin history
query { spinHistory(limit: 10) { spinId result resultColor } }

# Get hot/cold numbers
query { hotNumbers coldNumbers }

# Verify fairness
query {
  verifyFairness(serverSeed: "...", clientSeed: "...", nonce: "1") {
    result isValid
  }
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT

---

Built for the Linera Hackathon 🏆
