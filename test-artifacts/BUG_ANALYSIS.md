# Payout Bug Analysis - MicroRoulette

## Problem Summary
Even-money bets (Red/Black, Odd/Even, Low/High) are not paying out correctly. Players are receiving reduced winnings.

## Observed Behavior
**Test Case:** 10-chip bet on BLACK, result lands on BLACK (win)
- Expected profit: +10 chips (1:1 payout)
- Actual profit: 0 chips (break even)
- **Bug confirmed:** Payout calculation error

## Debug Output Analysis

From browser console logs during testing:
```
=== PAYOUT CALCULATION DEBUG ===
Result: 2 (BLACK)
Balance before: 1000
Total bet amount: 20  ← SHOULD BE 10!
Winning bet: type=black, amount=10, multiplier=1, payout=20
Total win amount: 20
Balance change: 0 (totalWin: 20 - totalBet: 20)
Balance after: 1000
```

## Root Cause

The `totalBetAmount` computed property is reporting **20 chips** when only **10 chips** were bet.

This indicates one of two issues:
1. **Bets are being placed twice** (duplication bug)
2. **Bets are being counted twice** in the computed property

## Code Flow Analysis

### Bet Placement Flow:
1. User clicks BLACK button → `BettingBoard.vue` emits `@place-bet`
2. `App.vue` receives event via `@place-bet="handlePlaceBet"`
3. `handlePlaceBet()` calls `placeBet(betInfo)` from `useBets` composable
4. `placeBet()` adds bet to `currentBets.value` array

### Multiple Event Listeners:
In `App.vue`, there are TWO components listening for `@place-bet`:
- Line 30: `<HotColdNumbers @place-bet="handlePlaceBet"` />`
- Line 48: `<BettingBoard @place-bet="handlePlaceBet" />`

However, HotColdNumbers only emits for straight number bets, not for BLACK bets.

### Payout Calculation (App.vue lines 288-315):
```javascript
function onSpinComplete(result) {
  let totalWin = 0;
  for (const bet of currentBets.value) {
    if (checkBetWin(bet, result)) {
      const multiplier = getPayoutMultiplier(bet.type);
      totalWin += bet.amount * (multiplier + 1);  // ✓ Correct
    }
  }

  if (totalWin > 0) {
    balance.value += totalWin - totalBetAmount.value;  // ← BUG HERE
  }
}
```

### The Calculation:
- `totalWin = 10 * (1 + 1) = 20` ✓ Correct (includes original bet)
- `totalBetAmount.value = 20` ❌ **Should be 10!**
- `balance += 20 - 20 = 0` ❌ Should be `+10`

## Hypothesis

The most likely cause is that clicking a bet button triggers the click event twice, or the `placeBet` function is being called twice. This could be due to:

1. **Event bubbling** - The click event bubbles up and triggers multiple handlers
2. **Vue reactivity issue** - The bet is being added twice due to reactive state update
3. **HMR (Hot Module Replacement)** - During development, event handlers might be duplicated

## Next Steps

1. Add detailed logging to `placeBet()` function to track:
   - How many times it's called
   - The state of `currentBets` before and after each call

2. Check if the bet button click is being handled multiple times

3. Verify the `totalBetAmount` computed property calculation

4. Test if the issue occurs in production build (without HMR)

## Temporary Workaround

None available - this is a critical bug affecting all even-money bets.

## Impact

- **Severity:** CRITICAL
- **Affected Bet Types:** Red, Black, Odd, Even, Low (1-18), High (19-36)
- **Player Impact:** Players receive 0 profit on winning even-money bets instead of 100% profit
- **House Edge Impact:** Effective house edge is much higher than intended
