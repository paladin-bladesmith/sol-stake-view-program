#!/usr/bin/env zx
import 'zx/globals';
import {
  cliArguments,
  getAllRustClientFolders,
  popArgument,
  workingDirectory,
} from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const cliArgs = cliArguments();
const clientPath = popArgument(cliArgs, '--client-path');

const hasSolfmt = await which('solfmt', { nothrow: true });

// Run the tests.
const clientFolders = clientPath ? [clientPath] : getAllRustClientFolders();
await Promise.all(
  clientFolders.map(async (folder) => {
    cd(path.join(workingDirectory, folder));
    const sbfOutDir = path.join(workingDirectory, 'target', 'deploy');
    if (hasSolfmt) {
      await $`SBF_OUT_DIR=${sbfOutDir} cargo test ${cliArgs} 2>&1 | solfmt`;
    } else {
      await $`SBF_OUT_DIR=${sbfOutDir} cargo test ${cliArgs}`;
    }
  })
);
