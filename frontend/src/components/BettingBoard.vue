<!-- src/components/BettingBoard.vue -->
<template>
  <div class="betting-board bg-felt-green rounded-xl p-4 shadow-2xl border-4 border-amber-800 overflow-x-auto">
    <div class="grid gap-1 min-w-max">
      <!-- Main betting area: 0 + Numbers + Columns -->
      <div class="flex gap-1">
        <!-- Zero (spans 3 rows) -->
        <div class="flex-shrink-0">
          <button
            @click="placeBet('Straight', 0)"
            @contextmenu.prevent="removeBet('Straight', 0)"
            class="roulette-number green h-full"
            :class="{ 'ring-2 ring-yellow-400': hasBetOn('Straight', 0) }"
            style="min-height: 100px; width: 40px;"
          >
            0
          </button>
        </div>

        <!-- Numbers grid (3 rows x 12 columns) with split bet overlays -->
        <div class="flex-1 relative">
          <!-- Row 3 (top): 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36 -->
          <div class="flex gap-1 mb-1 relative">
            <button
              v-for="n in row3Numbers"
              :key="n"
              @click="placeBet('Straight', n)"
              @contextmenu.prevent="removeBet('Straight', n)"
              class="roulette-number flex-1"
              :class="[
                getNumberColorClass(n),
                { 'ring-2 ring-yellow-400': hasBetOn('Straight', n) }
              ]"
            >
              {{ n }}
            </button>
            <!-- Horizontal split bets for row 3 -->
            <button
              v-for="split in horizontalSplitsRow3"
              :key="split.key"
              @click="placeSplitBet(split.numbers)"
              @contextmenu.prevent="removeSplitBet(split.numbers)"
              class="split-bet-horizontal"
              :class="{ 'ring-2 ring-yellow-400': hasSplitBetOn(split.numbers) }"
              :style="{ left: `calc(${(split.numbers[0] / 3 - 0.5) * (100 / 12)}% - 6px)` }"
              :title="`Split: ${split.numbers.join('-')} (17:1)`"
            ></button>
          </div>

          <!-- Vertical split bets between row 3 and row 2 -->
          <div class="flex gap-1 mb-1 relative" style="height: 0; margin-bottom: -4px;">
            <button
              v-for="split in verticalSplitsRow2Row3"
              :key="split.key"
              @click="placeSplitBet(split.numbers)"
              @contextmenu.prevent="removeSplitBet(split.numbers)"
              class="split-bet-vertical"
              :class="{ 'ring-2 ring-yellow-400': hasSplitBetOn(split.numbers) }"
              :style="{ left: `calc(${((split.numbers[1] / 3 - 1) * (100 / 12)) + (100 / 24)}% - 6px)` }"
              :title="`Split: ${split.numbers.join('-')} (17:1)`"
            ></button>
          </div>

          <!-- Row 2 (middle): 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35 -->
          <div class="flex gap-1 mb-1 relative">
            <button
              v-for="n in row2Numbers"
              :key="n"
              @click="placeBet('Straight', n)"
              @contextmenu.prevent="removeBet('Straight', n)"
              class="roulette-number flex-1"
              :class="[
                getNumberColorClass(n),
                { 'ring-2 ring-yellow-400': hasBetOn('Straight', n) }
              ]"
            >
              {{ n }}
            </button>
            <!-- Horizontal split bets for row 2 -->
            <button
              v-for="split in horizontalSplitsRow2"
              :key="split.key"
              @click="placeSplitBet(split.numbers)"
              @contextmenu.prevent="removeSplitBet(split.numbers)"
              class="split-bet-horizontal"
              :class="{ 'ring-2 ring-yellow-400': hasSplitBetOn(split.numbers) }"
              :style="{ left: `calc(${((split.numbers[0] - 2) / 3 + 0.5) * (100 / 12)}% - 6px)` }"
              :title="`Split: ${split.numbers.join('-')} (17:1)`"
            ></button>
          </div>

          <!-- Vertical split bets between row 2 and row 1 -->
          <div class="flex gap-1 mb-1 relative" style="height: 0; margin-bottom: -4px;">
            <button
              v-for="split in verticalSplitsRow1Row2"
              :key="split.key"
              @click="placeSplitBet(split.numbers)"
              @contextmenu.prevent="removeSplitBet(split.numbers)"
              class="split-bet-vertical"
              :class="{ 'ring-2 ring-yellow-400': hasSplitBetOn(split.numbers) }"
              :style="{ left: `calc(${((split.numbers[0] - 1) / 3) * (100 / 12) + (100 / 24)}% - 6px)` }"
              :title="`Split: ${split.numbers.join('-')} (17:1)`"
            ></button>
          </div>

          <!-- Row 1 (bottom): 1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34 -->
          <div class="flex gap-1 relative">
            <button
              v-for="n in row1Numbers"
              :key="n"
              @click="placeBet('Straight', n)"
              @contextmenu.prevent="removeBet('Straight', n)"
              class="roulette-number flex-1"
              :class="[
                getNumberColorClass(n),
                { 'ring-2 ring-yellow-400': hasBetOn('Straight', n) }
              ]"
            >
              {{ n }}
            </button>
            <!-- Horizontal split bets for row 1 -->
            <button
              v-for="split in horizontalSplitsRow1"
              :key="split.key"
              @click="placeSplitBet(split.numbers)"
              @contextmenu.prevent="removeSplitBet(split.numbers)"
              class="split-bet-horizontal"
              :class="{ 'ring-2 ring-yellow-400': hasSplitBetOn(split.numbers) }"
              :style="{ left: `calc(${((split.numbers[0] - 1) / 3 + 0.5) * (100 / 12)}% - 6px)` }"
              :title="`Split: ${split.numbers.join('-')} (17:1)`"
            ></button>
          </div>
        </div>

        <!-- Column bets (2:1) -->
        <div class="flex-shrink-0 flex flex-col gap-1">
          <button
            @click="placeBet('Column', 3)"
            @contextmenu.prevent="removeBet('Column', 3)"
            class="betting-zone flex-1"
            :class="{ 'ring-2 ring-yellow-400': hasBetOn('Column', 3) }"
          >
            2:1
          </button>
          <button
            @click="placeBet('Column', 2)"
            @contextmenu.prevent="removeBet('Column', 2)"
            class="betting-zone flex-1"
            :class="{ 'ring-2 ring-yellow-400': hasBetOn('Column', 2) }"
          >
            2:1
          </button>
          <button
            @click="placeBet('Column', 1)"
            @contextmenu.prevent="removeBet('Column', 1)"
            class="betting-zone flex-1"
            :class="{ 'ring-2 ring-yellow-400': hasBetOn('Column', 1) }"
          >
            2:1
          </button>
        </div>
      </div>

      <!-- Dozen bets -->
      <div class="flex gap-1 mt-2">
        <div class="w-10"></div> <!-- Spacer for 0 -->
        <button
          @click="placeBet('Dozen', 1)"
          @contextmenu.prevent="removeBet('Dozen', 1)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Dozen', 1) }"
        >
          1st 12
        </button>
        <button
          @click="placeBet('Dozen', 2)"
          @contextmenu.prevent="removeBet('Dozen', 2)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Dozen', 2) }"
        >
          2nd 12
        </button>
        <button
          @click="placeBet('Dozen', 3)"
          @contextmenu.prevent="removeBet('Dozen', 3)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Dozen', 3) }"
        >
          3rd 12
        </button>
        <div class="w-10"></div> <!-- Spacer for columns -->
      </div>

      <!-- Outside bets: 1-18, Even, Red, Black, Odd, 19-36 -->
      <div class="flex gap-1 mt-1">
        <div class="w-10"></div> <!-- Spacer for 0 -->
        <button
          @click="placeBet('Low', null)"
          @contextmenu.prevent="removeBet('Low', null)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Low', null) }"
        >
          1-18
        </button>
        <button
          @click="placeBet('Even', null)"
          @contextmenu.prevent="removeBet('Even', null)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Even', null) }"
        >
          EVEN
        </button>
        <button
          @click="placeBet('Red', null)"
          @contextmenu.prevent="removeBet('Red', null)"
          class="betting-zone flex-1 !bg-roulette-red"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Red', null) }"
        >
          RED
        </button>
        <button
          @click="placeBet('Black', null)"
          @contextmenu.prevent="removeBet('Black', null)"
          class="betting-zone flex-1 !bg-roulette-black"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Black', null) }"
        >
          BLACK
        </button>
        <button
          @click="placeBet('Odd', null)"
          @contextmenu.prevent="removeBet('Odd', null)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('Odd', null) }"
        >
          ODD
        </button>
        <button
          @click="placeBet('High', null)"
          @contextmenu.prevent="removeBet('High', null)"
          class="betting-zone flex-1"
          :class="{ 'ring-2 ring-yellow-400': hasBetOn('High', null) }"
        >
          19-36
        </button>
        <div class="w-10"></div> <!-- Spacer for columns -->
      </div>
    </div>

    <!-- Bet chips display overlay -->
    <div
      v-for="bet in currentBets"
      :key="`${bet.betType}-${bet.number}`"
      class="bet-chip-indicator"
      :style="getBetChipPosition(bet)"
    >
      <span class="text-xs font-bold">{{ formatBetAmount(bet.amount) }}</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { getNumberColor, formatAmount } from '../utils/roulette.js';

const props = defineProps({
  currentBets: {
    type: Array,
    default: () => []
  },
  disabled: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['placeBet', 'removeBet']);

// Number rows (standard roulette layout)
// Row 1 (bottom): 1, 4, 7, 10, 13, 16, 19, 22, 25, 28, 31, 34
const row1Numbers = computed(() => {
  const nums = [];
  for (let i = 1; i <= 34; i += 3) {
    nums.push(i);
  }
  return nums;
});

// Row 2 (middle): 2, 5, 8, 11, 14, 17, 20, 23, 26, 29, 32, 35
const row2Numbers = computed(() => {
  const nums = [];
  for (let i = 2; i <= 35; i += 3) {
    nums.push(i);
  }
  return nums;
});

// Row 3 (top): 3, 6, 9, 12, 15, 18, 21, 24, 27, 30, 33, 36
const row3Numbers = computed(() => {
  const nums = [];
  for (let i = 3; i <= 36; i += 3) {
    nums.push(i);
  }
  return nums;
});

// Helper function to get horizontal split bets (adjacent in same row)
function getHorizontalSplits(numbers) {
  const splits = [];
  for (let i = 0; i < numbers.length - 1; i++) {
    splits.push({
      numbers: [numbers[i], numbers[i + 1]],
      key: `h-${numbers[i]}-${numbers[i + 1]}`
    });
  }
  return splits;
}

// Helper function to get vertical split bets (adjacent vertically)
// In roulette layout: numbers differ by 3 when vertically adjacent
function getVerticalSplits(row1, row2) {
  const splits = [];
  for (let i = 0; i < row1.length; i++) {
    splits.push({
      numbers: [row1[i], row2[i]],
      key: `v-${row1[i]}-${row2[i]}`
    });
  }
  return splits;
}

// Computed split bet positions
const horizontalSplitsRow1 = computed(() => getHorizontalSplits(row1Numbers.value));
const horizontalSplitsRow2 = computed(() => getHorizontalSplits(row2Numbers.value));
const horizontalSplitsRow3 = computed(() => getHorizontalSplits(row3Numbers.value));

const verticalSplitsRow1Row2 = computed(() => getVerticalSplits(row1Numbers.value, row2Numbers.value));
const verticalSplitsRow2Row3 = computed(() => getVerticalSplits(row2Numbers.value, row3Numbers.value));

// Get CSS class for number color
function getNumberColorClass(num) {
  const color = getNumberColor(num);
  return color; // Uses CSS classes: .red, .black, .green
}

// Place bet handler
function placeBet(betType, number) {
  if (props.disabled) return;
  emit('placeBet', { betType, number });
}

// Place split bet handler
function placeSplitBet(numbers) {
  if (props.disabled) return;
  emit('placeBet', { betType: 'Split', numbers });
}

// Remove bet handler
function removeBet(betType, number) {
  if (props.disabled) return;
  emit('removeBet', { betType, number });
}

// Remove split bet handler
function removeSplitBet(numbers) {
  if (props.disabled) return;
  emit('removeBet', { betType: 'Split', numbers });
}

// Check if there's a bet on this position
function hasBetOn(betType, number) {
  const normalizedType = betType.toLowerCase();
  return props.currentBets.some(bet => {
    // Handle different bet types
    if (normalizedType === 'straight') {
      return bet.type === 'straight' && bet.number === number;
    } else if (normalizedType === 'dozen') {
      return bet.type === 'dozen' && bet.dozen === number;
    } else if (normalizedType === 'column') {
      return bet.type === 'column' && bet.column === number;
    } else {
      // For outside bets (red, black, odd, even, low, high)
      return bet.type === normalizedType;
    }
  });
}

// Check if there's a split bet on these numbers
function hasSplitBetOn(numbers) {
  return props.currentBets.some(bet => {
    if (bet.type !== 'split' || !bet.numbers || !Array.isArray(bet.numbers)) {
      return false;
    }
    // Check if arrays contain the same numbers (order doesn't matter)
    const sortedBet = [...bet.numbers].sort((a, b) => a - b);
    const sortedCheck = [...numbers].sort((a, b) => a - b);
    return sortedBet.length === sortedCheck.length &&
           sortedBet.every((num, idx) => num === sortedCheck[idx]);
  });
}

// Format bet amount for display
function formatBetAmount(amount) {
  return formatAmount(amount);
}

// Get position for bet chip indicator (simplified - for overlay display)
function getBetChipPosition(bet) {
  // This would calculate actual position based on bet type and number
  // For now, return hidden (actual positioning requires DOM measurements)
  return { display: 'none' };
}
</script>

<style scoped>
.betting-zone {
  @apply px-2 py-3 bg-felt-green border-2 border-amber-700 rounded text-white font-bold text-sm;
  @apply hover:bg-green-700 transition-colors cursor-pointer;
  min-width: 50px;
  min-height: 44px;
}

.betting-zone:hover {
  @apply brightness-110;
}

.bet-chip-indicator {
  @apply absolute w-6 h-6 rounded-full bg-yellow-400 text-black flex items-center justify-center;
  @apply shadow-lg pointer-events-none;
}

/* Split bet buttons - horizontal (between adjacent numbers in same row) */
.split-bet-horizontal {
  @apply absolute w-3 h-10 bg-amber-600 bg-opacity-30 hover:bg-opacity-60 transition-all;
  @apply border border-amber-500 rounded-sm cursor-pointer z-10;
  top: 50%;
  transform: translateY(-50%);
}

.split-bet-horizontal:hover {
  @apply bg-opacity-80 scale-110;
}

/* Split bet buttons - vertical (between adjacent numbers in different rows) */
.split-bet-vertical {
  @apply absolute h-3 bg-amber-600 bg-opacity-30 hover:bg-opacity-60 transition-all;
  @apply border border-amber-500 rounded-sm cursor-pointer z-10;
  width: calc(100% / 12 - 4px);
  top: 0;
  transform: translateY(-50%);
}

.split-bet-vertical:hover {
  @apply bg-opacity-80 scale-110;
}
</style>
