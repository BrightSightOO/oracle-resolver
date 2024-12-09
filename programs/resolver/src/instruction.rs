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
    CreateV1(CreateV1Args),

    /// Resolves a P2P market with the value from the request.
    #[account(0, name = "resolver", desc = "Resolver")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, name = "p2p_program", desc = "HPL P2P program")]
    ResolveP2pV1,

    /// Resolves a legacy AMM market with the value from the request.
    #[account(0, name = "resolver", desc = "Resolver")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, name = "outcome_tokens_program", desc = "Legacy outcome tokens program")]
    ResolveLegacyAmmV1,

    /// Resolves a parimutuel market with the value from the request.
    #[account(0, name = "resolver", desc = "Resolver")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, name = "mint", desc = "Deposit token mint")]
    #[account(4, writable, name = "deposit", desc = "Deposit token account")]
    #[account(5, signer, writable, name = "payer", desc = "Payer")]
    #[account(6, name = "token_program", desc = "SPL token program")]
    #[account(7, name = "system_program", desc = "System program")]
    #[account(8, name = "parimutuel_program", desc = "HPL parimutuel program")]
    ResolveParimutuelV1,

    /// Resolves a parimutuel (LULO) market with the value from the request.
    #[account(0, name = "resolver", desc = "Resolver")]
    #[account(1, writable, name = "market", desc = "Market")]
    #[account(2, name = "request", desc = "Oracle request")]
    #[account(3, name = "mint", desc = "Deposit token mint")]
    #[account(4, writable, name = "deposit", desc = "Deposit token account")]
    #[account(5, writable, name = "lulo_pool", desc = "LULO pool data")]
    #[account(6, writable, name = "lulo_user", desc = "LULO user account")]
    #[account(7, writable, name = "lulo_deposit", desc = "LULO deposit token account")]
    #[account(8, writable, name = "lulo_promotion_reserve", desc = "LULO promotion reserve")]
    #[account(9, signer, writable, name = "payer", desc = "Payer")]
    #[account(10, name = "lulo_program", desc = "LULO program")]
    #[account(11, name = "ata_program", desc = "SPL associated token program")]
    #[account(12, name = "token_program", desc = "SPL token program")]
    #[account(13, name = "system_program", desc = "System program")]
    #[account(14, name = "parimutuel_lulo_program", desc = "HPL parimutuel (LULO) program")]
    ResolveParimutuelLuloV1,
}
