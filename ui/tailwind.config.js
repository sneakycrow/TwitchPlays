/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            fontSize: {
                "6xl": "6rem",
                "7xl": "7rem",
            }
        },
    },
    plugins: [],
}

