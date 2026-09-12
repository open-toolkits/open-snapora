import { execSync } from 'node:child_process';
import { resolve } from 'node:path';

const isDryRun = process.argv.includes('--dry-run');
const packages = ['packages/electron', 'packages/tauri'];

for (let i = 0; i < packages.length; i++) {
  const pkg = packages[i];
  const cwd = resolve(process.cwd(), pkg);
  console.log(`\n📦 Publishing ${pkg} via npm publish${isDryRun ? ' (dry-run)' : ''}...`);
  const dryFlag = isDryRun ? ' --dry-run' : '';
  execSync(`npm publish --access public --registry=https://registry.npmjs.org/${dryFlag}`, {
    cwd,
    stdio: 'inherit'
  });
  if (i < packages.length - 1 && !isDryRun) {
    console.log('⏳ Waiting 3s for npm registry packument synchronization...');
    await new Promise((r) => setTimeout(r, 3000));
  }
}
