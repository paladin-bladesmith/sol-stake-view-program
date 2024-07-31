mod generated;

use {
    bytemuck_derive::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
};

pub use generated::programs::SOL_STAKE_VIEW_PROGRAM_ID as ID;
pub use generated::*;

/// Helper struct to easily handle the return data created by the
/// `GetStakeActivatingAndDeactivating` instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct GetStakeActivatingAndDeactivatingReturnData {
    pub delegated_vote: Pubkey,
    pub effective: u64,
    pub activating: u64,
    pub deactivating: u64,
}
