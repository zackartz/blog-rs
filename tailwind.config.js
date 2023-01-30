/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "./src/**/*.rs"]
  },
  theme: {
    fontFamily: {
        sans: ["Inter", "sans-serif"],
        serif: ["Fira Code", "serif"]
    },
    extend: {},
  },
  plugins: [require('@tailwindcss/typography')],
}
