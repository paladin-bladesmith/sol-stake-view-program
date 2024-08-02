#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getAllRustClientFolders,
  getToolchainArgument,
  partitionArguments,
  popArgument,
  popFlag,
  workingDirectory,
} from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const cliArgs = cliArguments();

const fix = popFlag(cliArgs, '--fix');
const clientPath = popArgument(cliArgs, '--client-path');
console.log(clientPath);
const [cargoArgs, fmtArgs] = partitionArguments(cliArgs, '--');
const toolchain = getToolchainArgument('format');

const clientFolders = clientPath ? [clientPath] : getAllRustClientFolders();
await Promise.all(
  clientFolders.map(async (folder) => {
    const manifestPath = path.join(workingDirectory, folder, 'Cargo.toml');

    // Format the client.
    if (fix) {
      await $`cargo ${toolchain} fmt --manifest-path ${manifestPath} ${cargoArgs} -- ${fmtArgs}`;
    } else {
      await $`cargo ${toolchain} fmt --manifest-path ${manifestPath} ${cargoArgs} -- --check ${fmtArgs}`;
    }
  })
);
