use borsh::BorshDeserialize;
use optimistic_oracle::accounts::RequestV1;
use optimistic_oracle::types::{RequestKind, RequestState};
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_utils::log;

use crate::error::ResolverError;
use crate::instruction::accounts::ResolveParimutuelLuloV1Accounts;
use crate::utils::OptionsOutcome;
use crate::{pda, utils};

pub fn resolve_parimutuel_v1<'a>(
    _program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
) -> ProgramResult {
    let ctx = ResolveParimutuelLuloV1Accounts::context(accounts)?;

    // Guard programs.
    utils::assert_parimutuel_lulo_program(ctx.accounts.parimutuel_lulo_program.key)?;

    // Guard signatures.
    utils::assert_signer(ctx.accounts.payer)?;

    // Guard PDAs.
    let resolver_bump = pda::resolver::assert_pda(
        ctx.accounts.resolver.key,
        ctx.accounts.market.key,
        ctx.accounts.request.key,
    )?;

    let outcome: OptionsOutcome;

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

        let options = match request.kind {
            RequestKind::Options { options } => options,
            // This should never happen, since we check on creation.
            _ => return Err(ResolverError::InvalidRequestKind.into()),
        };

        outcome = OptionsOutcome::from_value(request.value, options);
    }

    // Step 2: Resolve market.
    {
        let signer_seeds = pda::resolver::seeds_with_bump(
            ctx.accounts.market.key,
            ctx.accounts.request.key,
            &resolver_bump,
        );

        cpi::hpl::parimutuel_lulo::resolve_v1(
            outcome.into(),
            cpi::hpl::parimutuel_lulo::ResolveV1 {
                market: ctx.accounts.market,
                resolver: ctx.accounts.resolver,
                mint: ctx.accounts.mint,
                deposit: ctx.accounts.deposit,
                lulo_pool: ctx.accounts.lulo_pool,
                lulo_user: ctx.accounts.lulo_user,
                lulo_deposit: ctx.accounts.lulo_deposit,
                lulo_promotion_reserve: ctx.accounts.lulo_promotion_reserve,
                payer: ctx.accounts.payer,
                lulo_program: ctx.accounts.lulo_program,
                ata_program: ctx.accounts.ata_program,
                token_program: ctx.accounts.token_program,
                system_program: ctx.accounts.system_program,
                parimutuel_program: ctx.accounts.parimutuel_lulo_program,
            },
            &[&signer_seeds],
        )?;
    }

    Ok(())
}
