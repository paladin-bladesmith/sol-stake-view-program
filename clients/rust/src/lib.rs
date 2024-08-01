mod generated;

pub use generated::{programs::SOL_STAKE_VIEW_PROGRAM_ID as ID, *};
use {
    bytemuck_derive::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
    spl_pod::option::PodOption,
};

/// Helper struct to easily handle the return data created by the
/// `GetStakeActivatingAndDeactivating` instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GetStakeActivatingAndDeactivatingReturnData {
    pub withdrawer: PodOption<Pubkey>,
    pub delegated_vote: PodOption<Pubkey>,
    pub effective: u64,
    pub activating: u64,
    pub deactivating: u64,
}

impl Default for GetStakeActivatingAndDeactivatingReturnData {
    fn default() -> Self {
        Self {
            withdrawer: None.try_into().unwrap(),
            delegated_vote: None.try_into().unwrap(),
            effective: 0,
            activating: 0,
            deactivating: 0,
        }
    }
}
