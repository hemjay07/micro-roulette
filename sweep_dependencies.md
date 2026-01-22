# Dependencies Sweep

## Rust Dependencies

| Package | Required | Found | Status |
|---------|----------|-------|--------|
| linera-sdk | 0.15.8 | 0.15.8 | PASS |
| linera-views | 0.15.8 | 0.15.8 | PASS |
| async-graphql | 7.0 | 7.0 | PASS |
| serde | 1.0 (with derive) | 1.0 (with derive) | PASS |
| sha2 | 0.10 | 0.10 | PASS |
| hex | 0.4 | 0.4 | PASS |

### Additional Dependencies Found
| Package | Version | Notes |
|---------|---------|-------|
| serde_json | 1.0 | Standard JSON serialization |
| thiserror | 1.0 | Error handling |
| log | 0.4 | Logging |

### Dev Dependencies
| Package | Version | Status |
|---------|---------|--------|
| linera-sdk (test) | 0.15.8 | PASS |

### Dependency Quality Checks
| Check | Status |
|-------|--------|
| No `path = ` dependencies | PASS |
| No `git = ` dependencies | PASS |
| edition = "2021" | PASS |
| Correct lib target | PASS |
| Correct bin targets (contract + service) | PASS |

## Rust Toolchain

- Required: 1.86.0
- Found: 1.86.0
- WASM target: wasm32-unknown-unknown
- Status: **PASS**

**Note:** The rust-toolchain.toml correctly pins to 1.86.0 to avoid opcode 252 errors on Conway testnet.

## Frontend Dependencies

| Package | Required | Found | Status |
|---------|----------|-------|--------|
| vue | ^3.4.0 | ^3.4.0 | PASS |
| @linera/client | 0.15.8 | 0.15.8 | PASS |
| vite | ^5.0.0 | ^5.0.0 | PASS |
| tailwindcss | ^3.4.0 | ^3.4.1 | PASS |
| @vitejs/plugin-vue | (required) | ^5.0.0 | PASS |

### Additional Frontend Dependencies Found
| Package | Version | Notes |
|---------|---------|-------|
| canvas-confetti | ^1.9.0 | Animation effects |
| autoprefixer | ^10.4.17 | CSS processing |
| postcss | ^8.4.35 | CSS processing |

## Dependency Issues

**None found.** All required dependencies are present with correct versions.

## Summary

| Category | Status |
|----------|--------|
| Rust Core Dependencies | PASS |
| Rust Toolchain | PASS |
| Frontend Dependencies | PASS |
| No Path Dependencies | PASS |
| No Git Dependencies | PASS |

All dependencies are correctly configured for Linera SDK 0.15.8 and Conway testnet deployment.

## Recommended Fixes

No fixes required. All dependencies are correctly specified.

### Optional Recommendations

1. **Lock file verification**: Run `cargo update --dry-run` to ensure all transitive dependencies resolve correctly.

2. **Frontend lock file**: Ensure `package-lock.json` is committed for reproducible builds:
   ```bash
   cd frontend && npm install
   ```

3. **Pre-deployment check**: Before deploying to Conway, verify the WASM build:
   ```bash
   cd contracts && cargo build --release --target wasm32-unknown-unknown
   ```
