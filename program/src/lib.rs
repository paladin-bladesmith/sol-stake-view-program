#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

pub use solana_program;

solana_program::declare_id!("stkVUdWUiarMmkttUKMGLCLwHUkBqYfQ9vZcfG3T7LU");
