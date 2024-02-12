/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["./frontend/*.html", "./frontend/src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
