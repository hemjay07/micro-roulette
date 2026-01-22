// src/composables/useBets.js
import { ref, computed } from 'vue';

// Payout multipliers
const PAYOUTS = {
  straight: 35,
  split: 17,
  street: 11,
  corner: 8,
  sixLine: 5,
  red: 1,
  black: 1,
  odd: 1,
  even: 1,
  low: 1,
  high: 1,
  dozen: 2,
  column: 2,
};

// Available chip values
const CHIP_VALUES = [1, 5, 10, 25, 100, 500];

export function useBets() {
  const currentBets = ref([]);
  const lastBets = ref([]);
  const selectedChip = ref(10);

  // Computed
  const totalBetAmount = computed(() => {
    return currentBets.value.reduce((sum, bet) => sum + bet.amount, 0);
  });

  const maxPotentialWin = computed(() => {
    return currentBets.value.reduce((sum, bet) => {
      const multiplier = PAYOUTS[bet.type] || 0;
      return sum + (bet.amount * (multiplier + 1));
    }, 0);
  });

  /**
   * Place a bet
   */
  function placeBet(betInfo) {
    // Normalize bet type to lowercase for payout lookup
    const betType = (betInfo.betType || betInfo.type || '').toLowerCase();

    // Determine secondary identifier based on bet type
    let number = betInfo.number;
    let dozen = null;
    let column = null;

    if (betType === 'dozen') {
      dozen = betInfo.number;
      number = null;
    } else if (betType === 'column') {
      column = betInfo.number;
      number = null;
    }

    const bet = {
      id: Date.now() + Math.random(),
      type: betType,
      amount: selectedChip.value,
      number,
      dozen,
      column,
    };

    // Check if same bet exists, if so, add to it
    const existingIndex = currentBets.value.findIndex(b =>
      b.type === bet.type &&
      b.number === bet.number &&
      b.column === bet.column &&
      b.dozen === bet.dozen
    );

    if (existingIndex >= 0) {
      currentBets.value[existingIndex].amount += bet.amount;
    } else {
      currentBets.value.push(bet);
    }
  }

  /**
   * Clear all bets
   */
  function clearBets() {
    lastBets.value = [...currentBets.value];
    currentBets.value = [];
  }

  /**
   * Repeat last bet
   */
  function repeatLastBet() {
    if (lastBets.value.length > 0) {
      currentBets.value = lastBets.value.map(bet => ({
        ...bet,
        id: Date.now() + Math.random(),
      }));
    }
  }

  /**
   * Double current bets
   */
  function doubleBets() {
    currentBets.value = currentBets.value.map(bet => ({
      ...bet,
      amount: bet.amount * 2,
    }));
  }

  /**
   * Remove a specific bet
   */
  function removeBet(betId) {
    const index = currentBets.value.findIndex(b => b.id === betId);
    if (index >= 0) {
      currentBets.value.splice(index, 1);
    }
  }

  /**
   * Convert bets to format expected by contract
   */
  function getBetsForContract() {
    return currentBets.value.map(bet => {
      let betType;

      switch (bet.type) {
        case 'straight':
          betType = { Straight: bet.number };
          break;
        case 'red':
          betType = 'Red';
          break;
        case 'black':
          betType = 'Black';
          break;
        case 'odd':
          betType = 'Odd';
          break;
        case 'even':
          betType = 'Even';
          break;
        case 'low':
          betType = 'Low';
          break;
        case 'high':
          betType = 'High';
          break;
        case 'dozen':
          betType = { Dozen: bet.dozen };
          break;
        case 'column':
          betType = { Column: bet.column };
          break;
        default:
          betType = { Straight: 0 };
      }

      return {
        bet_type: betType,
        amount: (bet.amount * 1_000_000).toString(), // Convert to micro-units
      };
    });
  }

  return {
    currentBets,
    lastBets,
    selectedChip,
    availableChips: CHIP_VALUES,
    totalBetAmount,
    maxPotentialWin,
    placeBet,
    clearBets,
    repeatLastBet,
    doubleBets,
    removeBet,
    getBetsForContract,
  };
}
