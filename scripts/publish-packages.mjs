import { execSync } from 'node:child_process';
import { resolve } from 'node:path';

const isDryRun = process.argv.includes('--dry-run');
const packages = ['packages/electron', 'packages/tauri'];

for (const pkg of packages) {
  const cwd = resolve(process.cwd(), pkg);
  console.log(`\n📦 Publishing ${pkg} via npm publish${isDryRun ? ' (dry-run)' : ''}...`);
  const dryFlag = isDryRun ? ' --dry-run' : '';
  execSync(`npm publish --access public --registry=https://registry.npmjs.org/${dryFlag}`, {
    cwd,
    stdio: 'inherit'
  });
}
