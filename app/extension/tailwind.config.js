module.exports = {
    content: ["./src/**/*.{svelte,js,ts,jsx,tsx}"],
    theme: {
        extend: {},
    },
    plugins: [],
    safelist: process.env.NODE_ENV === "development" ? [{ pattern: /./ }] : [],
};
