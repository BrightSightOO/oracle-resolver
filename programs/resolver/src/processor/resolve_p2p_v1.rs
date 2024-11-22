use borsh::BorshDeserialize;
use optimistic_oracle::accounts::RequestV1;
use optimistic_oracle::types::RequestState;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_utils::log;

use crate::error::ResolverError;
use crate::instruction::accounts::ResolveP2pV1Accounts;
use crate::utils::YesNo;
use crate::{pda, utils};

pub fn resolve_p2p_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = ResolveP2pV1Accounts::context(accounts)?;

    // Guard programs.
    utils::assert_p2p_program(ctx.accounts.p2p_program.key)?;

    // Guard PDAs.
    let resolver_bump = pda::resolver::assert_pda(
        ctx.accounts.resolver.key,
        ctx.accounts.market.key,
        ctx.accounts.request.key,
    )?;

    let outcome: YesNo;

    // Step 1: Get outcome from request resolved value.
    {
        let data = ctx.accounts.request.try_borrow_data()?;
        let request = RequestV1::deserialize(&mut &data[..]).map_err(|err| {
            log!("Error: Request account deserialization failed: {err}");
            ResolverError::DeserializationError
        })?;

        if request.state != RequestState::Resolved {
            return Err(ResolverError::RequestNotResolved.into());
        }

        outcome = YesNo::from_value(request.value);
    }

    // Step 2: Resolve market.
    {
        let signer_seeds = pda::resolver::seeds_with_bump(
            ctx.accounts.market.key,
            ctx.accounts.request.key,
            &resolver_bump,
        );

        cpi::hpl::p2p::resolve_v1(
            outcome.into(),
            cpi::hpl::p2p::ResolveV1 {
                market: ctx.accounts.market,
                resolver: ctx.accounts.resolver,
                p2p_program: ctx.accounts.p2p_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
