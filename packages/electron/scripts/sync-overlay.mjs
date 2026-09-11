import { cpSync, existsSync, mkdirSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDir = dirname(fileURLToPath(import.meta.url));
const overlayDist = resolve(currentDir, '../../overlay/dist');
const electronOverlayDist = resolve(currentDir, '../dist/overlay');

if (existsSync(overlayDist)) {
  mkdirSync(electronOverlayDist, { recursive: true });
  cpSync(overlayDist, electronOverlayDist, { recursive: true });
  console.log('[open-snapora-electron] Synced overlay dist assets successfully.');
} else {
  console.warn('[open-snapora-electron] Warning: @open-snapora/overlay dist not found, build overlay first.');
}
