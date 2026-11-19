import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  server: {
    port: 1420,
    strictPort: true,
  },
  build: {
    // WebKitGTK di Linux; samakan dengan target webview Tauri.
    target: "safari13",
    outDir: "dist",
  },
});
