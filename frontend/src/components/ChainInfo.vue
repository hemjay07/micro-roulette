<!-- src/components/ChainInfo.vue -->
<template>
  <div class="bg-bg-surface border border-border rounded-xl p-4 mx-4 mt-4">
    <!-- Connection Status Header -->
    <div class="flex items-center justify-between mb-3">
      <div class="flex items-center gap-2">
        <!-- Status Indicator Dot -->
        <span class="relative flex h-3 w-3">
          <span
            v-if="isConnected || isConnecting"
            class="animate-ping absolute inline-flex h-full w-full rounded-full opacity-75"
            :class="isConnected ? 'bg-success' : 'bg-warning'"
          ></span>
          <span
            class="relative inline-flex rounded-full h-3 w-3"
            :class="statusDotClass"
          ></span>
        </span>
        <span class="text-sm font-medium" :class="statusTextClass">
          {{ statusText }}
        </span>
      </div>

      <!-- Provably Fair Badge -->
      <div
        v-if="isConnected"
        class="badge-success"
      >
        <svg class="w-3.5 h-3.5 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
        </svg>
        Provably Fair
      </div>
    </div>

    <!-- Chain Details -->
    <div v-if="isConnected" class="space-y-2">
      <!-- Chain ID -->
      <div class="flex items-center justify-between bg-bg-main rounded-lg px-3 py-2">
        <div class="flex-1 min-w-0">
          <span class="text-xs text-text-dim block">Chain ID</span>
          <span class="text-sm font-mono text-primary truncate block">
            {{ displayChainId }}
          </span>
        </div>
        <button
          @click="copyChainId"
          class="ml-2 p-2 hover:bg-bg-elevated rounded-lg transition-colors"
          :title="copied ? 'Copied!' : 'Copy Chain ID'"
        >
          <svg v-if="!copied" class="w-4 h-4 text-text-muted" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
          </svg>
          <svg v-else class="w-4 h-4 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
          </svg>
        </button>
      </div>

      <!-- App ID -->
      <div v-if="appId" class="bg-bg-main rounded-lg px-3 py-2">
        <span class="text-xs text-text-dim block">App ID</span>
        <span class="text-sm font-mono text-text-muted truncate block">
          {{ displayAppId }}
        </span>
      </div>

      <!-- Faucet Link -->
      <a
        href="https://faucet.testnet-conway.linera.net"
        target="_blank"
        rel="noopener noreferrer"
        class="flex items-center justify-between w-full bg-bg-main hover:bg-bg-elevated rounded-lg px-3 py-2 group transition-colors"
      >
        <div class="flex items-center gap-2">
          <svg class="w-4 h-4 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          <span class="text-sm text-primary font-medium group-hover:text-primary-hover">Get Test Tokens</span>
        </div>
        <svg class="w-4 h-4 text-text-dim group-hover:text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14" />
        </svg>
      </a>
    </div>

    <!-- Connect Button when disconnected -->
    <div v-else class="mt-2">
      <button
        @click="$emit('connect')"
        :disabled="isConnecting"
        class="btn-primary w-full"
      >
        <span v-if="isConnecting" class="flex items-center justify-center gap-2">
          <svg class="animate-spin w-4 h-4" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span>Connecting...</span>
        </span>
        <span v-else>Connect to Linera</span>
      </button>
    </div>

    <!-- Error Display - Compact inline -->
    <div v-if="error" class="mt-2 flex items-center gap-2 text-xs text-error bg-error/10 rounded-lg px-3 py-2">
      <svg class="w-4 h-4 flex-shrink-0" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <span class="truncate">{{ error }}</span>
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

const statusDotClass = computed(() => {
  if (props.isConnected) return 'bg-success';
  if (props.isConnecting) return 'bg-warning';
  return 'bg-error';
});

const statusTextClass = computed(() => {
  if (props.isConnected) return 'text-success';
  if (props.isConnecting) return 'text-warning';
  return 'text-error';
});

const statusText = computed(() => {
  if (props.isConnected) {
    return 'Connected to Conway Testnet';
  }
  if (props.isConnecting) return 'Connecting...';
  return 'Disconnected';
});

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
