import { crx } from "@crxjs/vite-plugin";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";
import manifest from "./src/manifest.config";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [svelte(), crx({ manifest }), tailwindcss()],
  server: {
    port: 5173,
    strictPort: false,
    hmr: {
      clientPort: 5173,
    },
  },
  build: {
    sourcemap: true,
  },
});
