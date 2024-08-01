#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getToolchainArgument,
  partitionArguments,
  popFlag,
  workingDirectory,
} from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const cliArgs = cliArguments();
const clientPath = cliArgs[0];
const formatArgs = cliArgs.slice(1);

const fix = popFlag(formatArgs, '--fix');
const [cargoArgs, fmtArgs] = partitionArguments(formatArgs, '--');
const toolchain = getToolchainArgument('format');
const manifestPath = path.join(
  workingDirectory,
  'clients',
  clientPath,
  'Cargo.toml'
);

// Format the client.
if (fix) {
  await $`cargo ${toolchain} fmt --manifest-path ${manifestPath} ${cargoArgs} -- ${fmtArgs}`;
} else {
  await $`cargo ${toolchain} fmt --manifest-path ${manifestPath} ${cargoArgs} -- --check ${fmtArgs}`;
}
