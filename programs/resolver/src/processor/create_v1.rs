use borsh::BorshDeserialize;
use optimistic_oracle::accounts::RequestV1;
use optimistic_oracle::types::RequestKind;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_utils::log;

use crate::error::ResolverError;
use crate::instruction::accounts::CreateV1Accounts;
use crate::state::{InitAccount, InitContext, InitResolver, ResolverV1};
use crate::{pda, utils};

pub fn create_v1<'a>(program_id: &'a Pubkey, accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    let ctx = CreateV1Accounts::context(accounts)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard programs.
    utils::assert_system_program(ctx.accounts.system_program.key)?;

    // Step 1: Validate request account.
    {
        if !solana_utils::pubkeys_eq(ctx.accounts.request.owner, &optimistic_oracle::ID) {
            log!("Error: Request account is owned by the wrong program");
            return Err(ProgramError::IncorrectProgramId);
        }

        let data = ctx.accounts.request.try_borrow_data()?;
        let request = RequestV1::deserialize(&mut &data[..]).map_err(|err| {
            log!("Error: Request account deserialization failed: {err}");
            ResolverError::DeserializationError
        })?;

        if request.kind == RequestKind::YesNo {
            return Err(ResolverError::InvalidRequestKind.into());
        }
    }

    // Step 2: Initialize resolver account.
    {
        let bump = pda::resolver::assert_pda(
            ctx.accounts.resolver.key,
            ctx.accounts.market.key,
            ctx.accounts.request.key,
        )?;
        let signer_seeds = pda::resolver::seeds_with_bump(
            ctx.accounts.market.key,
            ctx.accounts.request.key,
            &bump,
        );

        ResolverV1::init(InitResolver {
            market: *ctx.accounts.market.key,
            request: *ctx.accounts.request.key,
        })
        .save(InitContext {
            account: ctx.accounts.resolver,
            payer: ctx.accounts.payer,
            system_program: ctx.accounts.system_program,
            program_id,
            signers_seeds: &[&signer_seeds],
        })?;
    }

    Ok(())
}
