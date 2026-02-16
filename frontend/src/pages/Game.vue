<!-- src/pages/Game.vue -->
<template>
  <div class="min-h-screen bg-bg-main">
    <!-- Connection Status Banner -->
    <ChainInfo
      :chain-id="chainId"
      :app-id="appId"
      :is-connected="isConnected"
      :is-connecting="isConnecting"
      :error="connectionError"
      @connect="handleConnect"
    />

    <!-- Main game area -->
    <main class="container mx-auto px-4 py-4">
      <div class="grid grid-cols-1 lg:grid-cols-12 gap-4">

        <!-- Left column: History & Stats (narrower) -->
        <div class="lg:col-span-3 space-y-4">
          <SpinHistory
            :history="spinHistory"
            @verify-spin="handleVerifySpin"
          />
          <HotColdNumbers
            :hot-numbers="hotNumbers"
            :cold-numbers="coldNumbers"
            @place-bet="handlePlaceBet"
          />

          <!-- Fairness Verifier moved to sidebar -->
          <FairnessVerifier
            ref="fairnessVerifier"
            :next-seed-hash="lastSpinProof?.nextSeedHash"
            :current-seed="lastSpinProof?.currentSeed"
            :last-client-seed="lastSpinProof?.lastClientSeed"
            :last-result="lastSpinProof?.lastResult"
            :spin-number="spinNumber"
            @verify="handleVerifyFairness"
            @copied="handleCopied"
            :class="['transition-all duration-200', { 'ring-2 ring-primary': highlightVerifier }]"
          />
        </div>

        <!-- Center column: Wheel & Board (wider) -->
        <div class="lg:col-span-9 space-y-4">
          <!-- Roulette Wheel -->
          <RouletteWheel
            :is-spinning="isSpinning"
            :last-result="lastResult"
            :target-rotation="targetRotation"
            @spin-complete="onSpinComplete"
          />

          <!-- Betting Board -->
          <BettingBoard
            :selected-chip="selectedChip"
            :current-bets="currentBets"
            :is-betting-open="isBettingOpen"
            @place-bet="handlePlaceBet"
            @remove-bet="handleRemoveBet"
          />

          <!-- Chip Selector -->
          <ChipSelector
            :selected-chip="selectedChip"
            :balance="balance"
            @select="selectedChip = $event"
          />

          <!-- Action Bar: Buttons + Bet Summary -->
          <div class="card !p-4">
            <div class="flex flex-col sm:flex-row items-center justify-between gap-4">
              <!-- Bet Summary -->
              <div class="flex items-center gap-6">
                <div class="text-center sm:text-left">
                  <span class="text-xs text-text-dim block">Total Bet</span>
                  <span class="text-xl font-bold font-mono text-warning">{{ formatAmount(totalBetAmount) }}</span>
                </div>
                <div class="text-center sm:text-left">
                  <span class="text-xs text-text-dim block">Max Win</span>
                  <span class="text-xl font-bold font-mono text-success">{{ formatAmount(maxPotentialWin) }}</span>
                </div>
              </div>

              <!-- Action Buttons -->
              <div class="flex items-center gap-2">
                <button
                  @click="handleClearBets"
                  :disabled="currentBets.length === 0 || !isBettingOpen"
                  class="px-4 py-2 text-sm bg-bg-main hover:bg-error/20 text-text-muted hover:text-error border border-border hover:border-error/30 disabled:opacity-40 disabled:cursor-not-allowed rounded-lg font-medium transition-all duration-200"
                >
                  Clear
                </button>
                <button
                  @click="handleRepeatLastBet"
                  :disabled="lastBets.length === 0 || !isBettingOpen"
                  class="px-4 py-2 text-sm bg-bg-main hover:bg-warning/20 text-text-muted hover:text-warning border border-border hover:border-warning/30 disabled:opacity-40 disabled:cursor-not-allowed rounded-lg font-medium transition-all duration-200"
                >
                  Repeat
                </button>
                <button
                  @click="handleDoubleBets"
                  :disabled="currentBets.length === 0 || !isBettingOpen"
                  class="px-4 py-2 text-sm bg-bg-main hover:bg-primary/20 text-text-muted hover:text-primary border border-border hover:border-primary/30 disabled:opacity-40 disabled:cursor-not-allowed rounded-lg font-medium transition-all duration-200"
                >
                  2x
                </button>
                <button
                  @click="handleSpin"
                  :disabled="currentBets.length === 0 || !isBettingOpen || isSpinning"
                  class="px-8 py-2.5 bg-primary hover:bg-primary-hover disabled:bg-bg-elevated disabled:text-text-dim disabled:cursor-not-allowed rounded-lg font-bold text-lg text-white transition-all duration-200 shadow-md hover:shadow-lg disabled:shadow-none"
                >
                  {{ isSpinning ? 'Spinning...' : 'SPIN' }}
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Win Popup -->
    <WinningsPopup
      :show="showWinPopup"
      :amount="winAmount"
      :result="lastResult"
      @close="showWinPopup = false"
    />

    <!-- Connection Status Toast -->
    <Toast
      :show="showToast"
      :message="toastMessage"
      :type="toastType"
      @close="showToast = false"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, watch, onUnmounted } from 'vue';
import confetti from 'canvas-confetti';
import { useLinera } from '../composables/useLinera';
import { useRoulette } from '../composables/useRoulette';
import { useBets } from '../composables/useBets';
import { checkBetWin, getPayoutMultiplier, formatAmount, calculateSpinRotation } from '../utils/roulette';

// Components
import ChainInfo from '../components/ChainInfo.vue';
import RouletteWheel from '../components/RouletteWheel.vue';
import BettingBoard from '../components/BettingBoard.vue';
import ChipSelector from '../components/ChipSelector.vue';
import SpinHistory from '../components/SpinHistory.vue';
import HotColdNumbers from '../components/HotColdNumbers.vue';
import FairnessVerifier from '../components/FairnessVerifier.vue';
import WinningsPopup from '../components/WinningsPopup.vue';
import Toast from '../components/Toast.vue';

// Linera connection
const {
  chainId,
  appId,
  isConnected,
  isConnecting,
  isReconnecting,
  error: connectionError,
  connect,
  query,
  mutate
} = useLinera();

// Roulette state
const {
  spinHistory,
  hotNumbers,
  coldNumbers,
  lastResult,
  lastSpinProof,
  isSpinning,
  isBettingOpen,
  tableStatus,
  config,
  fetchTableState,
  spin: executeSpin,
  verifyFairness,
} = useRoulette(query, mutate);

// Betting state
const {
  currentBets,
  lastBets,
  selectedChip,
  availableChips,
  totalBetAmount,
  maxPotentialWin,
  validationError,
  placeBet,
  clearBets,
  repeatLastBet,
  doubleBets,
  removeBetFromPosition,
  getBetsForContract,
} = useBets(config);

// Player state
const balance = ref(1000); // Demo balance
const spinNumber = ref(0);
const targetRotation = ref(0);

// Win popup
const showWinPopup = ref(false);
const winAmount = ref(0);

// Fairness verifier ref and highlight
const fairnessVerifier = ref(null);
const highlightVerifier = ref(false);

// Toast notifications
const showToast = ref(false);
const toastMessage = ref('');
const toastType = ref('info');
let toastTimeout = null;

// Polling interval
let pollInterval = null;

// Watch for lastResult changes to update wheel rotation
watch(lastResult, (newResult) => {
  if (newResult !== null && newResult !== undefined) {
    targetRotation.value = calculateSpinRotation(newResult);
  }
});

function showNotification(message, type = 'info', duration = 3000) {
  toastMessage.value = message;
  toastType.value = type;
  showToast.value = true;

  if (toastTimeout) {
    clearTimeout(toastTimeout);
  }

  toastTimeout = setTimeout(() => {
    showToast.value = false;
  }, duration);
}

// Handlers
async function handleConnect() {
  try {
    await connect();
    await fetchTableState();
  } catch (err) {
    console.error('Connection failed:', err);
  }
}

function handlePlaceBet(betInfo) {
  const success = placeBet(betInfo);
  if (!success && validationError.value) {
    showNotification(validationError.value, 'error', 4000);
  }
}

function handleRemoveBet(betInfo) {
  const removed = removeBetFromPosition(betInfo);
  if (removed) {
    showNotification('Bet removed', 'info', 2000);
  }
}

function handleClearBets() {
  clearBets();
}

function handleRepeatLastBet() {
  repeatLastBet();
}

function handleDoubleBets() {
  doubleBets();
}

async function handleSpin() {
  try {
    const bets = getBetsForContract();
    await executeSpin(bets);
    spinNumber.value++;
  } catch (err) {
    console.error('Spin failed:', err);
    const errorMessage = err.message || 'Spin operation failed';
    showNotification(`Error: ${errorMessage}`, 'error', 5000);
  }
}

function onSpinComplete(result) {
  let totalWin = 0;

  for (const bet of currentBets.value) {
    if (checkBetWin(bet, result)) {
      const multiplier = getPayoutMultiplier(bet.type);
      totalWin += bet.amount * (multiplier + 1);
    }
  }

  if (totalWin > 0) {
    winAmount.value = totalWin;
    showWinPopup.value = true;
    balance.value += totalWin - totalBetAmount.value;

    confetti({
      particleCount: 100,
      spread: 70,
      origin: { y: 0.6 }
    });
  } else {
    balance.value -= totalBetAmount.value;
  }

  clearBets();
}

async function handleVerifyFairness(data) {
  const result = await verifyFairness(data.serverSeed, data.clientSeed, data.nonce);
  return result;
}

function handleVerifySpin(spin) {
  if (fairnessVerifier.value && fairnessVerifier.value.$el) {
    fairnessVerifier.value.$el.scrollIntoView({ behavior: 'smooth', block: 'center' });
  }

  highlightVerifier.value = true;
  setTimeout(() => {
    highlightVerifier.value = false;
  }, 2000);

  showNotification(`Viewing fairness data for Spin #${spin.spinId}`, 'info', 3000);
}

function handleCopied(data) {
  showNotification(`${data.label} copied to clipboard!`, 'success', 2000);
}

// Watch for connection status changes
watch(isConnected, (newValue, oldValue) => {
  if (newValue && !oldValue && !isConnecting.value) {
    showNotification('Connection restored!', 'success');
  }
});

watch(isReconnecting, (newValue) => {
  if (newValue) {
    showNotification('Reconnecting...', 'warning', 5000);
  }
});

// Note: Connection errors are shown in ChainInfo component, not as toast

// Initialize
onMounted(async () => {
  await handleConnect();

  pollInterval = setInterval(() => {
    if (isConnected.value) {
      fetchTableState();
    }
  }, 3000);
});

onUnmounted(() => {
  if (pollInterval) {
    clearInterval(pollInterval);
  }
  if (toastTimeout) {
    clearTimeout(toastTimeout);
  }
});
</script>
