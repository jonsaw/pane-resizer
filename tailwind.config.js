/** @type {import('tailwindcss').Config} */
const config = {
  darkMode: ["class"],
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  safelist: ["dark"],
  theme: {
    colors: {
      brand: "#00ff00",
      muted: "#f0f0f0",
      active: "#00cbff",
    },
  },
};

export default config;
