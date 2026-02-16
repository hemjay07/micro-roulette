// Linera Direct GraphQL Integration
// Direct connection to Linera service - bypasses faucet API issues

import { ref, readonly } from 'vue';

// Use environment variables with fallbacks
const CHAIN_ID = import.meta.env.VITE_CHAIN_ID || '781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc';
const APP_ID = import.meta.env.VITE_APP_ID || '9b16ccbe34c686f959ad6d3ebe9dde35a1ab7cf73a99a470feae0b082be59059';
const SERVICE_PORT = import.meta.env.VITE_LINERA_SERVICE_PORT || '8082';

// Build the GraphQL endpoint URL
const GRAPHQL_ENDPOINT = `http://localhost:${SERVICE_PORT}/chains/${CHAIN_ID}/applications/${APP_ID}`;

// Demo mode state
const DEMO_MODE = import.meta.env.VITE_DEMO_MODE === 'true';
let demoLastSpinResult = null;
let demoSpinHistory = [];
let demoNumberStats = {}; // Track count for each number 0-36

// State
const chainId = ref(CHAIN_ID);
const appId = ref(APP_ID);
const isConnected = ref(false);
const isConnecting = ref(false);
const isInitialized = ref(false);
const isDemoMode = ref(DEMO_MODE);
const wallet = ref({ id: 'mock-wallet-' + Date.now() }); // Mock wallet for UI display
const client = ref(null);
const application = ref(null);
const error = ref(null);

// Initialize - just verify the endpoint is reachable
async function initialize() {
  if (isInitialized.value) return;

  // If demo mode, skip initialization but keep real chain ID
  if (DEMO_MODE) {
    isDemoMode.value = true;
    // Always use real Chain ID for display
    isInitialized.value = true;
    return true;
  }

  try {
    // Test connection to GraphQL endpoint
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ query: '{ chainId }' })
    });

    if (!response.ok) {
      throw new Error(`Service unreachable: ${response.status}`);
    }

    const data = await response.json();
    if (data.errors) {
      throw new Error(data.errors[0].message);
    }

    isInitialized.value = true;
    console.log('Linera service connected:', GRAPHQL_ENDPOINT);
    console.log('Chain ID verified:', data.data.chainId);

    return true;
  } catch (err) {
    error.value = `Failed to initialize Linera: ${err.message}`;
    console.error('Linera init error:', err);
    throw err;
  }
}

// Connect to Linera network
async function connect() {
  if (isConnecting.value) return; // Prevent double connection attempts
  isConnecting.value = true;

  try {
    await initialize();

    console.log('Connected to Linera, chain:', chainId.value);
    console.log('Connected to application:', appId.value);

    isConnected.value = true;
    isConnecting.value = false;
    error.value = null;

    return { chainId: chainId.value, appId: appId.value, demoMode: isDemoMode.value };

  } catch (err) {
    error.value = `Failed to connect: ${err.message}`;
    console.error('Connection error:', err);
    isConnected.value = false;
    isConnecting.value = false;
    throw err;
  }
}

// Disconnect from Linera network
function disconnect() {
  chainId.value = null;
  wallet.value = null;
  client.value = null;
  application.value = null;
  isConnected.value = false;
  console.log('Disconnected from Linera');
}

// Execute a GraphQL query via direct HTTP
async function query(graphqlQuery, variables = {}) {
  if (isDemoMode.value) {
    // Return mock data in demo mode
    return getMockQueryResponse(graphqlQuery);
  }

  if (!isConnected.value) {
    throw new Error('Not connected to Linera service');
  }

  const request = {
    query: graphqlQuery,
    variables,
  };

  try {
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = await response.json();

    if (data.errors) {
      console.error('GraphQL errors:', data.errors);
      throw new Error(data.errors[0].message);
    }

    return data;
  } catch (err) {
    console.error('Query error:', err);
    throw err;
  }
}

// Execute a GraphQL mutation via direct HTTP
async function mutate(mutation, variables = {}) {
  if (isDemoMode.value) {
    // Simulate mutation when service unavailable
    // Handle spin mutation - generate random result
    if (mutation.includes('Spin')) {
      demoLastSpinResult = Math.floor(Math.random() * 37); // 0-36
      const RED_NUMBERS = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
      const resultColor = demoLastSpinResult === 0 ? 'green' : (RED_NUMBERS.includes(demoLastSpinResult) ? 'red' : 'black');

      // Add to history
      demoSpinHistory.unshift({
        spinId: demoSpinHistory.length + 1,
        result: demoLastSpinResult,
        resultColor: resultColor,
        seedHash: Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')
      });
      if (demoSpinHistory.length > 20) demoSpinHistory.pop();

      // Track number stats
      demoNumberStats[demoLastSpinResult] = (demoNumberStats[demoLastSpinResult] || 0) + 1;

      // Return the result so animation can start immediately
      return { data: { spin: demoLastSpinResult, success: true } };
    }

    return { data: { success: true } };
  }

  if (!isConnected.value) {
    throw new Error('Not connected to Linera service');
  }

  const request = {
    query: mutation,
    variables,
  };

  try {
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = await response.json();

    if (data.errors) {
      console.error('GraphQL errors:', data.errors);
      throw new Error(data.errors[0].message);
    }

    return data;
  } catch (err) {
    console.error('Mutation error:', err);
    throw err;
  }
}

// Subscribe to notifications via polling (simplified)
function onNotification(callback) {
  console.log('Notification subscription registered (polling mode)');

  // Poll for changes every 3 seconds
  const pollInterval = setInterval(async () => {
    try {
      const result = await query('{ tableStatus { status spinNumber } }');
      if (result.data) {
        callback({
          reason: { NewBlock: { height: Date.now() } },
          data: result.data
        });
      }
    } catch (err) {
      console.error('Notification poll error:', err);
    }
  }, 3000);

  // Return cleanup function
  return () => clearInterval(pollInterval);
}

// Export composable
export function useLinera() {
  return {
    // State (read-only)
    chainId: readonly(chainId),
    appId: readonly(appId),
    isConnected: readonly(isConnected),
    isConnecting: readonly(isConnecting),
    isInitialized: readonly(isInitialized),
    isDemoMode: readonly(isDemoMode),
    wallet: readonly(wallet),
    error: readonly(error),

    // Methods
    initialize,
    connect,
    disconnect,
    query,
    mutate,
    onNotification,
  };
}

/**
 * Get hot numbers (most frequent) from demo stats
 */
function getHotNumbers() {
  const entries = Object.entries(demoNumberStats);
  if (entries.length === 0) return [];

  // Sort by count descending and take top 5
  return entries
    .sort((a, b) => b[1] - a[1])
    .slice(0, 5)
    .map(([num]) => parseInt(num));
}

/**
 * Get cold numbers (least frequent) from demo stats
 */
function getColdNumbers() {
  const entries = Object.entries(demoNumberStats);
  if (entries.length === 0) return [];

  // Sort by count ascending and take bottom 5
  return entries
    .sort((a, b) => a[1] - b[1])
    .slice(0, 5)
    .map(([num]) => parseInt(num));
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
      seedHash: Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('')
    };
  }

  // Parse limit parameter from spinHistory query if present
  let spinHistoryLimit = 20; // default limit
  const limitMatch = query.match(/spinHistory\s*\(\s*limit\s*:\s*(\d+)\s*\)/);
  if (limitMatch) {
    spinHistoryLimit = parseInt(limitMatch[1], 10);
  }

  return {
    data: {
      tableStatus: 'Open',
      spinNumber: String(demoSpinHistory.length + 1),
      roundTotal: '0',
      isBettingOpen: true,
      config: {
        minBet: '1000000',
        maxBet: '100000000',
        maxTotalBet: '500000000',
        houseEdgeBps: 270,
      },
      spinHistory: demoSpinHistory.slice(0, spinHistoryLimit),
      hotNumbers: getHotNumbers(),
      coldNumbers: getColdNumbers(),
      numberStats: demoNumberStats,
      lastSpin: lastSpin,
      lastResult: demoLastSpinResult,
      fairnessInfo: {
        nextSeedHash: Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join(''),
        currentSeed: demoLastSpinResult !== null ? Array.from({length: 64}, () => Math.floor(Math.random() * 16).toString(16)).join('') : '',
      },
      // Player info - returns zeros for fresh/new player (per contract behavior)
      playerInfo: {
        balance: '0',
        totalBets: 0,
        totalWagered: '0',
        totalWon: '0',
        totalLost: '0',
        biggestWin: '0',
        currentStreak: 0,
        bestStreak: 0,
      },
    },
  };
}
