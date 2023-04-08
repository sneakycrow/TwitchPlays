import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontSize: {
        "6xl": "6rem",
        "7xl": "7rem",
      },
    },
  },
  plugins: [],
} satisfies Config;
