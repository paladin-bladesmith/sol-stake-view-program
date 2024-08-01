#!/usr/bin/env zx
import 'zx/globals';
import { cliArguments, workingDirectory } from '../utils.mjs';

// Configure additional arguments here, e.g.:
// ['--arg1', '--arg2', ...cliArguments()]
const cliArgs = cliArguments();
const clientPath = cliArgs[0];
const testArgs = cliArgs.slice(1);

const hasSolfmt = await which('solfmt', { nothrow: true });

// Run the tests.
cd(path.join(workingDirectory, 'clients', clientPath));
const sbfOutDir = path.join(workingDirectory, 'target', 'deploy');
if (hasSolfmt) {
  await $`SBF_OUT_DIR=${sbfOutDir} cargo test ${testArgs} 2>&1 | solfmt`;
} else {
  await $`SBF_OUT_DIR=${sbfOutDir} cargo test ${testArgs}`;
}
