import { defineConfig } from 'tsup';

export default defineConfig({
  entry: {
    index: 'src/index.ts',
    bridge: 'src/bridge.ts',
  },
  outDir: 'dist',
  format: ['esm'],
  target: 'es2022',
  dts: {
    resolve: ['@open-snapora/shared'],
  },
  noExternal: ['@open-snapora/shared'],
  sourcemap: true,
  clean: false,
  splitting: false,
  external: ['@tauri-apps/api', '@tauri-apps/api/core', '@tauri-apps/api/event'],
});
