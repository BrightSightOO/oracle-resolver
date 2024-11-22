use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use solana_utils::invoke::invoke_signed;

pub const ID: Pubkey = pubkey!("P2PototC41acvjMc9cvAoRjFjtaRD5Keo9PvNJfRwf3");

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
pub enum Outcome {
    Yes,
    No,
    Invalid,
}

pub struct ResolveV1<'a, 'info> {
    pub market: &'a AccountInfo<'info>,
    pub resolver: &'a AccountInfo<'info>,
    pub p2p_program: &'a AccountInfo<'info>,
}

pub fn resolve_v1(
    outcome: Outcome,
    accounts: ResolveV1,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    const DISCRIMINATOR: u8 = 5;

    let ResolveV1 { market, resolver, p2p_program } = accounts;

    let instruction = Instruction {
        program_id: *p2p_program.key,
        accounts: vec![
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*resolver.key, true),
        ],
        data: vec![DISCRIMINATOR, outcome as u8],
    };

    invoke_signed(&instruction, &[market.clone(), resolver.clone()], signers_seeds)?;

    Ok(())
}
