use {
    num_derive::{FromPrimitive, ToPrimitive},
    shank::{ShankContext, ShankInstruction}
};

#[derive(Clone, Debug, ShankContext, ShankInstruction, FromPrimitive, ToPrimitive)]
#[repr(u8)]
pub enum SolStakeViewInstruction {
    /// Outputs the validator vote account address that this stake account is
    /// delegated to, followed by the effective, activating, and deactivating
    /// SOL stake amount.
    /// Must be a valid SOL stake account.
    #[account(0, name="stake", desc = "The target SOL stake account")]
    #[account(1, name="stake_history", desc = "The stake history sysvar")]
    GetStakeActivatingAndDeactivating,
}
