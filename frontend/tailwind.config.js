// tailwind.config.js
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Design System - Crypto Casino Dark Theme (Gold Accent)
        'bg-main': '#0A0A0F',
        'bg-surface': '#12121A',
        'bg-elevated': '#1A1A24',
        'primary': '#F59E0B',
        'primary-hover': '#FBBF24',
        'primary-muted': '#D97706',
        'text-primary': '#FAFAFA',
        'text-muted': '#A1A1AA',
        'text-dim': '#71717A',
        'border': '#27272A',
        'border-light': '#3F3F46',
        'success': '#22C55E',
        'success-muted': '#16A34A',
        'error': '#EF4444',
        'error-muted': '#DC2626',
        'warning': '#F97316',
        'warning-muted': '#EA580C',

        // Roulette-specific colors
        'roulette-red': '#DC2626',
        'roulette-black': '#18181B',
        'roulette-green': '#16A34A',
        'felt': '#12121A',

      },
      fontFamily: {
        'sans': ['Inter', 'system-ui', 'sans-serif'],
        'mono': ['JetBrains Mono', 'monospace'],
      },
      animation: {
        'spin-wheel': 'spin 5s cubic-bezier(0.2, 0.8, 0.2, 1) forwards',
        'float': 'float 3s ease-in-out infinite',
        'slide-up': 'slide-up 0.3s ease-out',
        'slide-down': 'slide-down 0.3s ease-out',
        'fade-in': 'fade-in 0.2s ease-out',
        'scale-in': 'scale-in 0.2s ease-out',
        'shimmer': 'shimmer 2s linear infinite',
      },
      keyframes: {
        'float': {
          '0%, 100%': { transform: 'translateY(0)' },
          '50%': { transform: 'translateY(-10px)' },
        },
        'slide-up': {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        'slide-down': {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        'fade-in': {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        'scale-in': {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
        'shimmer': {
          '0%': { backgroundPosition: '-200% 0' },
          '100%': { backgroundPosition: '200% 0' },
        },
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(var(--tw-gradient-stops))',
        'gradient-primary': 'linear-gradient(135deg, #F59E0B 0%, #D97706 100%)',
        'gradient-dark': 'linear-gradient(180deg, #0A0A0F 0%, #12121A 100%)',
        'shimmer': 'linear-gradient(90deg, transparent, rgba(245, 158, 11, 0.1), transparent)',
      },
      borderRadius: {
        'lg': '8px',
        'xl': '12px',
        '2xl': '16px',
      },
      transitionTimingFunction: {
        'bounce-in': 'cubic-bezier(0.68, -0.55, 0.265, 1.55)',
        'smooth': 'cubic-bezier(0.4, 0, 0.2, 1)',
      },
    },
  },
  plugins: [],
};
