// src/composables/useRoulette.js
import { ref, computed } from 'vue';

export function useRoulette(query, mutate) {
  // State
  const spinHistory = ref([]);
  const hotNumbers = ref([]);
  const coldNumbers = ref([]);
  const lastResult = ref(null);
  const lastSpinProof = ref(null);
  const isSpinning = ref(false);
  const tableStatus = ref('Open');
  const spinNumber = ref(0);
  const roundTotal = ref('0');
  const config = ref({
    minBet: '1000000',
    maxBet: '100000000',
    maxTotalBet: '500000000',
    houseEdgeBps: 270,
  });

  // Computed
  const isBettingOpen = computed(() => tableStatus.value === 'Open');

  /**
   * Fetch current table state from chain
   */
  async function fetchTableState() {
    try {
      const result = await query(`
        query {
          tableStatus
          spinNumber
          roundTotal
          isBettingOpen
          config {
            minBet
            maxBet
            maxTotalBet
            houseEdgeBps
          }
          hotNumbers
          coldNumbers
          lastResult
          fairnessInfo {
            serverSeedHash
            revealedServerSeed
            lastClientSeed
            lastResult
            canVerify
          }
        }
      `);

      if (result.data) {
        const data = result.data;

        // tableStatus is now a plain string
        if (data.tableStatus) {
          tableStatus.value = data.tableStatus;
        }

        // spinNumber is now a separate field
        if (data.spinNumber !== undefined) {
          spinNumber.value = data.spinNumber;
        }

        // roundTotal is now a separate field
        if (data.roundTotal) {
          roundTotal.value = data.roundTotal;
        }

        if (data.config) {
          config.value = data.config;
        }

        if (data.hotNumbers) {
          hotNumbers.value = data.hotNumbers;
        }

        if (data.coldNumbers) {
          coldNumbers.value = data.coldNumbers;
        }

        if (data.lastResult !== undefined) {
          lastResult.value = data.lastResult;
        }

        if (data.fairnessInfo) {
          lastSpinProof.value = data.fairnessInfo;
        }
      }
    } catch (err) {
      console.error('Failed to fetch table state:', err);
    }
  }

  /**
   * Execute a spin
   */
  async function spin(bets) {
    if (isSpinning.value) return;

    isSpinning.value = true;

    try {
      // Generate client seed from timestamp + random
      const clientSeed = `${Date.now()}_${Math.random().toString(36).substring(7)}`;

      // First place the bets if any
      if (bets && bets.length > 0) {
        await mutate(`
          mutation PlaceBet($bets: String!) {
            placeBet(betsJson: $bets)
          }
        `, { bets: JSON.stringify(bets) });
      }

      // Execute spin
      await mutate(`
        mutation Spin($seed: String!) {
          spin(clientSeed: $seed)
        }
      `, { seed: clientSeed });

      // Wait for animation (5 seconds)
      await new Promise(resolve => setTimeout(resolve, 5000));

      // Fetch updated state
      await fetchTableState();

    } catch (err) {
      console.error('Spin failed:', err);
      throw err;
    } finally {
      isSpinning.value = false;
    }
  }

  /**
   * Verify fairness proof
   */
  async function verifyFairness(serverSeed, clientSeed, nonce) {
    const result = await query(`
      query VerifyFairness($serverSeed: String!, $clientSeed: String!, $nonce: String!) {
        verifyFairness(serverSeed: $serverSeed, clientSeed: $clientSeed, nonce: $nonce) {
          result
          resultColor
          combinedHash
          isValid
        }
      }
    `, { serverSeed, clientSeed, nonce: nonce.toString() });

    return result.data?.verifyFairness;
  }

  return {
    spinHistory,
    hotNumbers,
    coldNumbers,
    lastResult,
    lastSpinProof,
    isSpinning,
    tableStatus,
    spinNumber,
    roundTotal,
    config,
    isBettingOpen,
    fetchTableState,
    spin,
    verifyFairness,
  };
}
