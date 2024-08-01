use {
    bytemuck_derive::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
};

#[repr(C)]
#[derive(Copy, Clone, Default, Pod, Zeroable)]
pub struct GetStakeActivatingAndDeactivatingReturnData {
    pub withdrawer: Pubkey,
    pub delegated_vote: Pubkey,
    pub effective: u64,
    pub activating: u64,
    pub deactivating: u64,
}
