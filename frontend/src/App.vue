<!-- src/App.vue -->
<template>
  <div id="app" class="min-h-screen bg-bg-main text-text-primary">
    <!-- Navigation (shown on all pages except landing) -->
    <nav
      v-if="$route.name !== 'Landing'"
      class="sticky top-0 z-50 bg-bg-surface/80 backdrop-blur-safe border-b border-border"
    >
      <div class="container mx-auto px-4">
        <div class="flex items-center justify-between h-16">
          <!-- Logo -->
          <router-link to="/" class="flex items-center gap-3 group">
            <div class="w-10 h-10 bg-gradient-primary rounded-xl flex items-center justify-center shadow-md group-hover:shadow-lg transition-shadow">
              <svg class="w-6 h-6 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <circle cx="12" cy="12" r="10" />
                <path d="M12 2a10 10 0 0 1 0 20" />
                <circle cx="12" cy="12" r="3" />
              </svg>
            </div>
            <span class="text-xl font-bold text-text-primary">MicroRoulette</span>
          </router-link>

          <!-- Navigation Links -->
          <div class="hidden md:flex items-center gap-1">
            <router-link
              to="/play"
              class="nav-link"
              :class="{ 'nav-link-active': $route.name === 'Game' }"
            >
              Play
            </router-link>
            <router-link
              to="/history"
              class="nav-link"
              :class="{ 'nav-link-active': $route.name === 'History' }"
            >
              History
            </router-link>
            <router-link
              to="/leaderboard"
              class="nav-link"
              :class="{ 'nav-link-active': $route.name === 'Leaderboard' }"
            >
              Leaderboard
            </router-link>
            <router-link
              to="/rules"
              class="nav-link"
              :class="{ 'nav-link-active': $route.name === 'Rules' }"
            >
              How to Play
            </router-link>
          </div>

          <!-- Right side: Balance & Connection -->
          <div class="flex items-center gap-4">
            <!-- Balance Display -->
            <div class="hidden sm:flex items-center gap-2 px-4 py-2 bg-bg-elevated rounded-lg border border-border">
              <svg class="w-5 h-5 text-primary" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm.31-8.86c-1.77-.45-2.34-.94-2.34-1.67 0-.84.79-1.43 2.1-1.43 1.38 0 1.9.66 1.94 1.64h1.71c-.05-1.34-.87-2.57-2.49-2.97V5H10.9v1.69c-1.51.32-2.72 1.3-2.72 2.81 0 1.79 1.49 2.69 3.66 3.21 1.95.46 2.34 1.15 2.34 1.87 0 .53-.39 1.39-2.1 1.39-1.6 0-2.23-.72-2.32-1.64H8.04c.1 1.7 1.36 2.66 2.86 2.97V19h2.34v-1.67c1.52-.29 2.72-1.16 2.73-2.77-.01-2.2-1.9-2.96-3.66-3.42z"/>
              </svg>
              <span class="font-mono font-semibold text-text-primary">1,000</span>
              <span class="text-text-muted text-sm">chips</span>
            </div>

            <!-- Mobile Menu Button -->
            <button
              @click="mobileMenuOpen = !mobileMenuOpen"
              class="md:hidden p-2 text-text-muted hover:text-text-primary transition-colors"
            >
              <svg v-if="!mobileMenuOpen" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
              </svg>
              <svg v-else class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>
        </div>

        <!-- Mobile Menu -->
        <Transition name="slide-down">
          <div v-if="mobileMenuOpen" class="md:hidden py-4 border-t border-border">
            <div class="flex flex-col gap-2">
              <router-link
                to="/play"
                class="mobile-nav-link"
                @click="mobileMenuOpen = false"
              >
                Play Now
              </router-link>
              <router-link
                to="/history"
                class="mobile-nav-link"
                @click="mobileMenuOpen = false"
              >
                History
              </router-link>
              <router-link
                to="/leaderboard"
                class="mobile-nav-link"
                @click="mobileMenuOpen = false"
              >
                Leaderboard
              </router-link>
              <router-link
                to="/rules"
                class="mobile-nav-link"
                @click="mobileMenuOpen = false"
              >
                How to Play
              </router-link>
            </div>
          </div>
        </Transition>
      </div>
    </nav>

    <!-- Page Content -->
    <router-view v-slot="{ Component }">
      <Transition name="fade" mode="out-in">
        <component :is="Component" />
      </Transition>
    </router-view>

    <!-- Global Toast Container -->
    <Teleport to="body">
      <div id="toast-container" class="fixed top-4 right-4 z-[100] flex flex-col gap-2">
        <!-- Toasts will be rendered here -->
      </div>
    </Teleport>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const mobileMenuOpen = ref(false);
</script>

<style scoped>
.nav-link {
  @apply px-4 py-2 text-text-muted font-medium rounded-lg transition-all duration-200;
  @apply hover:text-text-primary hover:bg-bg-elevated;
}

.nav-link-active {
  @apply text-primary bg-primary/10;
}

.mobile-nav-link {
  @apply px-4 py-3 text-text-muted font-medium rounded-lg transition-all duration-200;
  @apply hover:text-text-primary hover:bg-bg-elevated;
  @apply active:bg-bg-elevated;
}

/* Page transitions */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s ease-out;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Mobile menu slide */
.slide-down-enter-active,
.slide-down-leave-active {
  transition: all 0.2s ease-out;
}

.slide-down-enter-from,
.slide-down-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}
</style>
