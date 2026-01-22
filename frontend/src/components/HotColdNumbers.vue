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
          <span
            v-for="num in hotNumbers"
            :key="`hot-${num}`"
            class="number-badge"
            :class="getNumberColorClass(num)"
          >
            {{ num }}
          </span>
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
          <span
            v-for="num in coldNumbers"
            :key="`cold-${num}`"
            class="number-badge opacity-60"
            :class="getNumberColorClass(num)"
          >
            {{ num }}
          </span>
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

// Get CSS class based on number color
function getNumberColorClass(num) {
  const color = getNumberColor(num);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
}
</script>

<style scoped>
.number-badge {
  @apply w-7 h-7 rounded-full flex items-center justify-center text-xs font-bold;
}
</style>
