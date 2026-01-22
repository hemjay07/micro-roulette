<!-- src/components/ChipSelector.vue -->
<template>
  <div class="chip-selector bg-black/30 backdrop-blur-sm rounded-xl p-4 border border-green-800/50">
    <h3 class="text-sm font-medium text-gray-400 mb-3">Select Chip Value</h3>

    <div class="flex flex-wrap gap-3 justify-center">
      <button
        v-for="chip in chips"
        :key="chip.value"
        @click="selectChip(chip.value)"
        class="chip"
        :class="[
          chip.colorClass,
          { 'selected': selectedChip === chip.value },
          { 'chip-disabled': chip.disabled }
        ]"
        :disabled="chip.disabled"
        :title="chip.disabled ? `Insufficient balance for ${chip.value} chip` : `${chip.value} chip`"
      >
        {{ chip.label }}
      </button>
    </div>

    <!-- Current selection indicator -->
    <div class="mt-3 text-center text-sm text-gray-400">
      Current: <span class="font-bold text-white">{{ selectedChip }}</span>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  selectedChip: {
    type: Number,
    default: 10
  },
  balance: {
    type: Number,
    default: Infinity
  }
});

const emit = defineEmits(['select']);

// Chip definitions with colors
const chipDefs = [
  { value: 1, label: '1', colorClass: 'chip-white' },
  { value: 5, label: '5', colorClass: 'chip-red' },
  { value: 10, label: '10', colorClass: 'chip-blue' },
  { value: 25, label: '25', colorClass: 'chip-green' },
  { value: 100, label: '100', colorClass: 'chip-black' },
  { value: 500, label: '500', colorClass: 'chip-purple' },
];

// Computed chips with disabled state based on balance
const chips = computed(() => {
  return chipDefs.map(chip => ({
    ...chip,
    disabled: chip.value > props.balance
  }));
});

function selectChip(value) {
  // Don't allow selecting chips that exceed balance
  if (value > props.balance) return;
  emit('select', value);
}
</script>

<style scoped>
.chip {
  @apply w-12 h-12 rounded-full flex items-center justify-center font-bold cursor-pointer transition-all;
  @apply border-4 border-white shadow-lg text-sm;
}

.chip:hover {
  @apply transform scale-110;
}

.chip.selected {
  @apply ring-4 ring-yellow-400 ring-offset-2 ring-offset-transparent;
}

/* Chip colors */
.chip-white {
  @apply bg-gray-100 text-gray-900 border-gray-300;
}

.chip-red {
  @apply bg-red-600 text-white border-red-300;
}

.chip-blue {
  @apply bg-blue-600 text-white border-blue-300;
}

.chip-green {
  @apply bg-green-600 text-white border-green-300;
}

.chip-black {
  @apply bg-gray-900 text-white border-gray-600;
}

.chip-purple {
  @apply bg-purple-600 text-white border-purple-300;
}

.chip-disabled {
  @apply opacity-40 cursor-not-allowed;
}

.chip-disabled:hover {
  @apply transform-none;
}
</style>
