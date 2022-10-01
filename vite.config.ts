import { defineConfig } from "vite";
import wasmPack from "vite-plugin-wasm-pack";

export default defineConfig({
  base: "/experiments/boids",
  build: {
    minify: false,
  },
  plugins: [wasmPack(["./flock"])],
});
