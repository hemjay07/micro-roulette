<!-- src/components/FairnessVerifier.vue -->
<template>
  <div class="fairness-verifier bg-black/30 backdrop-blur-sm rounded-xl p-4 border border-green-800/50">
    <div class="flex items-center justify-between mb-4">
      <h3 class="text-lg font-medium text-white flex items-center space-x-2">
        <svg class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
        </svg>
        <span>Provably Fair</span>
      </h3>
    </div>

    <!-- Seed Information -->
    <div class="space-y-3 mb-4">
      <!-- Next Seed Hash (Committed) -->
      <div class="bg-black/30 rounded-lg p-3">
        <div class="flex items-center justify-between">
          <span class="text-xs text-gray-400">Next Seed Hash (Committed)</span>
          <span class="text-[10px] px-2 py-0.5 bg-yellow-600/30 text-yellow-400 rounded">LOCKED</span>
        </div>
        <div class="flex items-center space-x-2">
          <div class="font-mono text-sm text-green-400 mt-1 truncate flex-1">
            {{ nextSeedHash || 'Not available' }}
          </div>
          <button
            v-if="nextSeedHash"
            @click="copyToClipboard(nextSeedHash, 'nextHash')"
            class="p-1 hover:bg-green-900/30 rounded transition-colors flex-shrink-0"
            :title="copiedNextHash ? 'Copied!' : 'Copy hash'"
          >
            <svg v-if="!copiedNextHash" class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            <svg v-else class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Current Seed (Revealed) -->
      <div class="bg-black/30 rounded-lg p-3">
        <div class="flex items-center justify-between">
          <span class="text-xs text-gray-400">Current Seed (Revealed)</span>
          <span v-if="currentSeed" class="text-[10px] px-2 py-0.5 bg-green-600/30 text-green-400 rounded">REVEALED</span>
        </div>
        <div class="flex items-center space-x-2">
          <div class="font-mono text-sm text-gray-400 mt-1 truncate flex-1">
            {{ currentSeed || 'Available after first spin' }}
          </div>
          <button
            v-if="currentSeed"
            @click="copyToClipboard(currentSeed, 'currentSeed')"
            class="p-1 hover:bg-green-900/30 rounded transition-colors flex-shrink-0"
            :title="copiedCurrentSeed ? 'Copied!' : 'Copy seed'"
          >
            <svg v-if="!copiedCurrentSeed" class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            <svg v-else class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Last Client Seed -->
      <div class="bg-black/30 rounded-lg p-3">
        <div class="flex items-center justify-between">
          <span class="text-xs text-gray-400">Last Client Seed</span>
          <span v-if="lastClientSeed" class="text-[10px] px-2 py-0.5 bg-blue-600/30 text-blue-400 rounded">CLIENT</span>
        </div>
        <div class="flex items-center space-x-2">
          <div class="font-mono text-sm text-gray-400 mt-1 truncate flex-1">
            {{ lastClientSeed || 'Available after first spin' }}
          </div>
          <button
            v-if="lastClientSeed"
            @click="copyToClipboard(lastClientSeed, 'clientSeed')"
            class="p-1 hover:bg-green-900/30 rounded transition-colors flex-shrink-0"
            :title="copiedClientSeed ? 'Copied!' : 'Copy client seed'"
          >
            <svg v-if="!copiedClientSeed" class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            <svg v-else class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Nonce (Spin Number) -->
      <div class="bg-black/30 rounded-lg p-3">
        <div class="flex items-center justify-between">
          <span class="text-xs text-gray-400">Nonce (Spin Number)</span>
          <span v-if="spinNumber > 0" class="text-[10px] px-2 py-0.5 bg-purple-600/30 text-purple-400 rounded">NONCE</span>
        </div>
        <div class="flex items-center space-x-2">
          <div class="font-mono text-sm text-gray-400 mt-1 flex-1">
            {{ spinNumber > 0 ? spinNumber : 'Available after first spin' }}
          </div>
          <button
            v-if="spinNumber > 0"
            @click="copyToClipboard(spinNumber.toString(), 'nonce')"
            class="p-1 hover:bg-green-900/30 rounded transition-colors flex-shrink-0"
            :title="copiedNonce ? 'Copied!' : 'Copy nonce'"
          >
            <svg v-if="!copiedNonce" class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
            </svg>
            <svg v-else class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
          </button>
        </div>
      </div>
    </div>

    <!-- Verification Form -->
    <div class="bg-black/40 rounded-lg p-4 mb-4">
      <h4 class="text-sm font-medium text-gray-300 mb-3">Verify a Spin</h4>

      <div class="space-y-3">
        <!-- Server Seed Input -->
        <div>
          <label class="text-xs text-gray-500 block mb-1">Server Seed</label>
          <input
            v-model="verifyServerSeed"
            type="text"
            placeholder="Enter server seed"
            class="w-full bg-black/50 border border-gray-700 rounded px-3 py-2 text-sm font-mono text-white placeholder-gray-600 focus:outline-none focus:border-green-500"
          />
        </div>

        <!-- Client Seed Input -->
        <div>
          <label class="text-xs text-gray-500 block mb-1">Client Seed</label>
          <input
            v-model="verifyClientSeed"
            type="text"
            placeholder="Enter client seed"
            class="w-full bg-black/50 border border-gray-700 rounded px-3 py-2 text-sm font-mono text-white placeholder-gray-600 focus:outline-none focus:border-green-500"
          />
        </div>

        <!-- Nonce Input -->
        <div>
          <label class="text-xs text-gray-500 block mb-1">Nonce (Spin Number)</label>
          <input
            v-model="verifyNonce"
            type="number"
            placeholder="Enter spin number"
            class="w-full bg-black/50 border border-gray-700 rounded px-3 py-2 text-sm font-mono text-white placeholder-gray-600 focus:outline-none focus:border-green-500"
          />
        </div>

        <!-- Verify Button -->
        <button
          @click="handleVerify"
          :disabled="!canVerify || isVerifying"
          class="w-full py-2 px-4 bg-green-600 hover:bg-green-500 disabled:bg-gray-700 disabled:cursor-not-allowed rounded-lg font-medium transition-colors flex items-center justify-center space-x-2"
        >
          <svg v-if="isVerifying" class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>{{ isVerifying ? 'Verifying...' : 'Verify Result' }}</span>
        </button>
      </div>

      <!-- Verification Result -->
      <div v-if="verificationResult" class="mt-3 p-3 rounded-lg" :class="verificationResult.isValid ? 'bg-green-900/30 border border-green-700' : 'bg-red-900/30 border border-red-700'">
        <div class="flex items-center space-x-2 mb-2">
          <svg v-if="verificationResult.isValid" class="w-5 h-5 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <svg v-else class="w-5 h-5 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span :class="verificationResult.isValid ? 'text-green-400' : 'text-red-400'">
            {{ verificationResult.isValid ? 'Valid!' : 'Invalid' }}
          </span>
        </div>
        <div class="text-sm">
          <div class="text-gray-400">Result: <span class="font-bold text-white">{{ verificationResult.result }}</span></div>
          <div class="text-gray-400">Color: <span class="font-bold" :class="getResultColor(verificationResult.resultColor)">{{ verificationResult.resultColor }}</span></div>
        </div>
      </div>
    </div>

    <!-- How it Works (Expandable) -->
    <div class="border-t border-gray-800 pt-3">
      <button
        @click="showHowItWorks = !showHowItWorks"
        class="flex items-center justify-between w-full text-sm text-gray-400 hover:text-white transition-colors"
        :aria-expanded="showHowItWorks"
        aria-controls="how-it-works-content"
      >
        <span id="how-it-works-label">How it works</span>
        <svg
          class="w-4 h-4 transition-transform"
          :class="{ 'rotate-180': showHowItWorks }"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          aria-hidden="true"
        >
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
        </svg>
      </button>

      <div
        v-if="showHowItWorks"
        id="how-it-works-content"
        role="region"
        aria-labelledby="how-it-works-label"
        class="mt-3 text-xs text-gray-500 space-y-2"
      >
        <p>
          <strong class="text-gray-400">1. Commit:</strong> Before each spin, the house commits to a server seed by publishing its SHA-256 hash.
        </p>
        <p>
          <strong class="text-gray-400">2. Bet:</strong> Players place bets. The client seed is generated from user input.
        </p>
        <p>
          <strong class="text-gray-400">3. Reveal:</strong> After betting closes, the server seed is revealed and combined with the client seed.
        </p>
        <p>
          <strong class="text-gray-400">4. Verify:</strong> SHA-256(server_seed || client_seed || nonce) % 37 = result. Anyone can verify!
        </p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  nextSeedHash: {
    type: String,
    default: null
  },
  currentSeed: {
    type: String,
    default: null
  },
  lastClientSeed: {
    type: String,
    default: null
  },
  lastResult: {
    type: Number,
    default: null
  },
  spinNumber: {
    type: Number,
    default: 0
  }
});

const emit = defineEmits(['verify']);

// Verification form state
const verifyServerSeed = ref('');
const verifyClientSeed = ref('');
const verifyNonce = ref('');
const isVerifying = ref(false);
const verificationResult = ref(null);
const showHowItWorks = ref(false);

// Copy state
const copiedNextHash = ref(false);
const copiedCurrentSeed = ref(false);
const copiedClientSeed = ref(false);
const copiedNonce = ref(false);

// Check if verification form is complete
const canVerify = computed(() => {
  return verifyServerSeed.value.trim() &&
         verifyClientSeed.value.trim() &&
         verifyNonce.value !== '';
});

// Handle verify button click
async function handleVerify() {
  if (!canVerify.value) return;

  isVerifying.value = true;
  verificationResult.value = null;

  try {
    // Emit verify event and wait for result
    const result = await new Promise((resolve) => {
      emit('verify', {
        serverSeed: verifyServerSeed.value,
        clientSeed: verifyClientSeed.value,
        nonce: parseInt(verifyNonce.value),
        callback: resolve
      });

      // Fallback timeout in case parent doesn't respond
      setTimeout(() => resolve(null), 10000);
    });

    if (result) {
      verificationResult.value = result;
    }
  } catch (err) {
    console.error('Verification failed:', err);
  } finally {
    isVerifying.value = false;
  }
}

// Get color class for result
function getResultColor(color) {
  if (color === 'Red') return 'text-roulette-red';
  if (color === 'Black') return 'text-gray-300';
  return 'text-roulette-green';
}

// Copy to clipboard
async function copyToClipboard(text, type) {
  try {
    await navigator.clipboard.writeText(text);
    if (type === 'nextHash') {
      copiedNextHash.value = true;
      setTimeout(() => { copiedNextHash.value = false; }, 2000);
    } else if (type === 'currentSeed') {
      copiedCurrentSeed.value = true;
      setTimeout(() => { copiedCurrentSeed.value = false; }, 2000);
    } else if (type === 'clientSeed') {
      copiedClientSeed.value = true;
      setTimeout(() => { copiedClientSeed.value = false; }, 2000);
    } else if (type === 'nonce') {
      copiedNonce.value = true;
      setTimeout(() => { copiedNonce.value = false; }, 2000);
    }
  } catch (err) {
    console.error('Failed to copy:', err);
  }
}
</script>
