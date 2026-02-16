<!-- src/components/Header.vue -->
<template>
  <header class="bg-bg-surface/80 backdrop-blur-safe border-b border-border px-4 py-4">
    <div class="container mx-auto flex items-center justify-between">
      <!-- Logo and Title -->
      <div class="flex items-center gap-3">
        <div class="w-12 h-12 bg-gradient-primary rounded-xl flex items-center justify-center shadow-md">
          <svg class="w-7 h-7 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="10" />
            <path d="M12 2a10 10 0 0 1 0 20" />
            <circle cx="12" cy="12" r="3" />
          </svg>
        </div>
        <div>
          <h1 class="text-2xl font-bold text-text-primary">MicroRoulette</h1>
          <p class="text-sm text-primary">Every Spin On-Chain</p>
        </div>
      </div>

      <!-- Balance Display -->
      <div class="flex items-center gap-4">
        <!-- Balance Section -->
        <div class="card flex items-center gap-3 !p-3">
          <div class="flex items-center gap-2">
            <svg class="w-5 h-5 text-primary" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.31-8.86c-1.77-.45-2.34-.94-2.34-1.67 0-.84.79-1.43 2.1-1.43 1.38 0 1.9.66 1.94 1.64h1.71c-.05-1.34-.87-2.57-2.49-2.97V5H10.9v1.69c-1.51.32-2.72 1.3-2.72 2.81 0 1.79 1.49 2.69 3.66 3.21 1.95.46 2.34 1.15 2.34 1.87 0 .53-.39 1.39-2.1 1.39-1.6 0-2.23-.72-2.32-1.64H8.04c.1 1.7 1.36 2.66 2.86 2.97V19h2.34v-1.67c1.52-.29 2.72-1.16 2.73-2.77-.01-2.2-1.9-2.96-3.66-3.42z"/>
            </svg>
            <span class="text-text-muted text-sm">Balance</span>
          </div>
          <div class="flex items-center gap-1">
            <span class="text-xl font-bold font-mono text-text-primary">{{ formattedBalance }}</span>
            <span class="text-xs text-text-dim">chips</span>
          </div>
        </div>

        <!-- Deposit/Withdraw Buttons (when connected) -->
        <div v-if="isConnected" class="hidden sm:flex gap-2">
          <button
            @click="$emit('deposit')"
            class="btn-primary !py-2 !px-4 text-sm"
          >
            Deposit
          </button>
          <button
            @click="$emit('withdraw')"
            class="btn-secondary !py-2 !px-4 text-sm"
          >
            Withdraw
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup>
import { ref, computed, watch } from 'vue';
import { formatAmount } from '../utils/roulette.js';

const props = defineProps({
  balance: {
    type: [String, Number],
    default: 0
  },
  isConnected: {
    type: Boolean,
    default: false
  }
});

defineEmits(['deposit', 'withdraw']);

// Animated balance value
const displayBalance = ref(Number(props.balance) || 0);
let animationFrameId = null;

// Watch for balance changes and animate
watch(() => props.balance, (newValue, oldValue) => {
  const newBalance = Number(newValue) || 0;
  const oldBalance = Number(oldValue) || 0;

  if (animationFrameId) {
    cancelAnimationFrame(animationFrameId);
  }

  const difference = newBalance - oldBalance;
  const duration = 1000;
  const steps = 30;
  const increment = difference / steps;
  const stepDuration = duration / steps;

  let currentStep = 0;

  function animate() {
    currentStep++;
    displayBalance.value += increment;

    if (currentStep < steps) {
      setTimeout(() => {
        animationFrameId = requestAnimationFrame(animate);
      }, stepDuration);
    } else {
      displayBalance.value = newBalance;
      animationFrameId = null;
    }
  }

  if (difference !== 0) {
    animate();
  } else {
    displayBalance.value = newBalance;
  }
});

const formattedBalance = computed(() => {
  return formatAmount(Math.round(displayBalance.value));
});
</script>
