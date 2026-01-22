// tailwind.config.js
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'roulette-red': '#c41e3a',
        'roulette-black': '#1a1a1a',
        'roulette-green': '#0a5c0a',
        'felt-green': '#1b4d3e',
      },
      animation: {
        'spin-wheel': 'spin 5s cubic-bezier(0.2, 0.8, 0.2, 1) forwards',
      },
    },
  },
  plugins: [],
};
