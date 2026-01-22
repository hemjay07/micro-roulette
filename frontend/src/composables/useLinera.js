// src/composables/useLinera.js
import { ref, readonly } from 'vue';

const FAUCET_URL = import.meta.env.VITE_LINERA_FAUCET_URL || 'https://faucet.testnet-conway.linera.net';

export function useLinera() {
  const chainId = ref(null);
  const appId = ref(import.meta.env.VITE_APP_ID || null);
  const isConnected = ref(false);
  const isConnecting = ref(false);
  const error = ref(null);
  const client = ref(null);
  const faucet = ref(null);
  const application = ref(null);

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

      // Initialize WASM module
      await linera.default();

      // Create faucet instance
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

    } catch (err) {
      error.value = `Failed to connect: ${err.message}`;
      console.error('Connection error:', err);
      throw err;
    } finally {
      isConnecting.value = false;
    }
  }

  /**
   * Execute a GraphQL query
   */
  async function query(graphqlQuery, variables = {}) {
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
    connect,
    query,
    mutate,
    onNotification,
  };
}
