import { readFileSync, writeFileSync } from 'node:fs';
import { resolve } from 'node:path';

const rootDir = process.cwd();
const rootPkgPath = resolve(rootDir, 'package.json');
const rootPkg = JSON.parse(readFileSync(rootPkgPath, 'utf8'));

const targetVersion = process.argv[2] || rootPkg.version;

if (!targetVersion) {
  console.error('Please specify a version, e.g.: node scripts/bump-version.mjs 1.0.3');
  process.exit(1);
}

console.log(`\n🚀 Synchronizing entire monorepo version to: ${targetVersion}\n`);

// 1. Update root package.json
rootPkg.version = targetVersion;
writeFileSync(rootPkgPath, JSON.stringify(rootPkg, null, 2) + '\n', 'utf8');
console.log(`✅ Updated package.json -> ${targetVersion}`);

// 2. Update sub package.jsons
const subPackages = [
  'packages/electron',
  'packages/tauri',
  'packages/shared',
  'packages/overlay',
  'demos/electron',
  'demos/tauri'
];

for (const pkgRel of subPackages) {
  const pkgPath = resolve(rootDir, pkgRel, 'package.json');
  try {
    const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
    pkg.version = targetVersion;
    writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n', 'utf8');
    console.log(`✅ Updated ${pkgRel}/package.json -> ${targetVersion}`);
  } catch (e) {
    console.warn(`⚠️ Skipped ${pkgRel}: ${e.message}`);
  }
}

// 3. Update Cargo.toml files
const cargoFiles = [
  'crates/tauri-plugin-snapora/Cargo.toml',
  'demos/tauri/src-tauri/Cargo.toml'
];

for (const cargoRel of cargoFiles) {
  const cargoPath = resolve(rootDir, cargoRel);
  try {
    let content = readFileSync(cargoPath, 'utf8');
    content = content.replace(/version\s*=\s*"[^"]+"/, `version = "${targetVersion}"`);
    writeFileSync(cargoPath, content, 'utf8');
    console.log(`✅ Updated ${cargoRel} -> ${targetVersion}`);
  } catch (e) {
    console.warn(`⚠️ Skipped ${cargoRel}: ${e.message}`);
  }
}

console.log(`\n🎉 Monorepo version successfully synchronized to ${targetVersion}!\n`);
