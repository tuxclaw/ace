/** @type {import('tailwindcss').Config} */
export default {
  content: ['./index.html', './src/**/*.{ts,tsx}'],
  theme: {
    extend: {
      colors: {
        ace: {
          bg: '#0d1117',
          card: '#161b22',
          border: '#30363d',
          accent: '#58a6ff',
          text: '#c9d1d9',
          muted: '#8b949e',
        },
      },
      boxShadow: {
        glow: '0 0 30px rgba(88, 166, 255, 0.22)',
      },
    },
  },
  plugins: [],
};
