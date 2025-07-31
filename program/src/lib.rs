#![allow(unexpected_cfgs)]
#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

pub use solana_program;

solana_program::declare_id!("5DVCQTwNrDBg5yBNRVWSQ7p6Jk1euxXJb6aDYYdjw4Vw");
