## Quick start

1. Install Rust from https://rustup.rs/
2. `cargo run`

## About

`paladin-sol-stake-view-cli` is a small app demonstrating how to work with the SOL stake
view program.

It provides only one command:

- `get`: Simulates a transaction containing the `GetStakeActivatingAndDeactivating` for
given stake account address

## Local step-by-step

1. Prepare account:
  - Start a local node: run `solana-test-validator`.
  - Generate a keypair: `solana-keygen new -o test.json`.
  - Add 100 SOL to the corresponding account `solana airdrop --url http://127.0.0.1:8899 --keypair test.json 100`.
  - Create a stake account keypair: `solana-keygen new -o stake.json`
  - Create a stake account: `solana create-stake-account stake.json 2`

2. Build app: `cargo run`.

3. Get:
```
  $ cargo run -- get <STAKE_ADDRESS> --url http://127.0.0.1:8899
```
