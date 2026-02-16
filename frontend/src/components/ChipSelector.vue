<!-- src/components/ChipSelector.vue -->
<template>
  <div class="card">
    <h3 class="text-sm font-semibold text-text-muted mb-4 flex items-center gap-2">
      <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      Select Chip Value
    </h3>

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
    <div class="mt-4 text-center">
      <span class="inline-flex items-center gap-2 px-4 py-2 bg-bg-main rounded-lg">
        <span class="text-text-muted text-sm">Selected:</span>
        <span class="font-bold font-mono text-primary text-lg">{{ selectedChip }}</span>
        <span class="text-text-dim text-sm">chips</span>
      </span>
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

const chipDefs = [
  { value: 1, label: '1', colorClass: 'chip-white' },
  { value: 5, label: '5', colorClass: 'chip-red' },
  { value: 10, label: '10', colorClass: 'chip-blue' },
  { value: 25, label: '25', colorClass: 'chip-green' },
  { value: 100, label: '100', colorClass: 'chip-black' },
  { value: 500, label: '500', colorClass: 'chip-purple' },
];

const chips = computed(() => {
  return chipDefs.map(chip => ({
    ...chip,
    disabled: chip.value > props.balance
  }));
});

function selectChip(value) {
  if (value > props.balance) return;
  emit('select', value);
}
</script>

<style scoped>
.chip {
  @apply w-14 h-14 rounded-full flex items-center justify-center font-bold cursor-pointer;
  @apply border-4 border-white/80 shadow-lg text-sm;
  @apply transition-all duration-150 ease-out;
}

@media (hover: hover) {
  .chip:hover:not(.chip-disabled) {
    @apply scale-110;
  }
}

.chip.selected {
  @apply ring-4 ring-primary ring-offset-2 ring-offset-bg-main shadow-lg;
}

/* Chip colors */
.chip-white {
  @apply bg-slate-100 text-slate-900 border-slate-300;
}

.chip-red {
  @apply bg-red-500 text-white border-red-300;
}

.chip-blue {
  @apply bg-blue-500 text-white border-blue-300;
}

.chip-green {
  @apply bg-emerald-500 text-white border-emerald-300;
}

.chip-black {
  @apply bg-slate-800 text-white border-slate-500;
}

.chip-purple {
  @apply bg-violet-600 text-white border-violet-400;
}

.chip-disabled {
  @apply opacity-30 cursor-not-allowed;
}

.chip-disabled:hover {
  @apply transform-none;
}
</style>
