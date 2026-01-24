<!-- src/components/SpinHistory.vue -->
<template>
  <div class="spin-history bg-black/30 backdrop-blur-sm rounded-xl p-4 border border-green-800/50">
    <h3 class="text-sm font-medium text-gray-400 mb-3">Recent Spins</h3>

    <!-- Empty state -->
    <div v-if="historyData.length === 0" class="text-center py-6">
      <svg class="w-12 h-12 mx-auto text-gray-600 mb-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <p class="text-gray-500">No spins yet</p>
      <p class="text-gray-600 text-sm">Place a bet and spin!</p>
    </div>

    <!-- History list -->
    <div v-else class="flex flex-wrap gap-2">
      <button
        v-for="spin in historyData"
        :key="spin.spinId"
        @click="handleSpinClick(spin)"
        class="spin-result group relative"
        :class="getResultColorClass(spin.result)"
        :title="`Spin #${spin.spinId} - Click to verify`"
        :aria-label="`Verify spin #${spin.spinId}, result ${spin.result}`"
      >
        <span class="font-bold">{{ spin.result }}</span>

        <!-- Tooltip on hover -->
        <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-2 py-1 bg-black rounded text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none z-10">
          <div class="text-gray-300">Spin #{{ spin.spinId }}</div>
          <div v-if="spin.seedHash" class="text-gray-500 text-[10px] font-mono truncate max-w-[120px]">
            {{ spin.seedHash.slice(0, 8) }}...
          </div>
          <div class="text-green-400 text-[10px] mt-1">Click to verify</div>
          <!-- Arrow -->
          <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-black"></div>
        </div>
      </button>
    </div>

    <!-- Statistics -->
    <div v-if="historyData.length > 0" class="mt-4 pt-3 border-t border-gray-800">
      <div class="grid grid-cols-3 gap-2 text-center text-xs">
        <div>
          <div class="text-gray-500">Red</div>
          <div class="font-bold text-roulette-red">{{ redCount }}</div>
        </div>
        <div>
          <div class="text-gray-500">Black</div>
          <div class="font-bold text-gray-300">{{ blackCount }}</div>
        </div>
        <div>
          <div class="text-gray-500">Green</div>
          <div class="font-bold text-roulette-green">{{ greenCount }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, toRef } from 'vue';
import { getNumberColor } from '../utils/roulette.js';

const props = defineProps({
  history: {
    type: Array,
    default: () => []
  }
});

const emit = defineEmits(['verify-spin']);

// Use toRef to maintain reactivity from props
const historyData = toRef(props, 'history');

// Handle spin click
function handleSpinClick(spin) {
  emit('verify-spin', spin);
}

// Get CSS class based on result color
function getResultColorClass(result) {
  const color = getNumberColor(result);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
}

// Count statistics
const redCount = computed(() => {
  return historyData.value.filter(s => getNumberColor(s.result) === 'red').length;
});

const blackCount = computed(() => {
  return historyData.value.filter(s => getNumberColor(s.result) === 'black').length;
});

const greenCount = computed(() => {
  return historyData.value.filter(s => getNumberColor(s.result) === 'green').length;
});
</script>

<style scoped>
.spin-result {
  @apply w-9 h-9 rounded-full flex items-center justify-center text-sm font-bold;
  @apply cursor-pointer transition-transform hover:scale-110;
}
</style>
