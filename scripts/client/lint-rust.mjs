#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getAllRustClientFolders,
  getToolchainArgument,
  popArgument,
  popFlag,
  workingDirectory,
} from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const lintArgs = [
  '-Zunstable-options',
  '--',
  '--deny=warnings',
  ...cliArguments(),
];

const fix = popFlag(lintArgs, '--fix');
const clientPath = popArgument(lintArgs, '--client-path');
const toolchain = getToolchainArgument('format');

const clientFolders = clientPath ? [clientPath] : getAllRustClientFolders();
await Promise.all(
  clientFolders.map(async (folder) => {
    const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml');

    // Check the client using Clippy.
    if (fix) {
      await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} --fix ${lintArgs}`;
    } else {
      await $`cargo ${toolchain} clippy --manifest-path ${manifestPath} ${lintArgs}`;
    }
  })
);
