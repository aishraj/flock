import { defineConfig } from "vite";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  base: "/flock",
  build: {
    minify: false,
  },
  plugins: [wasmPack(["./flock"])],
});
