# Priority Ordering Sweep

## Executive Summary

**CRITICAL FINDING**: The feature database has a fundamental ordering problem. The implementation/scaffolding features (rust-toolchain.toml, Cargo.toml, contract types, etc.) are assigned priorities 185-341, while acceptance test features are assigned priorities 1-184.

According to `app_spec.txt`, the implementation features should be done FIRST (phases 1-10, priorities 1-28 approximately), followed by the acceptance test features.

**Current state**: 341 features, 0 passing.

---

## Current Order Analysis

### What the Spec Requires (app_spec.txt lines 1-77):

| Phase | Priority | Features |
|-------|----------|----------|
| 1 | HIGHEST (1-3) | rust-toolchain.toml, Cargo.toml, package.json |
| 2 | HIGH (4-7) | types.rs, lib.rs, state.rs, operations.rs |
| 3 | HIGH (8-9) | contract.rs, service.rs |
| 4 | CRITICAL (10) | **WASM Build Verification checkpoint** |
| 5 | HIGH (11-12) | conway_deploy.sh, run.sh |
| 6 | MEDIUM-HIGH (13-15) | index.html, main.js, useLinera.js |
| 7 | MEDIUM (16-19) | useRoulette.js, ChainInfo.vue, RouletteWheel.vue, BettingBoard.vue |
| 8 | MEDIUM (20-23) | Header.vue, SpinHistory.vue, HotColdNumbers.vue, FairnessVerifier.vue |
| 9 | MEDIUM (24-25) | App.vue integration |
| 10 | MEDIUM (26-28) | Dockerfile, README, polish |

### What the Database Actually Has:

| Current Priority Range | Category | Description |
|------------------------|----------|-------------|
| 1-12 | A. Security & Access Control | Admin roles, bet validation, balance checks |
| 13-25 | B. Navigation Integrity | UI navigation acceptance tests |
| 26-45 | C. Real Data Verification | Blockchain data verification tests |
| 46-63 | D. Workflow Completeness | End-to-end workflow tests |
| 64-77 | E. Error Handling | Error state tests |
| 78-87 | F. UI-Backend Integration | Integration tests |
| 88-95 | G. State & Persistence | State persistence tests |
| 96-100 | H. URL & Direct Access | URL routing tests |
| 101-107 | I. Double-Action & Idempotency | Idempotency tests |
| 108-117 | J-K. Cleanup, Defaults | Cleanup and default tests |
| 118-145 | L-N. Search, Forms, Feedback | Various validation tests |
| 146-184 | O-T. Layout, Accessibility, etc. | UI quality tests |
| **185-341** | **functional/Buildathon/Frontend/style** | **ACTUAL IMPLEMENTATION FEATURES** |

**The problem is clear**: Acceptance tests (which require a working app) are prioritized BEFORE the features that build the app.

---

## Issues Found

| Feature ID | Current Priority | Should Be | Reason |
|------------|------------------|-----------|--------|
| 185 | 185 | **1** | rust-toolchain.toml - Phase 1 scaffolding, MUST be first |
| 186 | 186 | **2** | Contracts Cargo.toml - Phase 1 scaffolding |
| 197 | 197 | **3** | Frontend package.json - Phase 1 scaffolding |
| 187 | 187 | **4** | RouletteNumber type - Phase 2 types |
| 188-189 | 188-189 | **5-6** | BetType implementation - Phase 2 types |
| 194 | 194 | **7** | Linera Views state - Phase 2 state |
| 195-196 | 195-196 | **8-9** | Operations/Message enums - Phase 2 operations |
| 190, 267-269 | 190, 267-269 | **10-13** | Contract trait methods - Phase 3 contract |
| 191, 261 | 191, 261 | **14-15** | Service trait methods - Phase 3 service |
| 213, 284-285 | 213, 284-285 | **16** | **WASM compiles - CRITICAL PHASE 4 CHECKPOINT** |
| 322-323 | 322-323 | **17-18** | Deployment scripts - Phase 5 |
| 200, 198-199 | 200, 198-199 | **19-21** | Frontend foundation - Phase 6 |
| 201-203 | 201-203 | **22-24** | Composables - Phase 6 |
| 212, 204-206 | 212, 204-206 | **25-28** | Core components - Phase 7 |
| 211, 207-210 | 211, 207-210 | **29-33** | Supporting components - Phase 8 |
| 215 | 215 | **34** | Frontend connects to contract - Phase 9 integration |
| 216-217 | 216-217 | **35-36** | Docker deployment - Phase 10 |
| 324-331 | 324-331 | **37-44** | Buildathon requirements - Phase 10 |
| 1-184 | 1-184 | **45-228** | Acceptance tests - AFTER implementation |

---

## Recommended Priority Updates

```sql
-- ============================================================
-- PHASE 1: Project Scaffolding (NEW Priority 1-3)
-- ============================================================
UPDATE features SET priority = 1 WHERE name = 'rust-toolchain.toml pins Rust 1.86.0';
UPDATE features SET priority = 2 WHERE name = 'Contracts Cargo.toml correct';
UPDATE features SET priority = 3 WHERE name = 'Frontend package.json correct';

-- ============================================================
-- PHASE 2: Contract Types & ABI (NEW Priority 4-12)
-- ============================================================
UPDATE features SET priority = 4 WHERE name = 'RouletteNumber type implementation';
UPDATE features SET priority = 5 WHERE name = 'RouletteNumber is_red method';
UPDATE features SET priority = 6 WHERE name = 'RouletteNumber is_black method';
UPDATE features SET priority = 7 WHERE name = 'RouletteNumber dozen method';
UPDATE features SET priority = 8 WHERE name = 'RouletteNumber column method';
UPDATE features SET priority = 9 WHERE name = 'RouletteNumber is_low method';
UPDATE features SET priority = 10 WHERE name = 'RouletteNumber is_high method';
UPDATE features SET priority = 11 WHERE name = 'BetType payout multipliers correct';
UPDATE features SET priority = 12 WHERE name = 'BetType win detection correct';
UPDATE features SET priority = 13 WHERE name = 'BetType is_valid method';
UPDATE features SET priority = 14 WHERE name = 'Linera Views state correctly defined';
UPDATE features SET priority = 15 WHERE name = 'Operations enum complete';
UPDATE features SET priority = 16 WHERE name = 'Message enum complete';

-- ============================================================
-- PHASE 3: Contract Implementation (NEW Priority 17-28)
-- ============================================================
UPDATE features SET priority = 17 WHERE name = 'Contract implements correct traits';
UPDATE features SET priority = 18 WHERE name = 'Contract load() method';
UPDATE features SET priority = 19 WHERE name = 'Contract instantiate() method';
UPDATE features SET priority = 20 WHERE name = 'Contract store() method';
UPDATE features SET priority = 21 WHERE name = 'Contract source files exist';
UPDATE features SET priority = 22 WHERE name = 'Service implements correct traits';
UPDATE features SET priority = 23 WHERE name = 'Service new() initialization';
UPDATE features SET priority = 24 WHERE name = 'GraphQL schema complete';
UPDATE features SET priority = 25 WHERE name = 'Message::SpinResult handler';
UPDATE features SET priority = 26 WHERE name = 'Message::BetConfirmed handler';
UPDATE features SET priority = 27 WHERE name = 'Message::BetRejected handler';
UPDATE features SET priority = 28 WHERE name = 'Message::Payout handler';
UPDATE features SET priority = 29 WHERE name = 'Message::Refund handler';
UPDATE features SET priority = 30 WHERE name = 'FairnessProof verification works';
UPDATE features SET priority = 31 WHERE name = 'FairnessProof generate method';
UPDATE features SET priority = 32 WHERE name = 'FairnessProof hash_seed method';

-- ============================================================
-- PHASE 4: WASM Build Verification - CRITICAL CHECKPOINT (NEW Priority 33-35)
-- ============================================================
UPDATE features SET priority = 33 WHERE name = 'WASM contract compiles';
UPDATE features SET priority = 34 WHERE name = 'WASM file at expected path';
UPDATE features SET priority = 35 WHERE name = 'No mock blockchain code';
UPDATE features SET priority = 36 WHERE name = 'All deps from crates.io';

-- ============================================================
-- PHASE 5: Deployment Scripts (NEW Priority 37-40)
-- ============================================================
UPDATE features SET priority = 37 WHERE name = 'conway_deploy.sh script exists';
UPDATE features SET priority = 38 WHERE name = 'conway_deploy.sh deploys successfully';
UPDATE features SET priority = 39 WHERE name = 'Contract deploys to Conway';
UPDATE features SET priority = 40 WHERE name = 'Deployment persists App ID';
UPDATE features SET priority = 41 WHERE name = 'init.sh script works';

-- ============================================================
-- PHASE 6: Frontend Foundation (NEW Priority 42-52)
-- ============================================================
UPDATE features SET priority = 42 WHERE name = 'Vite config WASM settings correct';
UPDATE features SET priority = 43 WHERE name = 'Tailwind roulette colors defined';
UPDATE features SET priority = 44 WHERE name = 'index.html import map correct';
UPDATE features SET priority = 45 WHERE name = 'useLinera composable functional';
UPDATE features SET priority = 46 WHERE name = 'useRoulette composable functional';
UPDATE features SET priority = 47 WHERE name = 'useBets composable functional';
UPDATE features SET priority = 48 WHERE name = 'Vue version 3.4+';
UPDATE features SET priority = 49 WHERE name = '@linera/client version';
UPDATE features SET priority = 50 WHERE name = 'Environment variables propagated';

-- ============================================================
-- PHASE 7: Frontend Core Components (NEW Priority 51-65)
-- ============================================================
UPDATE features SET priority = 51 WHERE name = 'ChainInfo displays connection status';
UPDATE features SET priority = 52 WHERE name = 'RouletteWheel SVG correct';
UPDATE features SET priority = 53 WHERE name = 'BettingBoard layout correct';
UPDATE features SET priority = 54 WHERE name = 'ChipSelector displays all chips';

-- ============================================================
-- PHASE 8: Frontend Supporting Components (NEW Priority 55-70)
-- ============================================================
UPDATE features SET priority = 55 WHERE name = 'Header displays balance';
UPDATE features SET priority = 56 WHERE name = 'SpinHistory displays correctly';
UPDATE features SET priority = 57 WHERE name = 'HotColdNumbers displays correctly';
UPDATE features SET priority = 58 WHERE name = 'FairnessVerifier displays correctly';
UPDATE features SET priority = 59 WHERE name = 'WinningsPopup displays correctly';

-- ============================================================
-- PHASE 9: Integration (NEW Priority 60-65)
-- ============================================================
UPDATE features SET priority = 60 WHERE name = 'Frontend connects to contract';

-- ============================================================
-- PHASE 10: Deployment & Polish (NEW Priority 66-80)
-- ============================================================
UPDATE features SET priority = 66 WHERE name = 'Dockerfile builds successfully';
UPDATE features SET priority = 67 WHERE name = 'docker-compose up works from fresh clone';
UPDATE features SET priority = 68 WHERE name = 'README lists Linera features';
UPDATE features SET priority = 69 WHERE name = 'README documents on-chain vs off-chain';
UPDATE features SET priority = 70 WHERE name = 'Demo video exists';
UPDATE features SET priority = 71 WHERE name = 'Demo video shows Chain ID';
UPDATE features SET priority = 72 WHERE name = 'Demo video shows full game flow';
UPDATE features SET priority = 73 WHERE name = 'Repository is public';
UPDATE features SET priority = 74 WHERE name = 'App ID displayed in UI';
UPDATE features SET priority = 75 WHERE name = 'Linera integration score maximized';

-- ============================================================
-- PHASE 11+: Bet Type Payouts (NEW Priority 76-90)
-- ============================================================
UPDATE features SET priority = 76 WHERE name = 'Straight bet payout 35:1';
UPDATE features SET priority = 77 WHERE name = 'Split bet payout 17:1';
UPDATE features SET priority = 78 WHERE name = 'Street bet payout 11:1';
UPDATE features SET priority = 79 WHERE name = 'Corner bet payout 8:1';
UPDATE features SET priority = 80 WHERE name = 'SixLine bet payout 5:1';
UPDATE features SET priority = 81 WHERE name = 'Dozen bet payout 2:1';
UPDATE features SET priority = 82 WHERE name = 'Column bet payout 2:1';
UPDATE features SET priority = 83 WHERE name = 'Red bet payout 1:1';
UPDATE features SET priority = 84 WHERE name = 'Black bet payout 1:1';
UPDATE features SET priority = 85 WHERE name = 'Odd bet payout 1:1';
UPDATE features SET priority = 86 WHERE name = 'Even bet payout 1:1';
UPDATE features SET priority = 87 WHERE name = 'Low bet payout 1:1';
UPDATE features SET priority = 88 WHERE name = 'High bet payout 1:1';
UPDATE features SET priority = 89 WHERE name = 'Zero wins only straight bet';
UPDATE features SET priority = 90 WHERE name = 'Payout includes original stake';
UPDATE features SET priority = 91 WHERE name = 'Multiple winning bets all paid';

-- ============================================================
-- GraphQL Operations (NEW Priority 92-105)
-- ============================================================
UPDATE features SET priority = 92 WHERE name = 'GraphQL placeBet mutation';
UPDATE features SET priority = 93 WHERE name = 'GraphQL spin mutation';
UPDATE features SET priority = 94 WHERE name = 'GraphQL deposit mutation';
UPDATE features SET priority = 95 WHERE name = 'GraphQL withdraw mutation';
UPDATE features SET priority = 96 WHERE name = 'GraphQL clearBets mutation';
UPDATE features SET priority = 97 WHERE name = 'GraphQL doubleBets mutation';
UPDATE features SET priority = 98 WHERE name = 'GraphQL lastSpin query';
UPDATE features SET priority = 99 WHERE name = 'GraphQL isPaused query';
UPDATE features SET priority = 100 WHERE name = 'GraphQL platformStats query';

-- ============================================================
-- Contract State and Validation (NEW Priority 101-115)
-- ============================================================
UPDATE features SET priority = 101 WHERE name = 'StartSpin operation';
UPDATE features SET priority = 102 WHERE name = 'ResolveSpin operation';
UPDATE features SET priority = 103 WHERE name = 'TableStatus::PayingOut state';
UPDATE features SET priority = 104 WHERE name = 'SpinResult player_count field';
UPDATE features SET priority = 105 WHERE name = 'PlayerStats biggest_win tracking';
UPDATE features SET priority = 106 WHERE name = 'PlayerStats current_streak tracking';
UPDATE features SET priority = 107 WHERE name = 'PlayerStats best_streak persistence';
UPDATE features SET priority = 108 WHERE name = 'Corner bet validation';
UPDATE features SET priority = 109 WHERE name = 'Unauthorized admin rejected';
UPDATE features SET priority = 110 WHERE name = 'Invalid state transition rejected';
UPDATE features SET priority = 111 WHERE name = 'Betting deadline enforced';
UPDATE features SET priority = 112 WHERE name = 'Commit-reveal hash commitment';
UPDATE features SET priority = 113 WHERE name = 'Commit-reveal hash verification';
UPDATE features SET priority = 114 WHERE name = 'Hash to number derivation';

-- ============================================================
-- Frontend Advanced Features (NEW Priority 115-145)
-- ============================================================
UPDATE features SET priority = 115 WHERE name = 'BettingBoard split bet placement';
UPDATE features SET priority = 116 WHERE name = 'BettingBoard street bet placement';
UPDATE features SET priority = 117 WHERE name = 'BettingBoard corner bet placement';
UPDATE features SET priority = 118 WHERE name = 'BettingBoard six-line bet placement';
UPDATE features SET priority = 119 WHERE name = 'BettingBoard bet amount visualization';
UPDATE features SET priority = 120 WHERE name = 'RouletteWheel ball animation';
UPDATE features SET priority = 121 WHERE name = 'RouletteWheel winning highlight';
UPDATE features SET priority = 122 WHERE name = 'RouletteWheel spin-complete event';
UPDATE features SET priority = 123 WHERE name = 'ChipSelector disabled state';
UPDATE features SET priority = 124 WHERE name = 'useLinera reconnection logic';
UPDATE features SET priority = 125 WHERE name = 'useLinera wallet balance query';
UPDATE features SET priority = 126 WHERE name = 'useLinera transaction signing';
UPDATE features SET priority = 127 WHERE name = 'useBets balance validation';
UPDATE features SET priority = 128 WHERE name = 'useBets min/max validation';
UPDATE features SET priority = 129 WHERE name = 'useBets total limit validation';
UPDATE features SET priority = 130 WHERE name = 'useRoulette error state tracking';
UPDATE features SET priority = 131 WHERE name = 'useRoulette spin result callback';
UPDATE features SET priority = 132 WHERE name = 'Win popup bet breakdown';
UPDATE features SET priority = 133 WHERE name = 'Spin history click to verify';
UPDATE features SET priority = 134 WHERE name = 'Hot/cold numbers click to bet';
UPDATE features SET priority = 135 WHERE name = 'Header balance animation';
UPDATE features SET priority = 136 WHERE name = 'Header deposit/withdraw buttons';
UPDATE features SET priority = 137 WHERE name = 'Fairness verifier copy buttons';
UPDATE features SET priority = 138 WHERE name = 'Fairness verifier auto-populate';
UPDATE features SET priority = 139 WHERE name = 'ChainInfo connection retry button';
UPDATE features SET priority = 140 WHERE name = 'ChainInfo faucet link';
UPDATE features SET priority = 141 WHERE name = 'Win popup confetti animation';
UPDATE features SET priority = 142 WHERE name = 'BettingBoard bet removal';
UPDATE features SET priority = 143 WHERE name = 'roulette.js validateBet function';

-- ============================================================
-- Style Features (NEW Priority 144-160)
-- ============================================================
UPDATE features SET priority = 144 WHERE name = 'Roulette-red color correct';
UPDATE features SET priority = 145 WHERE name = 'Roulette-black color correct';
UPDATE features SET priority = 146 WHERE name = 'Roulette-green color correct';
UPDATE features SET priority = 147 WHERE name = 'Felt-green background used';
UPDATE features SET priority = 148 WHERE name = 'Chip selection ring visible';
UPDATE features SET priority = 149 WHERE name = 'Bet highlight ring visible';
UPDATE features SET priority = 150 WHERE name = 'Wheel animation smooth';
UPDATE features SET priority = 151 WHERE name = 'Status indicator colors correct';
UPDATE features SET priority = 152 WHERE name = 'Button hover states work';
UPDATE features SET priority = 153 WHERE name = 'Disabled button styling';
UPDATE features SET priority = 154 WHERE name = 'Win popup styling correct';
UPDATE features SET priority = 155 WHERE name = 'Result number styling correct';
UPDATE features SET priority = 156 WHERE name = 'Number cell hover effect';

-- ============================================================
-- ACCEPTANCE TESTS: Move to after implementation (Priority 200+)
-- These require a working application
-- ============================================================

-- A. Security & Access Control (currently 1-12) -> 200-211
UPDATE features SET priority = priority + 199 WHERE category = 'A. Security & Access Control';

-- B. Navigation Integrity (currently 13-25) -> 212-224
UPDATE features SET priority = priority + 199 WHERE category = 'B. Navigation Integrity';

-- C. Real Data Verification (currently 26-45) -> 225-244
UPDATE features SET priority = priority + 199 WHERE category = 'C. Real Data Verification';

-- D. Workflow Completeness (currently 46-63) -> 245-262
UPDATE features SET priority = priority + 199 WHERE category = 'D. Workflow Completeness';

-- E. Error Handling (currently 64-77) -> 263-276
UPDATE features SET priority = priority + 199 WHERE category = 'E. Error Handling';

-- F. UI-Backend Integration (currently 78-87) -> 277-286
UPDATE features SET priority = priority + 199 WHERE category = 'F. UI-Backend Integration';

-- G. State & Persistence (currently 88-95) -> 287-294
UPDATE features SET priority = priority + 199 WHERE category = 'G. State & Persistence';

-- H. URL & Direct Access (currently 96-100) -> 295-299
UPDATE features SET priority = priority + 199 WHERE category = 'H. URL & Direct Access';

-- I. Double-Action & Idempotency (currently 101-107) -> 300-306
UPDATE features SET priority = priority + 199 WHERE category = 'I. Double-Action & Idempotency';

-- J-T categories similarly shifted
UPDATE features SET priority = priority + 199 WHERE category LIKE 'J.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'K.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'L.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'M.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'N.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'O.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'P.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'Q.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'R.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'S.%';
UPDATE features SET priority = priority + 199 WHERE category LIKE 'T.%';
```

---

## Phase Gates Verification

Based on current database structure:

- [x] **Phase 4 gate (WASM compiles) exists** - Feature ID 213: "WASM contract compiles" and ID 285: "WASM file at expected path"
- [x] **Phase 5 gate (deploys to Conway) exists** - Feature ID 214: "Contract deploys to Conway" and ID 323: "conway_deploy.sh deploys successfully"
- [x] **Phase 9 gate (integration works) exists** - Feature ID 215: "Frontend connects to contract"

**ISSUE**: These gates exist but are at wrong priority levels (213-215 and 322-323 instead of ~16-18 and ~34-40).

---

## Additional Issues Identified

### 1. Missing Phase Gate Features
The spec mentions explicit test gates that should BLOCK further progress:
- After Phase 4: "Contract MUST compile to WASM" - exists but not marked as blocking
- After Phase 5: "Contract MUST deploy to Conway testnet" - exists but not marked as blocking
- After Phase 6: "Frontend MUST connect to deployed contract" - not explicitly a gate
- After Phase 9: "Full game flow MUST work end-to-end" - not a specific feature

### 2. Category Naming Issues
Some "functional" features should be categorized more specifically:
- "Contract" category for contract implementation
- "Frontend" category for frontend components (partially exists)
- "Deployment" category for deployment scripts

### 3. Dependency Chain Not Captured
The priority system doesn't capture hard dependencies:
- rust-toolchain.toml MUST exist before any Rust compilation
- Contract types MUST compile before contract logic
- WASM MUST compile before deployment can be tested
- Deployment MUST work before frontend integration testing

---

## Recommendation

**Option A (Recommended): Re-prioritize all features**
Execute the SQL statements above to reorder features properly. This ensures the coding agent builds the application in the correct order.

**Option B: Add explicit phase gate features**
Create new "checkpoint" features that must pass before proceeding:
```sql
INSERT INTO features (id, priority, category, name, description, steps, passes) VALUES
(400, 10, 'CHECKPOINT', 'PHASE 1 COMPLETE: Scaffolding verified', 'All config files exist', '[]', 0),
(401, 16, 'CHECKPOINT', 'PHASE 4 COMPLETE: WASM compiles', 'cargo build --release --target wasm32-unknown-unknown succeeds', '[]', 0),
(402, 40, 'CHECKPOINT', 'PHASE 5 COMPLETE: Conway deployment', 'linera deploy succeeds on testnet', '[]', 0),
(403, 65, 'CHECKPOINT', 'PHASE 9 COMPLETE: Integration', 'Frontend can place bet and spin', '[]', 0);
```

**Option C: Two-pass approach**
1. First pass: Only implement "functional" category features (current 185-341)
2. Second pass: Run acceptance tests (categories A-T)

---

## Summary of Changes Required

| Change Type | Count | Impact |
|-------------|-------|--------|
| Features needing priority reduction (implementation first) | ~110 | HIGH |
| Features needing priority increase (acceptance tests last) | ~184 | HIGH |
| Missing checkpoint features | 4 | MEDIUM |
| Category reorganization needed | ~50 | LOW |

**TOTAL FEATURES**: 341
**PASSING**: 0
**PRIORITY ORDERING ISSUES**: ~294 features (86%) are mis-prioritized

The recommended approach is to execute the SQL statements in the "Recommended Priority Updates" section above to properly align the database with the implementation phases defined in `app_spec.txt`.
