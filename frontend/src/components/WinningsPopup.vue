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
        <div class="relative bg-gradient-to-b from-green-800 to-green-900 rounded-2xl p-8 shadow-2xl border-2 border-yellow-500/50 max-w-md mx-4 transform">
          <!-- Celebration Emoji -->
          <div class="text-center">
            <span class="text-6xl block mb-4 animate-bounce">🎉</span>

            <!-- YOU WON! text -->
            <h2 class="text-4xl font-bold text-yellow-400 mb-4 animate-pulse">
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
            <div class="bg-black/30 rounded-xl p-4 mb-6">
              <div class="text-gray-400 text-sm">Amount Won</div>
              <div class="text-3xl font-bold text-green-400">
                +{{ formattedAmount }}
              </div>
              <div class="text-gray-500 text-xs">chips</div>
            </div>

            <!-- Awesome! Button -->
            <button
              @click="handleClose"
              class="w-full py-3 px-6 bg-yellow-500 hover:bg-yellow-400 text-black font-bold rounded-xl transition-colors text-lg"
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
import { computed, watch, onMounted } from 'vue';
import { getNumberColor, formatAmount } from '../utils/roulette.js';

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
  transition: all 0.3s ease;
}

.popup-enter-from,
.popup-leave-to {
  opacity: 0;
}

.popup-enter-from .relative,
.popup-leave-to .relative {
  transform: scale(0.9);
}
</style>
