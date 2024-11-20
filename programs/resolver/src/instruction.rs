use borsh::BorshDeserialize;
use shank::{ShankContext, ShankInstruction};
use solana_utils::VariantName;

use crate::processor::*;

/// Instructions supported by the P2P program.
#[rustfmt::skip::attributes(account)]
#[repr(u8)]
#[derive(Clone, BorshDeserialize, ShankContext, ShankInstruction, VariantName)]
#[borsh(use_discriminant = true)]
pub(crate) enum ResolverInstruction {
    /// Creates a user.
    #[account(0, writable, name = "resolver", desc = "Resolver")]
    #[account(1, name = "market", desc = "Market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, signer, writable, name = "payer", desc = "Payer")]
    #[account(4, name = "system_program", desc = "System program")]
    CreateV1,
}
