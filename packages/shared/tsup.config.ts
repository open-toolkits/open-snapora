import { defineConfig } from 'tsup';

export default defineConfig({
  entry: {
    index: 'src/index.ts',
    'core/index': 'src/core/index.ts',
    'protocol/index': 'src/protocol/index.ts',
    'i18n/index': 'src/i18n/index.ts',
  },
  outDir: 'dist',
  format: ['esm', 'cjs'],
  target: 'node20',
  dts: true,
  clean: false,
  sourcemap: true,
});
