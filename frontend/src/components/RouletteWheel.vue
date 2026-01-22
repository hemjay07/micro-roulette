<!-- src/components/RouletteWheel.vue -->
<template>
  <div class="relative inline-block">
    <!-- Outer rim / frame -->
    <div class="absolute inset-0 rounded-full border-8 border-amber-800 shadow-2xl"></div>

    <!-- Ball pointer at top -->
    <div class="absolute -top-4 left-1/2 -translate-x-1/2 z-20">
      <div class="w-6 h-6 bg-white rounded-full shadow-lg border-2 border-gray-300"></div>
    </div>

    <!-- SVG Wheel -->
    <svg
      :width="size"
      :height="size"
      :viewBox="`0 0 ${size} ${size}`"
      class="transform transition-transform"
      :class="{ 'wheel-spinning': isSpinning }"
      :style="wheelStyle"
    >
      <!-- Wheel background -->
      <circle
        :cx="center"
        :cy="center"
        :r="radius"
        fill="#1a1a1a"
      />

      <!-- Segments -->
      <g v-for="(num, index) in wheelOrder" :key="num">
        <path
          :d="getSegmentPath(index)"
          :fill="getSegmentColor(num)"
          stroke="#d4af37"
          stroke-width="1"
          class="cursor-pointer hover:brightness-110 transition-all"
          @click="$emit('numberClick', num)"
        />

        <!-- Number text -->
        <text
          :x="getTextX(index)"
          :y="getTextY(index)"
          :transform="getTextRotation(index)"
          fill="white"
          font-size="12"
          font-weight="bold"
          text-anchor="middle"
          dominant-baseline="middle"
          class="pointer-events-none"
        >
          {{ num }}
        </text>
      </g>

      <!-- Center hub -->
      <circle
        :cx="center"
        :cy="center"
        :r="innerRadius"
        fill="#2d2d2d"
        stroke="#d4af37"
        stroke-width="2"
      />

      <!-- Center decoration -->
      <circle
        :cx="center"
        :cy="center"
        :r="innerRadius - 10"
        fill="none"
        stroke="#8b4513"
        stroke-width="8"
      />

      <!-- Center logo area -->
      <circle
        :cx="center"
        :cy="center"
        :r="innerRadius - 25"
        fill="#1a1a1a"
      />
      <text
        :x="center"
        :y="center - 5"
        fill="#d4af37"
        font-size="14"
        font-weight="bold"
        text-anchor="middle"
      >
        MICRO
      </text>
      <text
        :x="center"
        :y="center + 12"
        fill="#d4af37"
        font-size="10"
        text-anchor="middle"
      >
        ROULETTE
      </text>
    </svg>

    <!-- Last result overlay -->
    <div
      v-if="lastResult !== null && !isSpinning"
      class="absolute inset-0 flex items-center justify-center pointer-events-none"
    >
      <div
        class="rounded-full w-20 h-20 flex items-center justify-center text-3xl font-bold shadow-lg animate-pulse"
        :class="lastResultClass"
      >
        {{ lastResult }}
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, watch } from 'vue';
import { WHEEL_ORDER, getNumberColor } from '../utils/roulette.js';

const props = defineProps({
  size: {
    type: Number,
    default: 400
  },
  isSpinning: {
    type: Boolean,
    default: false
  },
  targetRotation: {
    type: Number,
    default: 0
  },
  lastResult: {
    type: Number,
    default: null
  }
});

const emit = defineEmits(['numberClick', 'spinComplete']);

// Watch for spin completion (when isSpinning goes from true to false)
watch(() => props.isSpinning, (newVal, oldVal) => {
  if (oldVal === true && newVal === false && props.lastResult !== null) {
    emit('spinComplete', props.lastResult);
  }
});

// Computed values
const center = computed(() => props.size / 2);
const radius = computed(() => props.size / 2 - 10);
const innerRadius = computed(() => radius.value * 0.35);
const textRadius = computed(() => radius.value * 0.8);
const wheelOrder = WHEEL_ORDER;
const segmentAngle = 360 / 37;

// Wheel rotation style
const wheelStyle = computed(() => {
  if (props.isSpinning) {
    return {
      '--target-rotation': `${props.targetRotation}deg`
    };
  }
  return {
    transform: `rotate(${props.targetRotation}deg)`
  };
});

// Last result display class
const lastResultClass = computed(() => {
  if (props.lastResult === null) return '';
  const color = getNumberColor(props.lastResult);
  if (color === 'red') return 'bg-roulette-red text-white';
  if (color === 'black') return 'bg-roulette-black text-white';
  return 'bg-roulette-green text-white';
});

// Get segment fill color
function getSegmentColor(num) {
  const color = getNumberColor(num);
  if (color === 'red') return '#c41e3a';
  if (color === 'black') return '#1a1a1a';
  return '#0a5c0a';
}

// Calculate segment path (pie slice)
function getSegmentPath(index) {
  const startAngle = (index * segmentAngle - 90) * (Math.PI / 180);
  const endAngle = ((index + 1) * segmentAngle - 90) * (Math.PI / 180);

  const x1 = center.value + innerRadius.value * Math.cos(startAngle);
  const y1 = center.value + innerRadius.value * Math.sin(startAngle);
  const x2 = center.value + radius.value * Math.cos(startAngle);
  const y2 = center.value + radius.value * Math.sin(startAngle);
  const x3 = center.value + radius.value * Math.cos(endAngle);
  const y3 = center.value + radius.value * Math.sin(endAngle);
  const x4 = center.value + innerRadius.value * Math.cos(endAngle);
  const y4 = center.value + innerRadius.value * Math.sin(endAngle);

  const largeArc = segmentAngle > 180 ? 1 : 0;

  return `
    M ${x1} ${y1}
    L ${x2} ${y2}
    A ${radius.value} ${radius.value} 0 ${largeArc} 1 ${x3} ${y3}
    L ${x4} ${y4}
    A ${innerRadius.value} ${innerRadius.value} 0 ${largeArc} 0 ${x1} ${y1}
    Z
  `;
}

// Calculate text position X
function getTextX(index) {
  const angle = ((index + 0.5) * segmentAngle - 90) * (Math.PI / 180);
  return center.value + textRadius.value * Math.cos(angle);
}

// Calculate text position Y
function getTextY(index) {
  const angle = ((index + 0.5) * segmentAngle - 90) * (Math.PI / 180);
  return center.value + textRadius.value * Math.sin(angle);
}

// Calculate text rotation for readability
function getTextRotation(index) {
  const angle = (index + 0.5) * segmentAngle;
  const x = getTextX(index);
  const y = getTextY(index);
  // Rotate text to be readable (flip if on bottom half)
  const rotation = angle > 90 && angle < 270 ? angle + 180 : angle;
  return `rotate(${rotation}, ${x}, ${y})`;
}
</script>

<style scoped>
.wheel-spinning {
  animation: wheel-spin 5s cubic-bezier(0.2, 0.8, 0.2, 1) forwards;
}

@keyframes wheel-spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(var(--target-rotation));
  }
}
</style>
