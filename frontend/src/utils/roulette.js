// src/utils/roulette.js

// European roulette wheel order (clockwise starting from 0)
export const WHEEL_ORDER = [
  0, 32, 15, 19, 4, 21, 2, 25, 17, 34, 6, 27, 13, 36, 11, 30, 8, 23, 10,
  5, 24, 16, 33, 1, 20, 14, 31, 9, 22, 18, 29, 7, 28, 12, 35, 3, 26
];

// Red numbers in European roulette
export const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];

// Black numbers in European roulette
export const BLACK_NUMBERS = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];

/**
 * Get the color of a roulette number
 * @param {number} num - The roulette number (0-36)
 * @returns {'red' | 'black' | 'green'}
 */
export function getNumberColor(num) {
  if (num === 0) return 'green';
  if (RED_NUMBERS.includes(num)) return 'red';
  return 'black';
}

/**
 * Get the position of a number on the wheel (0-36)
 * @param {number} num - The roulette number
 * @returns {number} Position index in WHEEL_ORDER
 */
export function getWheelPosition(num) {
  return WHEEL_ORDER.indexOf(num);
}

/**
 * Get the rotation angle for a specific number
 * @param {number} num - The roulette number
 * @returns {number} Rotation angle in degrees
 */
export function getNumberRotation(num) {
  const position = getWheelPosition(num);
  const segmentAngle = 360 / 37;
  return position * segmentAngle;
}

/**
 * Calculate the target rotation for a spin result
 * @param {number} result - The winning number
 * @param {number} extraSpins - Extra full rotations (default 5)
 * @returns {number} Total rotation in degrees
 */
export function calculateSpinRotation(result, extraSpins = 5) {
  const segmentAngle = 360 / 37;
  const position = getWheelPosition(result);
  // Position the winning number at the top (ball pointer position)
  const targetAngle = (37 - position) * segmentAngle;
  return extraSpins * 360 + targetAngle;
}

/**
 * Get dozen for a number (1st, 2nd, or 3rd)
 * @param {number} num - The roulette number (1-36)
 * @returns {1 | 2 | 3 | null}
 */
export function getDozen(num) {
  if (num === 0) return null;
  if (num <= 12) return 1;
  if (num <= 24) return 2;
  return 3;
}

/**
 * Get column for a number (1, 2, or 3)
 * @param {number} num - The roulette number (1-36)
 * @returns {1 | 2 | 3 | null}
 */
export function getColumn(num) {
  if (num === 0) return null;
  const mod = num % 3;
  if (mod === 1) return 1;
  if (mod === 2) return 2;
  return 3;
}

/**
 * Check if number is in low range (1-18)
 * @param {number} num
 * @returns {boolean}
 */
export function isLow(num) {
  return num >= 1 && num <= 18;
}

/**
 * Check if number is in high range (19-36)
 * @param {number} num
 * @returns {boolean}
 */
export function isHigh(num) {
  return num >= 19 && num <= 36;
}

/**
 * Check if number is odd
 * @param {number} num
 * @returns {boolean}
 */
export function isOdd(num) {
  return num !== 0 && num % 2 === 1;
}

/**
 * Check if number is even
 * @param {number} num
 * @returns {boolean}
 */
export function isEven(num) {
  return num !== 0 && num % 2 === 0;
}

/**
 * Format amount for display (assumes microunits)
 * @param {string|number} amount - Amount in microunits
 * @returns {string} Formatted amount
 */
export function formatAmount(amount) {
  const num = typeof amount === 'string' ? parseInt(amount) : amount;
  if (num >= 1_000_000_000) {
    return `${(num / 1_000_000_000).toFixed(2)}B`;
  }
  if (num >= 1_000_000) {
    return `${(num / 1_000_000).toFixed(2)}M`;
  }
  if (num >= 1_000) {
    return `${(num / 1_000).toFixed(2)}K`;
  }
  return num.toString();
}

// Payout multipliers for each bet type
const PAYOUT_MULTIPLIERS = {
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

/**
 * Get the payout multiplier for a bet type
 * @param {string} betType - The type of bet
 * @returns {number} Payout multiplier
 */
export function getPayoutMultiplier(betType) {
  return PAYOUT_MULTIPLIERS[betType] || 0;
}

/**
 * Check if a bet wins for a given result
 * @param {object} bet - The bet object with type, number, column, dozen, sixLineStart
 * @param {number} result - The winning number (0-36)
 * @returns {boolean}
 */
export function checkBetWin(bet, result) {
  const num = typeof result === 'object' ? result.result || result : result;

  switch (bet.type) {
    case 'straight':
      return bet.number === num;
    case 'red':
      return RED_NUMBERS.includes(num);
    case 'black':
      return BLACK_NUMBERS.includes(num);
    case 'odd':
      return isOdd(num);
    case 'even':
      return isEven(num);
    case 'low':
      return isLow(num);
    case 'high':
      return isHigh(num);
    case 'dozen':
      return getDozen(num) === bet.dozen;
    case 'column':
      return getColumn(num) === bet.column;
    case 'sixline':
    case 'sixLine':
      // SixLine covers 6 consecutive numbers starting from bet.sixLineStart
      // e.g., sixLineStart=1 covers 1,2,3,4,5,6
      const start = bet.sixLineStart || bet.number;
      return num >= start && num <= start + 5;
    default:
      return false;
  }
}

/**
 * Get valid six-line starting numbers
 * Six-line bets start at 1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31
 * @returns {number[]}
 */
export function getSixLineStarts() {
  const starts = [];
  for (let i = 1; i <= 31; i += 3) {
    starts.push(i);
  }
  return starts;
}

/**
 * Validate a bet configuration
 * @param {object} bet - Bet object with type, number, amount, etc.
 * @returns {{valid: boolean, error?: string}}
 */
export function validateBet(bet) {
  if (!bet || typeof bet !== 'object') {
    return { valid: false, error: 'Invalid bet object' };
  }

  const { type, amount } = bet;

  // Check bet type
  if (!type) {
    return { valid: false, error: 'Bet type is required' };
  }

  // Check amount
  if (!amount || amount <= 0) {
    return { valid: false, error: 'Bet amount must be greater than zero' };
  }

  // Validate based on bet type
  switch (type.toLowerCase()) {
    case 'straight':
      if (bet.number === undefined || bet.number === null) {
        return { valid: false, error: 'Number is required for straight bets' };
      }
      if (bet.number < 0 || bet.number > 36) {
        return { valid: false, error: 'Number must be between 0 and 36' };
      }
      break;

    case 'dozen':
      if (!bet.dozen || bet.dozen < 1 || bet.dozen > 3) {
        return { valid: false, error: 'Dozen must be 1, 2, or 3' };
      }
      break;

    case 'column':
      if (!bet.column || bet.column < 1 || bet.column > 3) {
        return { valid: false, error: 'Column must be 1, 2, or 3' };
      }
      break;

    case 'sixline':
    case 'sixLine':
      const start = bet.sixLineStart || bet.number;
      if (!start) {
        return { valid: false, error: 'Six-line start number is required' };
      }
      const validStarts = getSixLineStarts();
      if (!validStarts.includes(start)) {
        return { valid: false, error: `Six-line must start at: ${validStarts.join(', ')}` };
      }
      break;

    case 'split':
      if (!bet.numbers || !Array.isArray(bet.numbers) || bet.numbers.length !== 2) {
        return { valid: false, error: 'Split bets require exactly 2 adjacent numbers' };
      }
      // Check adjacency (simplified - would need full board layout)
      const diff = Math.abs(bet.numbers[0] - bet.numbers[1]);
      if (diff !== 1 && diff !== 3) {
        return { valid: false, error: 'Split numbers must be adjacent on board' };
      }
      break;

    case 'street':
      if (!bet.number) {
        return { valid: false, error: 'Street start number is required' };
      }
      // Streets start at 1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34
      if ((bet.number - 1) % 3 !== 0 || bet.number < 1 || bet.number > 34) {
        return { valid: false, error: 'Invalid street start number' };
      }
      break;

    case 'corner':
      if (!bet.numbers || !Array.isArray(bet.numbers) || bet.numbers.length !== 4) {
        return { valid: false, error: 'Corner bets require exactly 4 numbers' };
      }
      break;

    case 'red':
    case 'black':
    case 'odd':
    case 'even':
    case 'low':
    case 'high':
      // These are valid simple bets with no additional parameters
      break;

    default:
      return { valid: false, error: `Unknown bet type: ${type}` };
  }

  return { valid: true };
}
