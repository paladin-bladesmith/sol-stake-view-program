/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import { type ParsedGetStakeActivatingAndDeactivatingInstruction } from '../instructions';

export const SOL_STAKE_VIEW_PROGRAM_PROGRAM_ADDRESS =
  'stkVUdWUiarMmkttUKMGLCLwHUkBqYfQ9vZcfG3T7LU' as Address<'stkVUdWUiarMmkttUKMGLCLwHUkBqYfQ9vZcfG3T7LU'>;

export enum SolStakeViewProgramInstruction {
  GetStakeActivatingAndDeactivating,
}

export function identifySolStakeViewProgramInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): SolStakeViewProgramInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return SolStakeViewProgramInstruction.GetStakeActivatingAndDeactivating;
  }
  throw new Error(
    'The provided instruction could not be identified as a solStakeViewProgram instruction.'
  );
}

export type ParsedSolStakeViewProgramInstruction<
  TProgram extends string = 'stkVUdWUiarMmkttUKMGLCLwHUkBqYfQ9vZcfG3T7LU',
> = {
  instructionType: SolStakeViewProgramInstruction.GetStakeActivatingAndDeactivating;
} & ParsedGetStakeActivatingAndDeactivatingInstruction<TProgram>;
