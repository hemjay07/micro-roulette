<!-- src/pages/Landing.vue -->
<template>
  <div class="min-h-screen bg-bg-main overflow-hidden">
    <!-- Hero Section -->
    <section class="relative min-h-screen flex items-center justify-center px-4">
      <!-- Animated Background -->
      <div class="absolute inset-0 overflow-hidden">
        <!-- Gradient Orbs -->
        <div class="absolute top-1/4 left-1/4 w-96 h-96 bg-primary/20 rounded-full blur-3xl animate-float"></div>
        <div class="absolute bottom-1/4 right-1/4 w-80 h-80 bg-primary/10 rounded-full blur-3xl animate-float" style="animation-delay: -1.5s;"></div>
        <div class="absolute top-1/2 left-1/2 w-64 h-64 bg-success/10 rounded-full blur-3xl animate-float" style="animation-delay: -3s;"></div>

        <!-- Grid Pattern -->
        <div class="absolute inset-0 bg-[linear-gradient(rgba(245,158,11,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(245,158,11,0.03)_1px,transparent_1px)] bg-[size:64px_64px]"></div>
      </div>

      <!-- Content -->
      <div class="relative z-10 max-w-5xl mx-auto text-center">
        <!-- Logo Badge -->
        <div class="inline-flex items-center gap-2 px-4 py-2 bg-bg-surface border border-border rounded-full mb-8 animate-fade-in">
          <span class="w-2 h-2 bg-success rounded-full animate-pulse"></span>
          <span class="text-text-muted text-sm font-medium">Live on Linera Testnet</span>
        </div>

        <!-- Main Heading -->
        <h1 class="text-5xl md:text-7xl font-bold text-text-primary mb-6 animate-slide-up">
          Provably Fair
          <span class="text-gradient block mt-2">On-Chain Roulette</span>
        </h1>

        <!-- Subheading -->
        <p class="text-xl md:text-2xl text-text-muted max-w-2xl mx-auto mb-12 animate-slide-up" style="animation-delay: 0.1s;">
          Experience true transparency in gambling. Every spin is verifiable, every outcome is fair.
        </p>

        <!-- CTA Buttons -->
        <div class="flex flex-col sm:flex-row items-center justify-center gap-4 animate-slide-up" style="animation-delay: 0.2s;">
          <router-link
            to="/play"
            class="group relative px-8 py-4 bg-primary hover:bg-primary-hover text-white font-bold text-lg rounded-xl transition-all duration-200 shadow-lg hover:shadow-xl"
          >
            <span class="relative z-10 flex items-center gap-2">
              Play Now
              <svg class="w-5 h-5 group-hover:translate-x-1 transition-transform" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </span>
          </router-link>

          <router-link
            to="/rules"
            class="px-8 py-4 bg-bg-elevated hover:bg-bg-surface text-text-primary font-semibold text-lg rounded-xl border border-border hover:border-primary/50 transition-all duration-200"
          >
            Learn How It Works
          </router-link>
        </div>

        <!-- Animated Wheel Preview -->
        <div class="mt-16 relative animate-fade-in" style="animation-delay: 0.3s;">
          <div class="relative w-64 h-64 md:w-80 md:h-80 mx-auto">
            <!-- Glow Effect -->
            <div class="absolute inset-0 bg-primary/10 rounded-full blur-2xl"></div>

            <!-- Wheel -->
            <svg class="w-full h-full animate-spin-slow" viewBox="0 0 200 200">
              <defs>
                <linearGradient id="wheelGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" style="stop-color:#F59E0B"/>
                  <stop offset="100%" style="stop-color:#D97706"/>
                </linearGradient>
              </defs>
              <!-- Outer Ring -->
              <circle cx="100" cy="100" r="95" fill="none" stroke="url(#wheelGradient)" stroke-width="3" opacity="0.5"/>
              <!-- Segments -->
              <g transform="translate(100,100)">
                <circle r="80" fill="#1A1A2E"/>
                <!-- Number segments (simplified visual) -->
                <g class="opacity-80">
                  <path d="M0,-75 L0,-85 A85,85 0 0,1 22,-82 L20,-72 A75,75 0 0,0 0,-75" fill="#DC2626"/>
                  <path d="M22,-82 L20,-72 A75,75 0 0,1 43,-63 L47,-72 A85,85 0 0,0 22,-82" fill="#1F2937"/>
                  <path d="M43,-63 L47,-72 A85,85 0 0,1 63,-47 L56,-42 A75,75 0 0,0 43,-63" fill="#DC2626"/>
                  <path d="M63,-47 L56,-42 A75,75 0 0,1 75,-20 L85,-22 A85,85 0 0,0 63,-47" fill="#1F2937"/>
                  <path d="M75,-20 L85,-22 A85,85 0 0,1 85,0 L75,0 A75,75 0 0,0 75,-20" fill="#DC2626"/>
                  <path d="M85,0 L75,0 A75,75 0 0,1 75,20 L85,22 A85,85 0 0,0 85,0" fill="#1F2937"/>
                  <path d="M75,20 L85,22 A85,85 0 0,1 63,47 L56,42 A75,75 0 0,0 75,20" fill="#DC2626"/>
                  <path d="M63,47 L56,42 A75,75 0 0,1 43,63 L47,72 A85,85 0 0,0 63,47" fill="#1F2937"/>
                </g>
                <!-- Center -->
                <circle r="25" fill="#0F0F1A"/>
                <circle r="20" fill="url(#wheelGradient)"/>
              </g>
            </svg>

            <!-- Ball Indicator -->
            <div class="absolute top-2 left-1/2 -translate-x-1/2 w-4 h-4 bg-white rounded-full shadow-lg"></div>
          </div>
        </div>
      </div>
    </section>

    <!-- Features Section -->
    <section class="py-24 px-4 bg-bg-surface border-t border-border">
      <div class="max-w-6xl mx-auto">
        <h2 class="text-3xl md:text-4xl font-bold text-center text-text-primary mb-4">
          Why MicroRoulette?
        </h2>
        <p class="text-text-muted text-center mb-16 max-w-2xl mx-auto">
          Built on Linera's microchain architecture for instant finality and true provable fairness.
        </p>

        <div class="grid md:grid-cols-3 gap-8">
          <!-- Feature 1 -->
          <div class="card-interactive group">
            <div class="w-14 h-14 bg-primary/20 rounded-xl flex items-center justify-center mb-6 group-hover:bg-primary/30 transition-colors">
              <svg class="w-7 h-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
              </svg>
            </div>
            <h3 class="text-xl font-bold text-text-primary mb-3">Provably Fair</h3>
            <p class="text-text-muted">
              Every spin uses cryptographic commitments that can be independently verified. No trust required.
            </p>
          </div>

          <!-- Feature 2 -->
          <div class="card-interactive group">
            <div class="w-14 h-14 bg-success/20 rounded-xl flex items-center justify-center mb-6 group-hover:bg-success/30 transition-colors">
              <svg class="w-7 h-7 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
              </svg>
            </div>
            <h3 class="text-xl font-bold text-text-primary mb-3">Instant Settlement</h3>
            <p class="text-text-muted">
              Linera microchains enable sub-second finality. Your winnings are available immediately.
            </p>
          </div>

          <!-- Feature 3 -->
          <div class="card-interactive group">
            <div class="w-14 h-14 bg-warning/20 rounded-xl flex items-center justify-center mb-6 group-hover:bg-warning/30 transition-colors">
              <svg class="w-7 h-7 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" />
              </svg>
            </div>
            <h3 class="text-xl font-bold text-text-primary mb-3">Fully On-Chain</h3>
            <p class="text-text-muted">
              Game logic runs entirely on the blockchain. Transparent, auditable, and censorship-resistant.
            </p>
          </div>
        </div>
      </div>
    </section>

    <!-- Stats Section -->
    <section class="py-20 px-4">
      <div class="max-w-4xl mx-auto">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
          <div>
            <div class="text-4xl md:text-5xl font-bold text-gradient mb-2">2.7%</div>
            <div class="text-text-muted">House Edge</div>
          </div>
          <div>
            <div class="text-4xl md:text-5xl font-bold text-gradient mb-2">36x</div>
            <div class="text-text-muted">Max Payout</div>
          </div>
          <div>
            <div class="text-4xl md:text-5xl font-bold text-gradient mb-2">&lt;1s</div>
            <div class="text-text-muted">Settlement</div>
          </div>
          <div>
            <div class="text-4xl md:text-5xl font-bold text-gradient mb-2">100%</div>
            <div class="text-text-muted">Verifiable</div>
          </div>
        </div>
      </div>
    </section>

    <!-- CTA Section -->
    <section class="py-24 px-4 bg-bg-surface border-t border-border">
      <div class="max-w-3xl mx-auto text-center">
        <h2 class="text-3xl md:text-4xl font-bold text-text-primary mb-6">
          Ready to Spin?
        </h2>
        <p class="text-xl text-text-muted mb-8">
          Join the future of fair gaming. No sign-up required.
        </p>
        <router-link
          to="/play"
          class="inline-flex items-center gap-2 px-8 py-4 bg-primary hover:bg-primary-hover text-white font-bold text-lg rounded-xl transition-all duration-200 shadow-lg hover:shadow-xl"
        >
          Start Playing
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
        </router-link>
      </div>
    </section>

    <!-- Footer -->
    <footer class="py-8 px-4 border-t border-border">
      <div class="max-w-6xl mx-auto flex flex-col md:flex-row items-center justify-between gap-4">
        <div class="flex items-center gap-2">
          <div class="w-8 h-8 bg-gradient-primary rounded-lg flex items-center justify-center">
            <svg class="w-5 h-5 text-white" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10" />
              <path d="M12 2a10 10 0 0 1 0 20" />
              <circle cx="12" cy="12" r="3" />
            </svg>
          </div>
          <span class="font-semibold text-text-primary">MicroRoulette</span>
        </div>

        <div class="flex items-center gap-6 text-sm text-text-muted">
          <router-link to="/rules" class="hover:text-text-primary transition-colors">How to Play</router-link>
          <a href="https://linera.dev" target="_blank" rel="noopener" class="hover:text-text-primary transition-colors">Built on Linera</a>
        </div>

        <div class="text-sm text-text-dim">
          &copy; 2026 MicroRoulette. Gamble responsibly.
        </div>
      </div>
    </footer>
  </div>
</template>

<style scoped>
@keyframes spin-slow {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.animate-spin-slow {
  animation: spin-slow 20s linear infinite;
}
</style>
