import { execSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const isDryRun = process.argv.includes('--dry-run');
const packages = ['packages/electron', 'packages/tauri'];

for (let i = 0; i < packages.length; i++) {
  const pkgRel = packages[i];
  const cwd = resolve(process.cwd(), pkgRel);
  const pkgJson = JSON.parse(readFileSync(resolve(cwd, 'package.json'), 'utf8'));
  const { name, version } = pkgJson;

  console.log(`\n========================================`);
  console.log(`📦 Checking ${name}@${version}...`);
  console.log(`========================================`);

  if (!isDryRun) {
    try {
      const remoteVer = execSync(`npm view ${name}@${version} version --registry=https://registry.npmjs.org/`, {
        stdio: ['ignore', 'pipe', 'ignore'],
        encoding: 'utf8'
      }).trim();

      if (remoteVer === version) {
        console.log(`⏩ ${name}@${version} is already published on npm. Skipping.`);
        continue;
      }
    } catch {
      // 404 or package doesn't exist yet, proceed with publish
    }
  }

  console.log(`🚀 Publishing ${name}@${version}${isDryRun ? ' (dry-run)' : ''}...`);
  const dryFlag = isDryRun ? ' --dry-run' : '';
  execSync(`npm publish --access public --registry=https://registry.npmjs.org/${dryFlag}`, {
    cwd,
    stdio: 'inherit'
  });

  if (i < packages.length - 1 && !isDryRun) {
    console.log('⏳ Waiting 3s for npm registry synchronization...');
    await new Promise((r) => setTimeout(r, 3000));
  }
}

console.log(`\n✨ Done processing all packages!\n`);
