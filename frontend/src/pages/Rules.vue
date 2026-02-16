<!-- src/pages/Rules.vue -->
<template>
  <div class="min-h-screen bg-bg-main py-8">
    <div class="container mx-auto px-4 max-w-4xl">
      <!-- Page Header -->
      <div class="text-center mb-12">
        <h1 class="text-4xl font-bold text-text-primary mb-3">How to Play</h1>
        <p class="text-text-muted text-lg">Learn the rules and start winning</p>
      </div>

      <!-- Quick Start -->
      <div class="card-elevated border-primary/30 mb-8">
        <h2 class="text-xl font-bold text-text-primary mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
          </svg>
          Quick Start
        </h2>
        <ol class="space-y-3 text-text-muted">
          <li class="flex items-start gap-3">
            <span class="flex-shrink-0 w-6 h-6 rounded-full bg-primary/20 text-primary text-sm font-bold flex items-center justify-center">1</span>
            <span>Select your chip value (1, 5, 10, 25, 100, or 500)</span>
          </li>
          <li class="flex items-start gap-3">
            <span class="flex-shrink-0 w-6 h-6 rounded-full bg-primary/20 text-primary text-sm font-bold flex items-center justify-center">2</span>
            <span>Click on the betting board to place your bets</span>
          </li>
          <li class="flex items-start gap-3">
            <span class="flex-shrink-0 w-6 h-6 rounded-full bg-primary/20 text-primary text-sm font-bold flex items-center justify-center">3</span>
            <span>Click <strong class="text-primary">SPIN</strong> to start the wheel</span>
          </li>
          <li class="flex items-start gap-3">
            <span class="flex-shrink-0 w-6 h-6 rounded-full bg-primary/20 text-primary text-sm font-bold flex items-center justify-center">4</span>
            <span>Win if the ball lands on your bet!</span>
          </li>
        </ol>
      </div>

      <!-- Bet Types -->
      <div class="card mb-8">
        <h2 class="text-xl font-bold text-text-primary mb-6">Bet Types & Payouts</h2>

        <!-- Inside Bets -->
        <div class="mb-8">
          <h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-primary"></span>
            Inside Bets
          </h3>
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-border">
                  <th class="px-4 py-3 text-left text-sm font-semibold text-text-muted">Bet Type</th>
                  <th class="px-4 py-3 text-left text-sm font-semibold text-text-muted">Description</th>
                  <th class="px-4 py-3 text-right text-sm font-semibold text-text-muted">Payout</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border">
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Straight</td>
                  <td class="px-4 py-3 text-text-muted">Bet on a single number</td>
                  <td class="px-4 py-3 text-right font-mono text-success">35:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Split</td>
                  <td class="px-4 py-3 text-text-muted">Bet on two adjacent numbers</td>
                  <td class="px-4 py-3 text-right font-mono text-success">17:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Street</td>
                  <td class="px-4 py-3 text-text-muted">Bet on three numbers in a row</td>
                  <td class="px-4 py-3 text-right font-mono text-success">11:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Corner</td>
                  <td class="px-4 py-3 text-text-muted">Bet on four numbers that meet at a corner</td>
                  <td class="px-4 py-3 text-right font-mono text-success">8:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Six Line</td>
                  <td class="px-4 py-3 text-text-muted">Bet on six numbers (two adjacent rows)</td>
                  <td class="px-4 py-3 text-right font-mono text-success">5:1</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- Outside Bets -->
        <div>
          <h3 class="text-lg font-semibold text-text-primary mb-4 flex items-center gap-2">
            <span class="w-3 h-3 rounded-full bg-warning"></span>
            Outside Bets
          </h3>
          <div class="overflow-x-auto">
            <table class="w-full">
              <thead>
                <tr class="border-b border-border">
                  <th class="px-4 py-3 text-left text-sm font-semibold text-text-muted">Bet Type</th>
                  <th class="px-4 py-3 text-left text-sm font-semibold text-text-muted">Description</th>
                  <th class="px-4 py-3 text-right text-sm font-semibold text-text-muted">Payout</th>
                </tr>
              </thead>
              <tbody class="divide-y divide-border">
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Red / Black</td>
                  <td class="px-4 py-3 text-text-muted">Bet on color</td>
                  <td class="px-4 py-3 text-right font-mono text-success">1:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Odd / Even</td>
                  <td class="px-4 py-3 text-text-muted">Bet on odd or even numbers</td>
                  <td class="px-4 py-3 text-right font-mono text-success">1:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">1-18 / 19-36</td>
                  <td class="px-4 py-3 text-text-muted">Bet on low or high numbers</td>
                  <td class="px-4 py-3 text-right font-mono text-success">1:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Dozens</td>
                  <td class="px-4 py-3 text-text-muted">Bet on 1-12, 13-24, or 25-36</td>
                  <td class="px-4 py-3 text-right font-mono text-success">2:1</td>
                </tr>
                <tr>
                  <td class="px-4 py-3 font-medium text-text-primary">Columns</td>
                  <td class="px-4 py-3 text-text-muted">Bet on a vertical column of 12 numbers</td>
                  <td class="px-4 py-3 text-right font-mono text-success">2:1</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </div>

      <!-- Provably Fair -->
      <div class="card mb-8">
        <h2 class="text-xl font-bold text-text-primary mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
          </svg>
          Provably Fair
        </h2>
        <p class="text-text-muted mb-4">
          Every spin uses cryptographic commitments to ensure fairness. Here's how it works:
        </p>
        <div class="space-y-4">
          <div class="bg-bg-main rounded-lg p-4">
            <h4 class="font-semibold text-text-primary mb-2">1. Before the Spin</h4>
            <p class="text-text-muted text-sm">
              The server commits to a secret seed by publishing its SHA-256 hash. This hash is visible before you place your bet.
            </p>
          </div>
          <div class="bg-bg-main rounded-lg p-4">
            <h4 class="font-semibold text-text-primary mb-2">2. Your Input</h4>
            <p class="text-text-muted text-sm">
              You provide a client seed that gets combined with the server seed. This ensures neither party can predict the outcome.
            </p>
          </div>
          <div class="bg-bg-main rounded-lg p-4">
            <h4 class="font-semibold text-text-primary mb-2">3. Result Calculation</h4>
            <p class="text-text-muted text-sm">
              The final number is derived from: <code class="bg-bg-elevated px-2 py-1 rounded text-primary text-xs">SHA256(server_seed + client_seed + nonce) mod 37</code>
            </p>
          </div>
          <div class="bg-bg-main rounded-lg p-4">
            <h4 class="font-semibold text-text-primary mb-2">4. Verification</h4>
            <p class="text-text-muted text-sm">
              After each spin, the server seed is revealed. You can verify that its hash matches the commitment and recalculate the result yourself.
            </p>
          </div>
        </div>
      </div>

      <!-- House Edge -->
      <div class="card mb-8">
        <h2 class="text-xl font-bold text-text-primary mb-4">House Edge</h2>
        <div class="flex items-center gap-4 p-4 bg-bg-main rounded-lg">
          <div class="text-4xl font-bold text-primary">2.7%</div>
          <p class="text-text-muted">
            European roulette with a single zero (0). This gives a house edge of 2.7%, meaning on average, you keep 97.3% of your bets over time.
          </p>
        </div>
      </div>

      <!-- Tips -->
      <div class="card">
        <h2 class="text-xl font-bold text-text-primary mb-4 flex items-center gap-2">
          <svg class="w-6 h-6 text-warning" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
          </svg>
          Tips
        </h2>
        <ul class="space-y-3 text-text-muted">
          <li class="flex items-start gap-3">
            <svg class="w-5 h-5 text-success flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>Set a budget and stick to it. Never bet more than you can afford to lose.</span>
          </li>
          <li class="flex items-start gap-3">
            <svg class="w-5 h-5 text-success flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>Outside bets (red/black, odd/even) have the best odds but lower payouts.</span>
          </li>
          <li class="flex items-start gap-3">
            <svg class="w-5 h-5 text-success flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>Use the "Repeat" button to quickly place the same bets again.</span>
          </li>
          <li class="flex items-start gap-3">
            <svg class="w-5 h-5 text-success flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>Right-click on a bet position to remove the bet.</span>
          </li>
          <li class="flex items-start gap-3">
            <svg class="w-5 h-5 text-success flex-shrink-0 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span>Always verify the fairness of your spins - transparency is key!</span>
          </li>
        </ul>
      </div>

      <!-- CTA -->
      <div class="text-center mt-12">
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
    </div>
  </div>
</template>
