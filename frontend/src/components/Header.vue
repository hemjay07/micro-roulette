<!-- src/components/Header.vue -->
<template>
  <header class="bg-black/50 backdrop-blur-sm border-b border-green-700 px-4 py-4">
    <div class="container mx-auto flex items-center justify-between">
      <!-- Logo and Title -->
      <div class="flex items-center space-x-3">
        <span class="text-4xl">🎰</span>
        <div>
          <h1 class="text-2xl font-bold text-white">MicroRoulette</h1>
          <p class="text-sm text-green-400">Every Spin On-Chain</p>
        </div>
      </div>

      <!-- Balance Display -->
      <div class="flex items-center space-x-4">
        <!-- Balance Section -->
        <div class="bg-black/30 rounded-lg px-4 py-2 flex items-center space-x-3">
          <div class="flex items-center space-x-2">
            <svg class="w-5 h-5 text-yellow-400" fill="currentColor" viewBox="0 0 20 20">
              <path d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.736 6.979C9.208 6.193 9.696 6 10 6c.304 0 .792.193 1.264.979a1 1 0 001.715-1.029C12.279 4.784 11.232 4 10 4s-2.279.784-2.979 1.95c-.285.475-.507 1-.67 1.55H6a1 1 0 000 2h.013a9.358 9.358 0 000 1H6a1 1 0 100 2h.351c.163.55.385 1.075.67 1.55C7.721 15.216 8.768 16 10 16s2.279-.784 2.979-1.95a1 1 0 10-1.715-1.029c-.472.786-.96.979-1.264.979-.304 0-.792-.193-1.264-.979a4.265 4.265 0 01-.264-.521H10a1 1 0 100-2H8.017a7.36 7.36 0 010-1H10a1 1 0 100-2H8.472c.08-.185.167-.36.264-.521z" />
            </svg>
            <span class="text-gray-400 text-sm">Balance</span>
          </div>
          <div class="flex items-center space-x-1">
            <span class="text-xl font-bold text-white">{{ formattedBalance }}</span>
            <span class="text-xs text-gray-500">chips</span>
          </div>
        </div>

        <!-- Deposit/Withdraw Buttons (when connected) -->
        <div v-if="isConnected" class="flex space-x-2">
          <button
            @click="$emit('deposit')"
            class="px-3 py-2 bg-green-600 hover:bg-green-500 rounded-lg text-sm font-medium transition-colors"
          >
            Deposit
          </button>
          <button
            @click="$emit('withdraw')"
            class="px-3 py-2 bg-gray-700 hover:bg-gray-600 rounded-lg text-sm font-medium transition-colors"
          >
            Withdraw
          </button>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup>
import { computed } from 'vue';
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

// Format balance for display
const formattedBalance = computed(() => {
  return formatAmount(props.balance);
});
</script>
