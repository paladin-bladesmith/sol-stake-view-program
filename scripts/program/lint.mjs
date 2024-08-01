#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getProgramFolders,
  getToolchainArgument,
  popFlag,
  workingDirectory,
} from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const lintArgs = [
  '-Zunstable-options',
  '--features',
  'bpf-entrypoint',
  '--',
  '--deny=warnings',
  '--deny=clippy::arithmetic_side_effects',
  ...cliArguments()
];

const fix = popFlag(lintArgs, '--fix');
const toolchain = getToolchainArgument('format');

// Lint the programs using clippy.
await Promise.all(
  getProgramFolders().map(async (folder) => {
    const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml');

    if (fix) {
      await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} --fix ${lintArgs}`;
    } else {
      await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} ${lintArgs}`;
    }
  })
);
