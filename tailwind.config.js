/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["./*.html", "./app/src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
