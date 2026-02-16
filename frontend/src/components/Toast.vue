<!-- src/components/Toast.vue -->
<template>
  <Transition name="toast">
    <div
      v-if="show"
      class="fixed top-20 right-4 z-50 max-w-sm"
      role="alert"
      aria-live="polite"
    >
      <div
        class="rounded-xl shadow-lg p-4 border backdrop-blur-safe"
        :class="toastClasses"
      >
        <div class="flex items-center gap-3">
          <!-- Icon -->
          <div class="flex-shrink-0">
            <svg
              v-if="type === 'success'"
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M5 13l4 4L19 7"
              />
            </svg>
            <svg
              v-else-if="type === 'error'"
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
            <svg
              v-else-if="type === 'warning'"
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
            <svg
              v-else
              class="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>

          <!-- Message -->
          <div class="flex-1">
            <p class="font-medium">{{ message }}</p>
          </div>

          <!-- Close button -->
          <button
            @click="handleClose"
            class="flex-shrink-0 hover:opacity-70 transition-opacity"
            aria-label="Close notification"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  show: {
    type: Boolean,
    default: false
  },
  message: {
    type: String,
    required: true
  },
  type: {
    type: String,
    default: 'info', // 'success', 'error', 'warning', 'info'
    validator: (value) => ['success', 'error', 'warning', 'info'].includes(value)
  }
});

const emit = defineEmits(['close']);

const toastClasses = computed(() => {
  const base = 'text-text-primary';
  switch (props.type) {
    case 'success':
      return `${base} bg-success/90 border-success`;
    case 'error':
      return `${base} bg-error/90 border-error`;
    case 'warning':
      return `${base} bg-warning/90 border-warning text-bg-main`;
    default:
      return `${base} bg-primary/90 border-primary`;
  }
});

function handleClose() {
  emit('close');
}
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: transform 0.25s ease-out, opacity 0.25s ease-out;
}

.toast-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.toast-leave-to {
  transform: translateY(-20px);
  opacity: 0;
}
</style>
