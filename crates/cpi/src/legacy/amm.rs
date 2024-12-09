use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use solana_utils::invoke::invoke_signed;

pub const AMM_ID: Pubkey = pubkey!("Hr4whNgXr3yZsJvx3TVSwfsFgXuSEPB1xKmvgrtLhsrM");
pub const OUTCOME_TOKENS_ID: Pubkey = pubkey!("D8vMVKonxkbBtAXAxBwPPWyTfon8337ARJmHvwtsF98G");

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
pub enum Outcome {
    Yes = 1,
    No,
    Invalid,
}

pub struct UpdateStatus<'a, 'info> {
    pub market: &'a AccountInfo<'info>,
    pub resolver: &'a AccountInfo<'info>,
    pub outcome_tokens_program: &'a AccountInfo<'info>,
}

pub fn update_status(
    outcome: Outcome,
    accounts: UpdateStatus,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    const DISCRIMINATOR: [u8; 8] = [147, 215, 74, 174, 55, 191, 42, 0];

    let UpdateStatus { market, resolver, outcome_tokens_program } = accounts;

    let mut data = Vec::with_capacity(9);
    data.extend_from_slice(&DISCRIMINATOR);
    data.push(outcome as u8);

    let instruction = Instruction {
        program_id: *outcome_tokens_program.key,
        accounts: vec![
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*resolver.key, true),
        ],
        data,
    };

    invoke_signed(&instruction, &[market.clone(), resolver.clone()], signers_seeds)?;

    Ok(())
}
