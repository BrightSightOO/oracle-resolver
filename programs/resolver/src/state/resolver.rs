use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use shank::ShankAccount;
use solana_program::pubkey::Pubkey;

use super::{Account, AccountType};

#[derive(Clone, BorshDeserialize, BorshSerialize, BorshSize, ShankAccount)]
pub struct ResolverV1 {
    account_type: AccountType,

    /// The address of the market that will be resolved.
    pub market: Pubkey,
    /// The address of the oracle request used to source outcome.
    pub request: Pubkey,
    /// The address of the oracle request used to source outcome.
    pub program: MarketProgram,
}

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize, BorshSize)]
#[borsh(use_discriminant = true)]
#[repr(u8)]
pub enum MarketProgram {
    P2p,
    LegacyAmm,
    Parimutuel,
    ParimutuelLulo,
}

impl Account for ResolverV1 {
    const TYPE: AccountType = AccountType::ResolverV1;
}

impl From<InitResolver> for (ResolverV1, usize) {
    fn from(params: InitResolver) -> (ResolverV1, usize) {
        let InitResolver { market, request, program } = params;

        (
            ResolverV1 { account_type: ResolverV1::TYPE, market, request, program },
            ResolverV1::FIXED_SIZE,
        )
    }
}

pub(crate) struct InitResolver {
    pub market: Pubkey,
    pub request: Pubkey,
    pub program: MarketProgram,
}
