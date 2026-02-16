<!-- src/pages/Leaderboard.vue -->
<template>
  <div class="min-h-screen bg-bg-main py-8">
    <div class="container mx-auto px-4">
      <!-- Page Header -->
      <div class="text-center mb-12">
        <h1 class="text-4xl font-bold text-text-primary mb-3">Leaderboard</h1>
        <p class="text-text-muted text-lg">Top players ranked by total winnings</p>
      </div>

      <!-- Time Filter -->
      <div class="flex justify-center gap-2 mb-8">
        <button
          v-for="period in timePeriods"
          :key="period.value"
          @click="activePeriod = period.value"
          class="px-5 py-2.5 rounded-xl font-medium transition-all duration-200"
          :class="activePeriod === period.value
            ? 'bg-primary text-white shadow-md'
            : 'bg-bg-surface text-text-muted hover:text-text-primary border border-border hover:border-primary/50'"
        >
          {{ period.label }}
        </button>
      </div>

      <!-- Top 3 Podium -->
      <div class="flex justify-center items-end gap-4 mb-12">
        <!-- 2nd Place -->
        <div class="flex flex-col items-center">
          <div class="w-20 h-20 md:w-24 md:h-24 rounded-full bg-gradient-to-br from-gray-300 to-gray-400 flex items-center justify-center text-3xl md:text-4xl font-bold text-gray-800 mb-3 shadow-lg">
            2
          </div>
          <div class="card text-center w-36 md:w-44">
            <div class="text-text-primary font-semibold truncate">{{ topPlayers[1]?.name || 'Player 2' }}</div>
            <div class="text-success font-mono font-bold text-lg">+{{ formatAmount(topPlayers[1]?.winnings || 0) }}</div>
            <div class="text-text-dim text-xs mt-1">{{ topPlayers[1]?.spins || 0 }} spins</div>
          </div>
        </div>

        <!-- 1st Place -->
        <div class="flex flex-col items-center -mt-8">
          <div class="relative">
            <div class="absolute -top-6 left-1/2 -translate-x-1/2">
              <svg class="w-8 h-8 text-warning" viewBox="0 0 24 24" fill="currentColor">
                <path d="M5 16L3 5l5.5 5L12 4l3.5 6L21 5l-2 11H5zm14 3c0 .6-.4 1-1 1H6c-.6 0-1-.4-1-1v-1h14v1z"/>
              </svg>
            </div>
            <div class="w-24 h-24 md:w-32 md:h-32 rounded-full bg-gradient-to-br from-yellow-400 to-yellow-600 flex items-center justify-center text-4xl md:text-5xl font-bold text-yellow-900 shadow-lg animate-pulse">
              1
            </div>
          </div>
          <div class="card-elevated text-center w-40 md:w-52 mt-3 border-warning/30">
            <div class="text-text-primary font-bold text-lg truncate">{{ topPlayers[0]?.name || 'Player 1' }}</div>
            <div class="text-success font-mono font-bold text-2xl">+{{ formatAmount(topPlayers[0]?.winnings || 0) }}</div>
            <div class="text-text-dim text-sm mt-1">{{ topPlayers[0]?.spins || 0 }} spins</div>
          </div>
        </div>

        <!-- 3rd Place -->
        <div class="flex flex-col items-center">
          <div class="w-20 h-20 md:w-24 md:h-24 rounded-full bg-gradient-to-br from-amber-600 to-amber-800 flex items-center justify-center text-3xl md:text-4xl font-bold text-amber-200 mb-3 shadow-lg">
            3
          </div>
          <div class="card text-center w-36 md:w-44">
            <div class="text-text-primary font-semibold truncate">{{ topPlayers[2]?.name || 'Player 3' }}</div>
            <div class="text-success font-mono font-bold text-lg">+{{ formatAmount(topPlayers[2]?.winnings || 0) }}</div>
            <div class="text-text-dim text-xs mt-1">{{ topPlayers[2]?.spins || 0 }} spins</div>
          </div>
        </div>
      </div>

      <!-- Full Leaderboard Table -->
      <div class="card overflow-hidden p-0 max-w-4xl mx-auto">
        <div class="overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="border-b border-border bg-bg-elevated/50">
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Rank</th>
                <th class="px-6 py-4 text-left text-xs font-semibold text-text-muted uppercase tracking-wider">Player</th>
                <th class="px-6 py-4 text-right text-xs font-semibold text-text-muted uppercase tracking-wider">Winnings</th>
                <th class="px-6 py-4 text-right text-xs font-semibold text-text-muted uppercase tracking-wider">Spins</th>
                <th class="px-6 py-4 text-right text-xs font-semibold text-text-muted uppercase tracking-wider">Win Rate</th>
              </tr>
            </thead>
            <tbody class="divide-y divide-border">
              <tr
                v-for="(player, index) in leaderboard"
                :key="player.address"
                class="hover:bg-bg-elevated/50 transition-colors"
                :class="{ 'bg-primary/5': player.isYou }"
              >
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="inline-flex items-center justify-center w-8 h-8 rounded-full text-sm font-bold"
                    :class="getRankStyle(index + 1)"
                  >
                    {{ index + 1 }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-gradient-primary flex items-center justify-center text-white font-bold">
                      {{ player.name.charAt(0).toUpperCase() }}
                    </div>
                    <div>
                      <div class="font-medium text-text-primary flex items-center gap-2">
                        {{ player.name }}
                        <span v-if="player.isYou" class="badge-primary text-xs">You</span>
                      </div>
                      <div class="text-xs text-text-dim font-mono">{{ truncateAddress(player.address) }}</div>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right">
                  <span class="font-mono font-bold" :class="player.winnings >= 0 ? 'text-success' : 'text-error'">
                    {{ player.winnings >= 0 ? '+' : '' }}{{ formatAmount(player.winnings) }}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right text-text-muted font-mono">
                  {{ player.spins.toLocaleString() }}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-right">
                  <span class="font-mono" :class="player.winRate >= 50 ? 'text-success' : 'text-text-muted'">
                    {{ player.winRate.toFixed(1) }}%
                  </span>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>

      <!-- Your Stats -->
      <div class="max-w-4xl mx-auto mt-8">
        <div class="card border-primary/30">
          <div class="flex items-center gap-4 mb-4">
            <div class="w-12 h-12 rounded-full bg-gradient-primary flex items-center justify-center text-white font-bold text-lg">
              Y
            </div>
            <div>
              <h3 class="font-bold text-text-primary">Your Position</h3>
              <p class="text-sm text-text-muted">Connected wallet: 0x7a3d...f9e2</p>
            </div>
          </div>

          <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
            <div class="bg-bg-main rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-primary">#24</div>
              <div class="text-xs text-text-muted">Current Rank</div>
            </div>
            <div class="bg-bg-main rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-success font-mono">+2,450</div>
              <div class="text-xs text-text-muted">Total Winnings</div>
            </div>
            <div class="bg-bg-main rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-text-primary">156</div>
              <div class="text-xs text-text-muted">Total Spins</div>
            </div>
            <div class="bg-bg-main rounded-lg p-4 text-center">
              <div class="text-2xl font-bold text-warning">48.7%</div>
              <div class="text-xs text-text-muted">Win Rate</div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const timePeriods = [
  { label: 'Today', value: 'today' },
  { label: 'This Week', value: 'week' },
  { label: 'This Month', value: 'month' },
  { label: 'All Time', value: 'all' },
];

const activePeriod = ref('week');

// Demo leaderboard data
const leaderboard = ref([
  { name: 'CryptoKing', address: '0x1a2b3c4d5e6f7890abcdef1234567890abcdef12', winnings: 45230, spins: 892, winRate: 54.2, isYou: false },
  { name: 'LuckyDegen', address: '0x2b3c4d5e6f7890abcdef1234567890abcdef1234', winnings: 38750, spins: 654, winRate: 52.8, isYou: false },
  { name: 'WheelMaster', address: '0x3c4d5e6f7890abcdef1234567890abcdef123456', winnings: 31200, spins: 1203, winRate: 49.1, isYou: false },
  { name: 'RouletteRider', address: '0x4d5e6f7890abcdef1234567890abcdef12345678', winnings: 28900, spins: 445, winRate: 55.7, isYou: false },
  { name: 'SpinToWin', address: '0x5e6f7890abcdef1234567890abcdef1234567890', winnings: 24100, spins: 789, winRate: 48.3, isYou: false },
  { name: 'ChainGambler', address: '0x6f7890abcdef1234567890abcdef1234567890ab', winnings: 21500, spins: 567, winRate: 51.2, isYou: false },
  { name: 'BlockBetter', address: '0x7890abcdef1234567890abcdef1234567890abcd', winnings: 18750, spins: 334, winRate: 53.9, isYou: false },
  { name: 'DeFiDice', address: '0x890abcdef1234567890abcdef1234567890abcde', winnings: 15200, spins: 678, winRate: 47.5, isYou: false },
  { name: 'Web3Wheeler', address: '0x90abcdef1234567890abcdef1234567890abcdef', winnings: 12800, spins: 445, winRate: 50.1, isYou: false },
  { name: 'TokenTurner', address: '0x0abcdef1234567890abcdef1234567890abcdef1', winnings: 9500, spins: 234, winRate: 52.6, isYou: false },
]);

const topPlayers = computed(() => leaderboard.value.slice(0, 3));

function formatAmount(amount) {
  if (amount >= 1000000) return (amount / 1000000).toFixed(1) + 'M';
  if (amount >= 1000) return (amount / 1000).toFixed(1) + 'K';
  return amount.toLocaleString();
}

function truncateAddress(address) {
  return address.slice(0, 6) + '...' + address.slice(-4);
}

function getRankStyle(rank) {
  if (rank === 1) return 'bg-yellow-500 text-yellow-900';
  if (rank === 2) return 'bg-gray-300 text-gray-800';
  if (rank === 3) return 'bg-amber-600 text-amber-100';
  return 'bg-bg-elevated text-text-muted';
}
</script>
