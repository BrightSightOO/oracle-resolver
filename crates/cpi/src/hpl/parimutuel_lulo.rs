use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey;
use solana_program::pubkey::Pubkey;
use solana_utils::invoke::invoke_signed;

pub const ID: Pubkey = pubkey!("PARrVs6F5egaNuz8g6pKJyU4ze3eX5xGZCFb3GLiVvu");

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
#[borsh(use_discriminant = true)]
pub enum ResolveV1Args {
    Outcome { outcome: u8 },
    Invalid,
}

pub struct ResolveV1<'a, 'info> {
    pub market: &'a AccountInfo<'info>,
    pub resolver: &'a AccountInfo<'info>,
    pub mint: &'a AccountInfo<'info>,
    pub deposit: &'a AccountInfo<'info>,
    pub lulo_pool: &'a AccountInfo<'info>,
    pub lulo_user: &'a AccountInfo<'info>,
    pub lulo_deposit: &'a AccountInfo<'info>,
    pub lulo_promotion_reserve: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub lulo_program: &'a AccountInfo<'info>,
    pub ata_program: &'a AccountInfo<'info>,
    pub token_program: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub parimutuel_program: &'a AccountInfo<'info>,
}

pub fn resolve_v1(
    args: ResolveV1Args,
    accounts: ResolveV1,
    signers_seeds: &[&[&[u8]]],
) -> ProgramResult {
    const DISCRIMINATOR: u8 = 5;

    let ResolveV1 {
        market,
        resolver,
        mint,
        deposit,
        lulo_pool,
        lulo_user,
        lulo_deposit,
        lulo_promotion_reserve,
        payer,
        lulo_program,
        ata_program,
        token_program,
        system_program,
        parimutuel_program,
    } = accounts;

    let data = match args {
        ResolveV1Args::Outcome { outcome } => vec![DISCRIMINATOR, 0, outcome],
        ResolveV1Args::Invalid => vec![DISCRIMINATOR, 1],
    };

    let instruction = Instruction {
        program_id: *parimutuel_program.key,
        accounts: vec![
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*resolver.key, true),
            AccountMeta::new_readonly(*mint.key, false),
            AccountMeta::new(*deposit.key, false),
            AccountMeta::new(*lulo_pool.key, false),
            AccountMeta::new(*lulo_user.key, false),
            AccountMeta::new(*lulo_deposit.key, false),
            AccountMeta::new(*lulo_promotion_reserve.key, false),
            AccountMeta::new(*payer.key, true),
            AccountMeta::new_readonly(*lulo_program.key, false),
            AccountMeta::new_readonly(*ata_program.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
            AccountMeta::new_readonly(*system_program.key, false),
        ],
        data,
    };

    invoke_signed(
        &instruction,
        &[
            market.clone(),
            resolver.clone(),
            mint.clone(),
            deposit.clone(),
            lulo_pool.clone(),
            lulo_user.clone(),
            lulo_deposit.clone(),
            lulo_promotion_reserve.clone(),
            payer.clone(),
            lulo_program.clone(),
            ata_program.clone(),
            token_program.clone(),
            system_program.clone(),
        ],
        signers_seeds,
    )?;

    Ok(())
}
