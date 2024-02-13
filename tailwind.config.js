/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["./*.html", "./frontend/src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
