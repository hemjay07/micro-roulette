<!-- src/components/SpinHistory.vue -->
<template>
  <div class="card">
    <h3 class="text-sm font-semibold text-text-muted mb-4 flex items-center gap-2">
      <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      Recent Spins
    </h3>

    <!-- Empty state -->
    <div v-if="historyData.length === 0" class="text-center py-8">
      <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-bg-elevated flex items-center justify-center">
        <svg class="w-8 h-8 text-text-dim" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
      </div>
      <p class="text-text-muted font-medium">No spins yet</p>
      <p class="text-text-dim text-sm mt-1">Place a bet and spin!</p>
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
        <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-2 bg-bg-elevated border border-border rounded-lg text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity duration-150 pointer-events-none z-10 shadow-lg">
          <div class="text-text-primary font-medium">Spin #{{ spin.spinId }}</div>
          <div v-if="spin.seedHash" class="text-text-dim text-[10px] font-mono truncate max-w-[120px] mt-1">
            {{ spin.seedHash.slice(0, 8) }}...
          </div>
          <div class="text-primary text-[10px] mt-1 font-medium">Click to verify</div>
          <!-- Arrow -->
          <div class="absolute top-full left-1/2 -translate-x-1/2 border-4 border-transparent border-t-bg-elevated"></div>
        </div>
      </button>
    </div>

    <!-- Statistics -->
    <div v-if="historyData.length > 0" class="mt-4 pt-4 border-t border-border">
      <div class="grid grid-cols-3 gap-3 text-center">
        <div class="bg-bg-main rounded-lg p-2">
          <div class="text-text-dim text-xs">Red</div>
          <div class="font-bold text-roulette-red text-lg">{{ redCount }}</div>
        </div>
        <div class="bg-bg-main rounded-lg p-2">
          <div class="text-text-dim text-xs">Black</div>
          <div class="font-bold text-text-primary text-lg">{{ blackCount }}</div>
        </div>
        <div class="bg-bg-main rounded-lg p-2">
          <div class="text-text-dim text-xs">Green</div>
          <div class="font-bold text-roulette-green text-lg">{{ greenCount }}</div>
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

const historyData = toRef(props, 'history');

function handleSpinClick(spin) {
  emit('verify-spin', spin);
}

function getResultColorClass(result) {
  const color = getNumberColor(result);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
}

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
  @apply w-10 h-10 rounded-full flex items-center justify-center text-sm font-bold;
  @apply cursor-pointer transition-all duration-150;
  @apply hover:scale-110 hover:shadow-lg;
}
</style>
