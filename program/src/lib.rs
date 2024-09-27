#[cfg(all(target_os = "solana", feature = "bpf-entrypoint"))]
mod entrypoint;
pub mod instruction;
pub mod processor;
pub mod state;

pub use solana_program;

solana_program::declare_id!("7GXAVdVr4QkdZBJTUAkKNKnBD4tabJQAG7R79dj4kibk");
