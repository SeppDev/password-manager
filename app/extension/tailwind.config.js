module.exports = {
  // content: [
  //   "./src/popup/*.{svelte,js,ts,jsx,tsx}",
  //   "./src/content/*.{svelte,js,ts,jsx,tsx}",
  // ],
  content: [
    "./src/popup/*.{html,js,svelte,ts}",
    "./src/content/**/*.{svelte,js,ts}", // if shadow content is separate
  ],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: process.env.NODE_ENV === "development" ? [{ pattern: /./ }] : [],
};
