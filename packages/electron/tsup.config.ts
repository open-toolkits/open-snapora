import { defineConfig } from 'tsup';

export default defineConfig({
  entry: {
    'main/index': 'src/main.ts',
    'preload/index': 'src/preload.ts',
    'preload/auto': 'src/preload/auto-preload.ts',
    'overlay/preload': 'src/preload/overlay-preload.ts',
    'pinned/preload': 'src/preload/pinned-preload.ts',
  },
  outDir: 'dist',
  format: ['esm', 'cjs'],
  target: 'node20',
  platform: 'node',
  dts: {
    resolve: ['@open-snapora/shared'],
  },
  noExternal: ['@open-snapora/shared'],
  shims: true,
  sourcemap: true,
  clean: false,
  splitting: false,
  external: ['electron'],
  outExtension({ format }) {
    return {
      js: format === 'esm' ? '.mjs' : '.cjs',
    };
  },
});
