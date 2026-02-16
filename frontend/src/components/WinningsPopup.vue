<!-- src/components/WinningsPopup.vue -->
<template>
  <Teleport to="body">
    <Transition name="popup">
      <div
        v-if="show"
        class="fixed inset-0 flex items-center justify-center"
        style="z-index: 200"
      >
        <!-- Backdrop -->
        <div class="absolute inset-0 bg-black/70 backdrop-blur-sm" @click="handleClose"></div>

        <!-- Popup Content -->
        <div class="relative bg-gradient-to-b from-bg-elevated to-bg-surface rounded-2xl p-8 shadow-2xl border border-primary/50 max-w-md mx-4 transform">
          <!-- Celebration Emoji -->
          <div class="text-center">
            <span class="text-6xl block mb-4 animate-bounce">🎉</span>

            <!-- YOU WON! text -->
            <h2 class="text-4xl font-bold text-primary mb-4 animate-pulse">
              YOU WON!
            </h2>

            <!-- Result Number -->
            <div class="mb-6">
              <div
                class="inline-flex items-center justify-center w-20 h-20 rounded-full text-4xl font-bold shadow-lg"
                :class="resultColorClass"
              >
                {{ result }}
              </div>
              <div class="mt-2 text-gray-400 text-sm">
                {{ resultColorText }}
              </div>
            </div>

            <!-- Amount Won -->
            <div class="bg-bg-main rounded-xl p-4 mb-6">
              <div class="text-text-muted text-sm">Amount Won</div>
              <div class="text-3xl font-bold font-mono text-success">
                +{{ formattedAmount }}
              </div>
              <div class="text-text-dim text-xs">chips</div>
            </div>

            <!-- Awesome! Button -->
            <button
              ref="closeButton"
              @click="handleClose"
              class="btn-primary w-full text-lg"
            >
              Awesome!
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { computed, watch, onMounted, ref, nextTick } from 'vue';
import { getNumberColor, formatAmount } from '../utils/roulette.js';

// Reference to the close button
const closeButton = ref(null);

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  result: {
    type: Number,
    default: null
  },
  amount: {
    type: [String, Number],
    default: 0
  }
});

const emit = defineEmits(['close']);

// Handle keyboard events
function handleKeydown(event) {
  if (event.key === 'Escape') {
    handleClose();
  }
}

// Add/remove keyboard listener when popup shows/hides
watch(() => props.show, async (newVal) => {
  if (newVal) {
    window.addEventListener('keydown', handleKeydown);
    // Auto-focus the close button for keyboard accessibility
    await nextTick();
    if (closeButton.value) {
      closeButton.value.focus();
    }
  } else {
    window.removeEventListener('keydown', handleKeydown);
  }
});

// Cleanup on unmount
import { onUnmounted } from 'vue';
onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});

// Result color styling
const resultColorClass = computed(() => {
  if (props.result === null) return '';
  const color = getNumberColor(props.result);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
});

const resultColorText = computed(() => {
  if (props.result === null) return '';
  const color = getNumberColor(props.result);
  return color.charAt(0).toUpperCase() + color.slice(1);
});

// Format amount for display
const formattedAmount = computed(() => {
  return formatAmount(props.amount);
});

// Handle close
function handleClose() {
  emit('close');
}

// Trigger confetti when popup shows
watch(() => props.show, async (newVal) => {
  if (newVal) {
    triggerConfetti();
  }
});

// Confetti animation using canvas-confetti library
async function triggerConfetti() {
  try {
    const confetti = (await import('canvas-confetti')).default;

    // Fire confetti from both sides
    const count = 200;
    const defaults = {
      origin: { y: 0.7 },
      zIndex: 100,
    };

    function fire(particleRatio, opts) {
      confetti({
        ...defaults,
        ...opts,
        particleCount: Math.floor(count * particleRatio),
      });
    }

    fire(0.25, {
      spread: 26,
      startVelocity: 55,
      origin: { x: 0.2 },
    });

    fire(0.2, {
      spread: 60,
      origin: { x: 0.8 },
    });

    fire(0.35, {
      spread: 100,
      decay: 0.91,
      scalar: 0.8,
      origin: { x: 0.5 },
    });

    fire(0.1, {
      spread: 120,
      startVelocity: 25,
      decay: 0.92,
      scalar: 1.2,
      origin: { x: 0.5 },
    });

    fire(0.1, {
      spread: 120,
      startVelocity: 45,
      origin: { x: 0.5 },
    });

  } catch (err) {
    console.warn('Confetti animation not available:', err);
  }
}
</script>

<style scoped>
/* Popup transition */
.popup-enter-active,
.popup-leave-active {
  transition: opacity 0.25s ease-out;
}

.popup-enter-active .relative,
.popup-leave-active .relative {
  transition: transform 0.25s ease-out;
}

.popup-enter-from,
.popup-leave-to {
  opacity: 0;
}

.popup-enter-from .relative,
.popup-leave-to .relative {
  transform: scale(0.95);
}
</style>
