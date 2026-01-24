<!-- src/components/ChainInfo.vue -->
<template>
  <div class="bg-black/30 backdrop-blur-sm rounded-xl p-4 border border-green-800/50">
    <!-- Connection Status Header -->
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center space-x-2">
        <!-- Status Indicator Dot -->
        <span
          class="w-3 h-3 rounded-full"
          :class="statusDotClass"
        ></span>
        <span class="text-sm font-medium" :class="statusTextClass">
          {{ statusText }}
        </span>
      </div>

      <!-- Demo Mode Badge -->
      <div
        v-if="isDemoMode && isConnected"
        class="flex items-center space-x-1 bg-yellow-900/50 px-2 py-1 rounded-full"
      >
        <svg class="w-4 h-4 text-yellow-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        <span class="text-xs text-yellow-400 font-medium">Demo Mode</span>
      </div>

      <!-- Provably Fair Badge (when not in demo mode) -->
      <div
        v-else-if="isConnected"
        class="flex items-center space-x-1 bg-green-900/50 px-2 py-1 rounded-full"
      >
        <svg class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
        </svg>
        <span class="text-xs text-green-400 font-medium">Provably Fair</span>
      </div>
    </div>

    <!-- Chain Details -->
    <div v-if="isConnected" class="space-y-2">
      <!-- Chain ID -->
      <div class="flex items-center justify-between bg-black/30 rounded-lg px-3 py-2">
        <div class="flex-1 min-w-0">
          <span class="text-xs text-gray-500 block">Chain ID</span>
          <span class="text-sm font-mono text-green-400 truncate block">
            {{ displayChainId }}
          </span>
        </div>
        <button
          @click="copyChainId"
          class="ml-2 p-2 hover:bg-green-900/30 rounded-lg transition-colors"
          :title="copied ? 'Copied!' : 'Copy Chain ID'"
        >
          <svg v-if="!copied" class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
          <svg v-else class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </button>
      </div>

      <!-- App ID -->
      <div v-if="appId" class="bg-black/30 rounded-lg px-3 py-2">
        <span class="text-xs text-gray-500 block">App ID</span>
        <span class="text-sm font-mono text-gray-400 truncate block">
          {{ displayAppId }}
        </span>
      </div>
    </div>

    <!-- Connect Button when disconnected -->
    <div v-else class="mt-2">
      <button
        @click="$emit('connect')"
        :disabled="isConnecting"
        class="w-full py-2 px-4 rounded-lg font-medium transition-all
               bg-green-600 hover:bg-green-500 disabled:bg-gray-700 disabled:cursor-not-allowed"
      >
        <span v-if="isConnecting" class="flex items-center justify-center space-x-2">
          <svg class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Connecting...</span>
        </span>
        <span v-else>Connect to Linera</span>
      </button>
    </div>

    <!-- Error Display -->
    <div v-if="error" class="mt-2 p-2 bg-red-900/30 border border-red-700/50 rounded-lg">
      <span class="text-xs text-red-400">{{ error }}</span>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  chainId: {
    type: String,
    default: null
  },
  appId: {
    type: String,
    default: null
  },
  isConnected: {
    type: Boolean,
    default: false
  },
  isConnecting: {
    type: Boolean,
    default: false
  },
  isDemoMode: {
    type: Boolean,
    default: false
  },
  error: {
    type: String,
    default: null
  }
});

defineEmits(['connect']);

const copied = ref(false);

// Status indicator classes
const statusDotClass = computed(() => {
  if (props.isConnected) return 'bg-green-400 shadow-green-400/50 shadow-lg';
  if (props.isConnecting) return 'bg-yellow-400 animate-pulse shadow-yellow-400/50 shadow-lg';
  return 'bg-red-400 shadow-red-400/50 shadow-lg';
});

const statusTextClass = computed(() => {
  if (props.isConnected) return 'text-green-400';
  if (props.isConnecting) return 'text-yellow-400';
  return 'text-red-400';
});

const statusText = computed(() => {
  if (props.isConnected) {
    return props.isDemoMode ? 'Connected (Demo Mode)' : 'Connected to Conway Testnet';
  }
  if (props.isConnecting) return 'Connecting...';
  return 'Disconnected';
});

// Truncate long IDs for display
const displayChainId = computed(() => {
  if (!props.chainId) return '';
  if (props.chainId.length <= 20) return props.chainId;
  return `${props.chainId.slice(0, 10)}...${props.chainId.slice(-8)}`;
});

const displayAppId = computed(() => {
  if (!props.appId) return '';
  if (props.appId.length <= 20) return props.appId;
  return `${props.appId.slice(0, 10)}...${props.appId.slice(-8)}`;
});

// Copy chain ID to clipboard
async function copyChainId() {
  if (!props.chainId) return;

  try {
    await navigator.clipboard.writeText(props.chainId);
    copied.value = true;
    setTimeout(() => {
      copied.value = false;
    }, 2000);
  } catch (err) {
    console.error('Failed to copy:', err);
  }
}
</script>
