import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const srcDir = fileURLToPath(new URL('./src', import.meta.url));

export default defineConfig({
  root: srcDir,
  base: './',
  build: {
    outDir: fileURLToPath(new URL('./dist', import.meta.url)),
    emptyOutDir: false,
    sourcemap: true,
    rollupOptions: {
      input: {
        overlay: fileURLToPath(new URL('./src/index.html', import.meta.url)),
        pinned: fileURLToPath(new URL('./src/pinned.html', import.meta.url)),
        dev: fileURLToPath(new URL('./src/dev.html', import.meta.url)),
      },
    },
  },
});
