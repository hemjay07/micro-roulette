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

export function useBets(config) {
  const currentBets = ref([]);
  const lastBets = ref([]);
  const selectedChip = ref(10);
  let isDoubling = false; // Guard against rapid double-clicks
  const validationError = ref(null);

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
   * Validate bet amount against config limits
   */
  function validateBetAmount(amount, existingBetAmount = 0) {
    if (!config || !config.value) {
      return { valid: true };
    }

    const minBet = parseFloat(config.value.minBet) / 1_000_000; // Convert from micro-units
    const maxBet = parseFloat(config.value.maxBet) / 1_000_000;
    const totalAmount = amount + existingBetAmount;

    if (amount < minBet) {
      return {
        valid: false,
        error: `Minimum bet is ${minBet} chips`
      };
    }

    if (totalAmount > maxBet) {
      return {
        valid: false,
        error: `Maximum bet per position is ${maxBet} chips`
      };
    }

    // Check total bet limit
    const maxTotalBet = parseFloat(config.value.maxTotalBet || config.value.maxBet * 5) / 1_000_000;
    const newTotal = totalBetAmount.value + amount;

    if (newTotal > maxTotalBet) {
      return {
        valid: false,
        error: `Maximum total bet per round is ${maxTotalBet} chips`
      };
    }

    return { valid: true };
  }

  /**
   * Place a bet
   */
  function placeBet(betInfo) {
    validationError.value = null;

    // Normalize bet type to lowercase for payout lookup
    const betType = (betInfo.betType || betInfo.type || '').toLowerCase();

    // Determine secondary identifier based on bet type
    let number = betInfo.number;
    let dozen = null;
    let column = null;
    let sixLineStart = null;
    let numbers = betInfo.numbers; // For split, street, corner bets

    if (betType === 'dozen') {
      dozen = betInfo.number;
      number = null;
    } else if (betType === 'column') {
      column = betInfo.number;
      number = null;
    } else if (betType === 'sixline') {
      sixLineStart = betInfo.number || betInfo.sixLineStart;
      number = null;
    } else if (betType === 'split' && numbers && Array.isArray(numbers)) {
      // Normalize numbers array to sorted order for consistent comparison
      numbers = [...numbers].sort((a, b) => a - b);
      number = null;
    }

    // Check if same bet exists to get existing amount
    const existingBet = currentBets.value.find(b => {
      if (b.type !== betType) return false;

      // For split bets, compare numbers arrays
      if (betType === 'split') {
        if (!b.numbers || !numbers) return false;
        const bNums = [...b.numbers].sort((a, b) => a - b);
        const checkNums = [...numbers].sort((a, b) => a - b);
        return bNums.length === checkNums.length &&
               bNums.every((n, i) => n === checkNums[i]);
      }

      // For other bet types
      return b.number === number &&
             b.column === column &&
             b.dozen === dozen &&
             b.sixLineStart === sixLineStart;
    });

    // Validate bet amount
    const validation = validateBetAmount(
      selectedChip.value,
      existingBet ? existingBet.amount : 0
    );

    if (!validation.valid) {
      validationError.value = validation.error;
      console.warn('Bet validation failed:', validation.error);
      return false;
    }

    const bet = {
      id: Date.now() + Math.random(),
      type: betType,
      amount: selectedChip.value,
      number,
      dozen,
      column,
      sixLineStart,
      numbers, // Include numbers array for split/corner bets
    };

    // Check if same bet exists, if so, add to it
    const existingIndex = currentBets.value.findIndex(b => {
      if (b.type !== bet.type) return false;

      // For split bets, compare numbers arrays
      if (betType === 'split') {
        if (!b.numbers || !bet.numbers) return false;
        const bNums = [...b.numbers].sort((a, b) => a - b);
        const betNums = [...bet.numbers].sort((a, b) => a - b);
        return bNums.length === betNums.length &&
               bNums.every((n, i) => n === betNums[i]);
      }

      // For other bet types
      return b.number === bet.number &&
             b.column === bet.column &&
             b.dozen === bet.dozen &&
             b.sixLineStart === bet.sixLineStart;
    });

    if (existingIndex >= 0) {
      currentBets.value[existingIndex].amount += bet.amount;
    } else {
      currentBets.value.push(bet);
    }

    return true;
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
   * Double current bets (with debounce protection)
   */
  function doubleBets() {
    // Guard against rapid double-clicks
    if (isDoubling) return;
    isDoubling = true;

    currentBets.value = currentBets.value.map(bet => ({
      ...bet,
      amount: bet.amount * 2,
    }));

    // Reset guard after short delay
    setTimeout(() => {
      isDoubling = false;
    }, 300);
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
   * Remove bet from a specific position
   */
  function removeBetFromPosition(betInfo) {
    const betType = (betInfo.betType || betInfo.type || '').toLowerCase();
    let number = betInfo.number;
    let dozen = null;
    let column = null;
    let sixLineStart = null;
    let numbers = betInfo.numbers;

    if (betType === 'dozen') {
      dozen = betInfo.number;
      number = null;
    } else if (betType === 'column') {
      column = betInfo.number;
      number = null;
    } else if (betType === 'sixline') {
      sixLineStart = betInfo.number || betInfo.sixLineStart;
      number = null;
    } else if (betType === 'split' && numbers && Array.isArray(numbers)) {
      // Normalize numbers array to sorted order for consistent comparison
      numbers = [...numbers].sort((a, b) => a - b);
      number = null;
    }

    const index = currentBets.value.findIndex(b => {
      if (b.type !== betType) return false;

      // For split bets, compare numbers arrays
      if (betType === 'split') {
        if (!b.numbers || !numbers) return false;
        const bNums = [...b.numbers].sort((a, b) => a - b);
        const checkNums = [...numbers].sort((a, b) => a - b);
        return bNums.length === checkNums.length &&
               bNums.every((n, i) => n === checkNums[i]);
      }

      // For other bet types
      return b.number === number &&
             b.column === column &&
             b.dozen === dozen &&
             b.sixLineStart === sixLineStart;
    });

    if (index >= 0) {
      currentBets.value.splice(index, 1);
      return true;
    }

    return false;
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
        case 'split':
          betType = { Split: bet.numbers };
          break;
        case 'street':
          betType = { Street: bet.number };
          break;
        case 'corner':
          betType = { Corner: bet.numbers };
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
        case 'sixline':
        case 'sixLine':
          betType = { SixLine: bet.sixLineStart || bet.number };
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
    validationError,
    placeBet,
    clearBets,
    repeatLastBet,
    doubleBets,
    removeBet,
    removeBetFromPosition,
    getBetsForContract,
  };
}
