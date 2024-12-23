/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,rs}', '*.html'],
  theme: {
    extend: {
      fontFamily: {
        clash: ['ClashDisplay-Regular', 'sans-serif'],
      },
    },
  },
  plugins: [],
};
