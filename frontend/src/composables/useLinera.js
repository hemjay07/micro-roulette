// src/composables/useLinera.js
import { ref, readonly } from 'vue';

const FAUCET_URL = import.meta.env.VITE_LINERA_FAUCET_URL || 'https://faucet.testnet-conway.linera.net';
const DEMO_MODE = import.meta.env.VITE_DEMO_MODE === 'true';

// Demo mode state
let demoLastSpinResult = null;
let demoSpinHistory = [];

export function useLinera() {
  const chainId = ref(import.meta.env.VITE_CHAIN_ID || null);
  const appId = ref(import.meta.env.VITE_APP_ID || null);
  const isConnected = ref(false);
  const isConnecting = ref(false);
  const error = ref(null);
  const client = ref(null);
  const faucet = ref(null);
  const application = ref(null);
  const isDemoMode = ref(false);

  /**
   * Connect to Linera network
   */
  async function connect() {
    if (isConnecting.value || isConnected.value) return;

    isConnecting.value = true;
    error.value = null;

    try {
      // Dynamic import of linera-web
      const linera = await import('@linera/client');

      // Initialize WASM module - try different initialization methods
      if (typeof linera.default === 'function') {
        await linera.default();
      } else if (typeof linera.init === 'function') {
        await linera.init();
      }
      // If neither exists, the module may auto-initialize

      // Create faucet instance
      if (linera.Faucet) {
        faucet.value = new linera.Faucet(FAUCET_URL);

        // Create wallet
        const wallet = await faucet.value.createWallet();

        // Create client
        client.value = new linera.Client(wallet);

        // Claim a chain from faucet
        chainId.value = await faucet.value.claimChain(client.value);

        console.log('Connected to Linera');
        console.log('Chain ID:', chainId.value);

        // Get application handle if APP_ID is set
        if (appId.value) {
          application.value = await client.value.frontend().application(appId.value);
          console.log('Connected to application:', appId.value);
        }

        isConnected.value = true;
        return { chainId: chainId.value, appId: appId.value };
      } else {
        throw new Error('Linera client not available');
      }

    } catch (err) {
      console.error('Connection error:', err);

      // Fall back to demo mode
      console.log('Falling back to demo mode');
      isDemoMode.value = true;
      chainId.value = 'demo-chain-' + Math.random().toString(36).substring(7);
      isConnected.value = true;
      error.value = 'Running in demo mode (contract not deployed)';

      return { chainId: chainId.value, appId: appId.value, demoMode: true };
    } finally {
      isConnecting.value = false;
    }
  }

  /**
   * Execute a GraphQL query
   */
  async function query(graphqlQuery, variables = {}) {
    if (isDemoMode.value) {
      // Return mock data in demo mode
      return getMockQueryResponse(graphqlQuery);
    }

    if (!application.value) {
      throw new Error('Not connected to application');
    }

    const request = JSON.stringify({
      query: graphqlQuery,
      variables,
    });

    try {
      const response = await application.value.query(request);
      return JSON.parse(response);
    } catch (err) {
      console.error('Query error:', err);
      throw err;
    }
  }

  /**
   * Execute a GraphQL mutation
   */
  async function mutate(mutation, variables = {}) {
    if (isDemoMode.value) {
      // Simulate mutation in demo mode
      console.log('Demo mode mutation:', mutation, variables);

      // Handle spin mutation - generate random result
      if (mutation.includes('Spin')) {
        demoLastSpinResult = Math.floor(Math.random() * 37); // 0-36
        const colors = ['red', 'black', 'green'];
        const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
        const resultColor = demoLastSpinResult === 0 ? 'green' : (RED_NUMBERS.includes(demoLastSpinResult) ? 'red' : 'black');

        // Add to history
        demoSpinHistory.unshift({
          spinId: demoSpinHistory.length + 1,
          result: demoLastSpinResult,
          resultColor: resultColor,
          seedHash: 'demo-seed-' + Math.random().toString(36).substring(7)
        });
        if (demoSpinHistory.length > 20) demoSpinHistory.pop();

        console.log('Demo spin result:', demoLastSpinResult, resultColor);
      }

      return { data: { success: true } };
    }

    if (!application.value) {
      throw new Error('Not connected to application');
    }

    const request = JSON.stringify({
      query: mutation,
      variables,
    });

    try {
      const response = await application.value.query(request);
      return JSON.parse(response);
    } catch (err) {
      console.error('Mutation error:', err);
      throw err;
    }
  }

  /**
   * Subscribe to notifications (if supported)
   */
  function onNotification(callback) {
    if (application.value && application.value.subscribe) {
      return application.value.subscribe(callback);
    }
    return () => {}; // No-op unsubscribe
  }

  return {
    chainId: readonly(chainId),
    appId: readonly(appId),
    isConnected: readonly(isConnected),
    isConnecting: readonly(isConnecting),
    error: readonly(error),
    isDemoMode: readonly(isDemoMode),
    connect,
    query,
    mutate,
    onNotification,
  };
}

/**
 * Mock query responses for demo mode
 */
function getMockQueryResponse(query) {
  const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];

  // Build last spin data if we have a result
  let lastSpin = null;
  if (demoLastSpinResult !== null) {
    const resultColor = demoLastSpinResult === 0 ? 'green' : (RED_NUMBERS.includes(demoLastSpinResult) ? 'red' : 'black');
    lastSpin = {
      spinId: demoSpinHistory.length,
      result: demoLastSpinResult,
      resultColor: resultColor,
      seedHash: 'demo-seed-hash'
    };
  }

  return {
    data: {
      tableStatus: {
        status: 'Open',
        spinNumber: String(demoSpinHistory.length + 1),
        roundTotal: '0',
        isBettingOpen: true,
      },
      config: {
        minBet: '1000000',
        maxBet: '100000000',
        maxTotalBet: '500000000',
        houseEdgeBps: 270,
      },
      spinHistory: demoSpinHistory,
      hotNumbers: [],
      coldNumbers: [],
      lastSpin: lastSpin,
      fairnessInfo: {
        nextSeedHash: 'a1b2c3d4e5f6...',
        currentSeed: demoLastSpinResult !== null ? 'demo-revealed-seed' : '',
      },
    },
  };
}
