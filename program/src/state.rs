use {
    bytemuck_derive::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GetStakeActivatingAndDeactivatingReturnData {
    pub delegated_vote: Pubkey,
    pub effective: u64,
    pub activating: u64,
    pub deactivating: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Wrapper {
    pub data: GetStakeActivatingAndDeactivatingReturnData
}