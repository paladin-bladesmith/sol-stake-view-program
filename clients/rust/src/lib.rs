mod generated;

pub use generated::{programs::SOL_STAKE_VIEW_PROGRAM_ID as ID, *};
use {
    bytemuck_derive::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
    spl_pod::{option::PodOption, primitives::PodU64},
};

/// Helper struct to easily handle the return data created by the
/// `GetStakeActivatingAndDeactivating` instruction.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
pub struct GetStakeActivatingAndDeactivatingReturnData {
    pub staker: PodOption<Pubkey>,
    pub withdrawer: PodOption<Pubkey>,
    pub delegated_vote: PodOption<Pubkey>,
    pub effective: PodU64,
    pub activating: PodU64,
    pub deactivating: PodU64,
}

impl Default for GetStakeActivatingAndDeactivatingReturnData {
    fn default() -> Self {
        Self {
            staker: None.try_into().unwrap(),
            withdrawer: None.try_into().unwrap(),
            delegated_vote: None.try_into().unwrap(),
            effective: 0.into(),
            activating: 0.into(),
            deactivating: 0.into(),
        }
    }
}
