# Duplicate Detection Sweep

## Summary
- Total features: 341
- Potential duplicates found: 12 pairs
- Recommended removals: 8

## Exact Duplicates

| ID 1 | ID 2 | Name | Action |
|------|------|------|--------|
| 7 | 133 | "Bet amount cannot exceed maximum" | **EXACT DUPLICATE** - Both test max bet validation. Remove ID 133 (duplicate in Form Validation category) |
| 6 | 132 | "Bet amount must meet minimum" | **EXACT DUPLICATE** - Both test min bet validation. Remove ID 132 (duplicate in Form Validation category) |
| 44 | 110 | "Spin history limited to 20 entries" / "History trimmed to 20 entries" | **EXACT DUPLICATE** - Same test, different names. Remove ID 110 |

## Semantic Duplicates (Test Same Thing)

| ID 1 | ID 2 | Overlap | Recommendation |
|------|------|---------|----------------|
| 62 | 173 | "Multiple players betting workflow" vs "Multiple players same round" | Both test multiple players betting in same round. **Remove ID 173** - ID 62 covers workflow completely |
| 28 | 90 | "Spin history persists across refresh" vs "Spin history persists across restarts" | Different scopes (refresh vs restart) but semantically very similar. **Keep both** - refresh and restart are distinct tests |
| 10 | 137 | "House edge cannot exceed 10%" vs "House edge validation in UpdateSettings" | ID 10 is specific constraint, ID 137 is general validation. **Keep both** - different specificity levels |
| 87 | 167 | "Timestamp displayed correctly" vs "Timestamp format readable" | Both about readable timestamp display. **Remove ID 167** - redundant with ID 87 |
| 101 | 106 | "Double-click SPIN only spins once" vs "Spin button disabled during spin" | Both prevent multiple spins, but test different mechanisms (idempotency vs disabled state). **Keep both** |
| 11 | 172 | "Bets locked during spinning phase" vs "Bet during spin rejected" | Same behavior tested from different angles (Security vs Concurrency). **Remove ID 172** - ID 11 is more fundamental |
| 5 | 124 | "Invalid bet types are rejected" vs "Bet type validation" | Similar validation tests. ID 5 is security-focused, ID 124 is edge case-focused. **Keep both** - different testing perspectives |
| 182 | 225 | "Spin animation smooth 60fps" vs "Wheel animation smooth" | Both test smooth animation. **Remove ID 225** - ID 182 is more specific |

## Overlapping Features (Subset)

| Parent ID | Child ID | Relationship | Action |
|-----------|----------|--------------|--------|
| 1 | 288 | "Admin-only operations require admin role" contains "Unauthorized admin rejected" | ID 288 is subset of ID 1. **Remove ID 288** |
| 143 | 71, 72 | "Loading state during async operations" covers "Loading state during connection" and "Loading state during spin" | ID 143 is superset. **Keep all** - specific tests valuable for regression |
| 8 | 307 | "Total bet per spin cannot exceed max_total_bet" overlaps "useBets total limit validation" | ID 8 is security constraint, ID 307 is frontend implementation. **Keep both** |
| 124 | 281 | "Bet type validation" overlaps "BetType is_valid method" | ID 124 is integration test, ID 281 is unit test. **Keep both** |

## Recommended Consolidations

None required. The features that remain after duplicate removal are sufficiently distinct.

## Analysis Notes

### Features that appear similar but are legitimately different:

1. **Persistence features (28, 90, 89, 88)**: Test different persistence scenarios (refresh vs restart vs session) - all valid
2. **Balance features (27, 81, 211)**: Test different aspects (chain sync, UI match, header display) - all valid
3. **Spin history features (44, 118, 122, 207)**: Test different aspects (limit, display, parameter, component) - all valid
4. **BetType features (5, 124, 188, 189, 281)**: Test different methods and perspectives - all valid
5. **Animation features (142, 169, 182)**: Test different aspects (feedback, duration, performance) - all valid

### Categories with good separation:
- Security (A) vs Form Validation (M): Security tests enforcement, Validation tests UX
- Workflow (D) vs Functional: Workflow is E2E, Functional is unit/integration
- Frontend vs Functional: Frontend is component-specific, Functional is behavior-specific

## SQL to Remove Duplicates

```sql
-- Remove exact duplicates (Form Validation duplicates of Security tests)
DELETE FROM features WHERE id = 133; -- Bet amount cannot exceed maximum (dup of ID 7)
DELETE FROM features WHERE id = 132; -- Bet amount must meet minimum (dup of ID 6)

-- Remove semantic duplicates
DELETE FROM features WHERE id = 110; -- History trimmed to 20 entries (dup of ID 44)
DELETE FROM features WHERE id = 173; -- Multiple players same round (dup of ID 62)
DELETE FROM features WHERE id = 167; -- Timestamp format readable (dup of ID 87)
DELETE FROM features WHERE id = 172; -- Bet during spin rejected (dup of ID 11)
DELETE FROM features WHERE id = 225; -- Wheel animation smooth (dup of ID 182)

-- Remove subset features
DELETE FROM features WHERE id = 288; -- Unauthorized admin rejected (subset of ID 1)
```

## Post-Cleanup Count

After removing 8 duplicates: **333 features** remaining

## Verification Query

Run this after cleanup to verify no orphaned references:
```sql
SELECT COUNT(*) FROM features; -- Should be 333
```
