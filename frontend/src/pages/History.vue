<!-- src/pages/History.vue -->
<template>
  <div class="min-h-screen bg-bg-main py-8">
    <div class="container mx-auto px-4">
      <!-- Page Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-text-primary mb-2">Game History</h1>
        <p class="text-text-muted">View your past spins, bets, and verify fairness of any round.</p>
      </div>

      <!-- Stats Overview -->
      <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-8">
        <div class="card text-center">
          <div class="text-3xl font-bold text-text-primary mb-1">{{ totalSpins }}</div>
          <div class="text-sm text-text-muted">Total Spins</div>
        </div>
        <div class="card text-center">
          <div class="text-3xl font-bold text-success mb-1">{{ totalWins }}</div>
          <div class="text-sm text-text-muted">Wins</div>
        </div>
        <div class="card text-center">
          <div class="text-3xl font-bold text-error mb-1">{{ totalLosses }}</div>
          <div class="text-sm text-text-muted">Losses</div>
        </div>
        <div class="card text-center">
          <div class="text-3xl font-bold font-mono" :class="netProfit >= 0 ? 'text-success' : 'text-error'">
            {{ netProfit >= 0 ? '+' : '' }}{{ netProfit.toLocaleString() }}
          </div>
          <div class="text-sm text-text-muted">Net Profit</div>
        </div>
      </div>

      <!-- Filters -->
      <div class="card mb-6">
        <div class="flex flex-wrap items-center gap-4">
          <div class="flex items-center gap-2">
            <span class="text-text-muted text-sm">Filter:</span>
            <button
              v-for="filter in filters"
              :key="filter.value"
              @click="activeFilter = filter.value"
              class="px-3 py-1.5 rounded-lg text-sm font-medium transition-all duration-200"
              :class="activeFilter === filter.value
                ? 'bg-primary text-white'
                : 'bg-bg-elevated text-text-muted hover:text-text-primary'"
            >
              {{ filter.label }}
            </button>
          </div>

          <div class="flex-1"></div>

          <div class="flex items-center gap-2">
            <span class="text-text-muted text-sm">Show:</span>
            <select
              v-model="pageSize"
              class="bg-bg-elevated border border-border rounded-lg px-3 py-1.5 text-sm text-text-primary focus:border-primary focus:ring-1 focus:ring-primary"
            >
              <option :value="10">10</option>
              <option :value="25">25</option>
              <option :value="50">50</option>
            </select>
          </div>
        </div>
      </div>

      <!-- History Table -->
      <div class="card overflow-hidden p-0">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-border">
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Spin #</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Result</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Bet Amount</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Payout</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Profit</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Time</th>
                <th class="px-6 py-4 text-right text-xs font-semibold text-text-muted uppercase tracking-wider">Actions</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border">
              <tr
                v-for="spin in filteredHistory"
                :key="spin.id"
                class="hover:bg-bg-elevated/50 transition-colors"
              >
                <td class="px-6 py-4 whitespace-nowrap">
                  <span class="font-mono text-text-primary">#{{ spin.id }}</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex items-center gap-2">
                    <span
                      class="w-8 h-8 rounded-full flex items-center justify-center text-sm font-bold text-white"
                      :class="getResultColor(spin.result)"
                    >
                      {{ spin.result }}
                    </span>
                    <span class="text-text-muted text-sm capitalize">{{ getColorName(spin.result) }}</span>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span class="font-mono text-text-primary">{{ spin.betAmount.toLocaleString() }}</span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span class="font-mono" :class="spin.payout > 0 ? 'text-success' : 'text-text-muted'">
                    {{ spin.payout > 0 ? '+' + spin.payout.toLocaleString() : '0' }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="font-mono font-semibold"
                    :class="spin.profit > 0 ? 'text-success' : spin.profit < 0 ? 'text-error' : 'text-text-muted'"
                  >
                    {{ spin.profit > 0 ? '+' : '' }}{{ spin.profit.toLocaleString() }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-text-muted text-sm">
                  {{ formatTime(spin.timestamp) }}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right">
                  <button
                    @click="verifySpin(spin)"
                    class="text-primary hover:text-primary-hover text-sm font-medium transition-colors"
                  >
                    Verify
                  </button>
                </td>
              </tr>

              <!-- Empty State -->
              <tr v-if="filteredHistory.length === 0">
                <td colspan="7" class="px-6 py-12 text-center">
                  <div class="text-text-dim">
                    <svg class="w-12 h-12 mx-auto mb-4 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2" />
                    </svg>
                    <p class="text-lg font-medium mb-1">No history yet</p>
                    <p class="text-sm">Your game history will appear here after you play.</p>
                    <router-link to="/play" class="btn-primary mt-4 inline-block">
                      Start Playing
                    </router-link>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Pagination -->
      <div v-if="totalPages > 1" class="flex justify-center items-center gap-2 mt-6">
        <button
          @click="currentPage = Math.max(1, currentPage - 1)"
          :disabled="currentPage === 1"
          class="px-3 py-2 rounded-lg bg-bg-elevated text-text-muted hover:text-text-primary disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>

        <span class="text-text-muted text-sm">
          Page {{ currentPage }} of {{ totalPages }}
        </span>

        <button
          @click="currentPage = Math.min(totalPages, currentPage + 1)"
          :disabled="currentPage === totalPages"
          class="px-3 py-2 rounded-lg bg-bg-elevated text-text-muted hover:text-text-primary disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

// Demo data - in real app this would come from blockchain/API
const history = ref([
  { id: 1042, result: 17, betAmount: 100, payout: 3600, profit: 3500, timestamp: Date.now() - 120000 },
  { id: 1041, result: 0, betAmount: 50, payout: 0, profit: -50, timestamp: Date.now() - 240000 },
  { id: 1040, result: 32, betAmount: 200, payout: 400, profit: 200, timestamp: Date.now() - 360000 },
  { id: 1039, result: 15, betAmount: 100, payout: 0, profit: -100, timestamp: Date.now() - 480000 },
  { id: 1038, result: 19, betAmount: 150, payout: 300, profit: 150, timestamp: Date.now() - 600000 },
  { id: 1037, result: 4, betAmount: 75, payout: 0, profit: -75, timestamp: Date.now() - 720000 },
  { id: 1036, result: 21, betAmount: 100, payout: 200, profit: 100, timestamp: Date.now() - 840000 },
  { id: 1035, result: 2, betAmount: 50, payout: 0, profit: -50, timestamp: Date.now() - 960000 },
]);

const filters = [
  { label: 'All', value: 'all' },
  { label: 'Wins', value: 'wins' },
  { label: 'Losses', value: 'losses' },
];

const activeFilter = ref('all');
const pageSize = ref(10);
const currentPage = ref(1);

// Computed stats
const totalSpins = computed(() => history.value.length);
const totalWins = computed(() => history.value.filter(s => s.profit > 0).length);
const totalLosses = computed(() => history.value.filter(s => s.profit < 0).length);
const netProfit = computed(() => history.value.reduce((sum, s) => sum + s.profit, 0));

// Filtered history
const filteredHistory = computed(() => {
  let filtered = history.value;

  if (activeFilter.value === 'wins') {
    filtered = filtered.filter(s => s.profit > 0);
  } else if (activeFilter.value === 'losses') {
    filtered = filtered.filter(s => s.profit < 0);
  }

  const start = (currentPage.value - 1) * pageSize.value;
  return filtered.slice(start, start + pageSize.value);
});

const totalPages = computed(() => {
  let filtered = history.value;
  if (activeFilter.value === 'wins') filtered = filtered.filter(s => s.profit > 0);
  if (activeFilter.value === 'losses') filtered = filtered.filter(s => s.profit < 0);
  return Math.ceil(filtered.length / pageSize.value);
});

// Roulette number colors
const redNumbers = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];

function getResultColor(num) {
  if (num === 0) return 'bg-roulette-green';
  return redNumbers.includes(num) ? 'bg-roulette-red' : 'bg-roulette-black';
}

function getColorName(num) {
  if (num === 0) return 'green';
  return redNumbers.includes(num) ? 'red' : 'black';
}

function formatTime(timestamp) {
  const diff = Date.now() - timestamp;
  const minutes = Math.floor(diff / 60000);
  if (minutes < 1) return 'Just now';
  if (minutes < 60) return `${minutes}m ago`;
  const hours = Math.floor(minutes / 60);
  if (hours < 24) return `${hours}h ago`;
  return new Date(timestamp).toLocaleDateString();
}

function verifySpin(spin) {
  // Navigate to fairness verifier or open modal
  console.log('Verify spin:', spin);
}
</script>
