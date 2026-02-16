<!-- src/components/HotColdNumbers.vue -->
<template>
  <div class="card">
    <div class="grid grid-cols-2 gap-4">
      <!-- Hot Numbers -->
      <div>
        <h3 class="text-sm font-semibold text-text-muted mb-3 flex items-center gap-2">
          <span class="w-5 h-5 rounded-full bg-error/20 flex items-center justify-center">
            <svg class="w-3 h-3 text-error" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12.395 2.553a1 1 0 00-1.45-.385c-.345.23-.614.558-.822.88-.214.33-.403.713-.57 1.116-.334.804-.614 1.768-.84 2.734a31.365 31.365 0 00-.613 3.58 2.64 2.64 0 01-.945-1.067c-.328-.68-.398-1.534-.398-2.654A1 1 0 005.05 6.05 6.981 6.981 0 003 11a7 7 0 1011.95-4.95c-.592-.591-.98-.985-1.348-1.467-.363-.476-.724-1.063-1.207-2.03zM12.12 15.12A3 3 0 017 13s.879.5 2.5.5c0-1 .5-4 1.25-4.5.5 1 .786 1.293 1.371 1.879A2.99 2.99 0 0113 13a2.99 2.99 0 01-.879 2.121z" clip-rule="evenodd" />
            </svg>
          </span>
          Hot Numbers
        </h3>

        <div v-if="hotNumbers && hotNumbers.length > 0" class="flex flex-wrap gap-2">
          <button
            v-for="num in hotNumbers"
            :key="`hot-${num}`"
            @click="handleNumberClick(num)"
            class="number-badge"
            :class="getNumberColorClass(num)"
            :title="`Bet on ${num}`"
            :aria-label="`Place bet on number ${num}`"
          >
            {{ num }}
          </button>
        </div>
        <div v-else class="text-text-dim text-sm py-4 text-center bg-bg-main rounded-lg">
          No data yet
        </div>
      </div>

      <!-- Cold Numbers -->
      <div>
        <h3 class="text-sm font-semibold text-text-muted mb-3 flex items-center gap-2">
          <span class="w-5 h-5 rounded-full bg-blue-500/20 flex items-center justify-center">
            <svg class="w-3 h-3 text-blue-400" fill="currentColor" viewBox="0 0 20 20">
              <path d="M10 2a1 1 0 011 1v1.323l3.954 1.582 1.599-.8a1 1 0 01.894 1.79l-1.233.616 1.738 5.42a1 1 0 01-.285 1.05A3.989 3.989 0 0115 15a3.989 3.989 0 01-2.667-1.019 1 1 0 01-.285-1.05l1.715-5.349L10 6.015 6.237 7.582l1.715 5.349a1 1 0 01-.285 1.05A3.989 3.989 0 015 15a3.989 3.989 0 01-2.667-1.019 1 1 0 01-.285-1.05l1.738-5.42-1.233-.617a1 1 0 01.894-1.788l1.599.799L9 4.323V3a1 1 0 011-1z" />
            </svg>
          </span>
          Cold Numbers
        </h3>

        <div v-if="coldNumbers && coldNumbers.length > 0" class="flex flex-wrap gap-2">
          <button
            v-for="num in coldNumbers"
            :key="`cold-${num}`"
            @click="handleNumberClick(num)"
            class="number-badge opacity-60 hover:opacity-100"
            :class="getNumberColorClass(num)"
            :title="`Bet on ${num}`"
            :aria-label="`Place bet on number ${num}`"
          >
            {{ num }}
          </button>
        </div>
        <div v-else class="text-text-dim text-sm py-4 text-center bg-bg-main rounded-lg">
          No data yet
        </div>
      </div>
    </div>

    <!-- Helper text -->
    <p class="text-xs text-text-dim mt-4 text-center">
      Click any number to place a straight bet
    </p>
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

function getNumberColorClass(num) {
  const color = getNumberColor(num);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
}

function handleNumberClick(number) {
  emit('place-bet', { betType: 'Straight', number });
}
</script>

<style scoped>
.number-badge {
  @apply w-8 h-8 rounded-full flex items-center justify-center text-xs font-bold;
  @apply cursor-pointer transition-all duration-150;
  @apply hover:scale-110 hover:shadow-lg;
}
</style>
