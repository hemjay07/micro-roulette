// vite.config.js
import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

export default defineConfig({
  plugins: [vue()],

  build: {
    target: 'esnext',
    rollupOptions: {
      output: {
        manualChunks: undefined,
      },
    },
  },

  // CRITICAL: Exclude @linera/client from optimization (it's WASM)
  optimizeDeps: {
    exclude: ['@linera/client'],
  },

  // CRITICAL: Required headers for SharedArrayBuffer (used by WASM)
  server: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
    host: '0.0.0.0',
    port: 8080,
  },

  preview: {
    headers: {
      'Cross-Origin-Opener-Policy': 'same-origin',
      'Cross-Origin-Embedder-Policy': 'require-corp',
    },
  },
});
