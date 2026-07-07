#!/usr/bin/env node

const { spawnSync } = require('node:child_process');
const { dirname, join } = require('node:path');

if (process.platform !== 'darwin') {
  console.error('imageoptim-cli is macOS only');
  process.exit(1);
}

const args = process.argv.slice(2);
const optionalDep = `imageoptim-cli-darwin-${process.arch}`;
const binaryName = 'imageoptim';

const pathToBinary = resolveBinaryPath();

const result = spawnSync(pathToBinary, args, {
  cwd: process.cwd(),
  stdio: 'inherit',
  env: process.env,
});

if (result.error) {
  console.error(result.error.message);
  process.exit(1);
}

// status is null when the child was killed by a signal: that is not success
process.exit(result.status === null ? 1 : result.status);

function resolveBinaryPath() {
  // Strategy 1: Resolve via package.json for pnpm Plug'n'Play
  try {
    const packageJsonPath = require.resolve(`${optionalDep}/package.json`);
    const packageDir = dirname(packageJsonPath);
    return join(packageDir, 'bin', binaryName);
  } catch (_) {}

  // Strategy 2: Original approach (works with traditional node_modules)
  try {
    return require.resolve(`${optionalDep}/bin/${binaryName}`);
  } catch (_) {}

  throw new Error(
    `Failed to resolve binary for darwin-${process.arch}. Please ensure ${optionalDep} is installed as an optional dependency.`,
  );
}
