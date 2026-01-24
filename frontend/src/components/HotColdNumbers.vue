<!-- src/components/HotColdNumbers.vue -->
<template>
  <div class="hot-cold-numbers bg-black/30 backdrop-blur-sm rounded-xl p-4 border border-green-800/50">
    <div class="grid grid-cols-2 gap-4">
      <!-- Hot Numbers -->
      <div>
        <h3 class="text-sm font-medium text-gray-400 mb-2 flex items-center space-x-1">
          <span>🔥</span>
          <span>Hot Numbers</span>
        </h3>

        <div v-if="hotNumbers && hotNumbers.length > 0" class="flex flex-wrap gap-1">
          <button
            v-for="num in hotNumbers"
            :key="`hot-${num}`"
            @click="handleNumberClick(num)"
            class="number-badge hover:scale-110 transition-transform cursor-pointer"
            :class="getNumberColorClass(num)"
            :title="`Bet on ${num}`"
            :aria-label="`Place bet on number ${num}`"
          >
            {{ num }}
          </button>
        </div>
        <div v-else class="text-gray-600 text-sm italic">
          No data yet
        </div>
      </div>

      <!-- Cold Numbers -->
      <div>
        <h3 class="text-sm font-medium text-gray-400 mb-2 flex items-center space-x-1">
          <span>❄️</span>
          <span>Cold Numbers</span>
        </h3>

        <div v-if="coldNumbers && coldNumbers.length > 0" class="flex flex-wrap gap-1">
          <button
            v-for="num in coldNumbers"
            :key="`cold-${num}`"
            @click="handleNumberClick(num)"
            class="number-badge opacity-60 hover:opacity-100 hover:scale-110 transition-all cursor-pointer"
            :class="getNumberColorClass(num)"
            :title="`Bet on ${num}`"
            :aria-label="`Place bet on number ${num}`"
          >
            {{ num }}
          </button>
        </div>
        <div v-else class="text-gray-600 text-sm italic">
          No data yet
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { getNumberColor } from '../utils/roulette.js';

defineProps({
  hotNumbers: {
    type: Array,
    default: () => []
  },
  coldNumbers: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['place-bet']);

// Get CSS class based on number color
function getNumberColorClass(num) {
  const color = getNumberColor(num);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
}

// Handle number click - place straight bet
function handleNumberClick(number) {
  emit('place-bet', { betType: 'Straight', number });
}
</script>

<style scoped>
.number-badge {
  @apply w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold;
}
</style>
