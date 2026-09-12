#!/usr/bin/env node
import { cpSync, existsSync, mkdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDir = dirname(fileURLToPath(import.meta.url));
const bundledOverlay = resolve(currentDir, '../dist/overlay');

const args = process.argv.slice(2);
const command = args[0] || 'sync';
const targetRelative = args[1] || 'public/overlay';
const targetDir = resolve(process.cwd(), targetRelative);

if (command === 'sync') {
  if (!existsSync(bundledOverlay)) {
    console.error('[open-snapora-tauri] Error: Bundled overlay not found in package dist/overlay.');
    process.exit(1);
  }

  mkdirSync(targetDir, { recursive: true });
  cpSync(bundledOverlay, targetDir, { recursive: true });
  console.log(`[open-snapora-tauri] Successfully synced overlay assets to ${targetDir}`);
} else {
  console.log('Usage: open-snapora-tauri sync [targetDir=public/overlay]');
}
