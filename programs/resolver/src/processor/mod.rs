use borsh::BorshDeserialize;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;
use solana_utils::{log, VariantName};

mod create_v1;
mod resolve_legacy_amm_v1;
mod resolve_p2p_v1;
mod resolve_parimutuel_lulo_v1;
mod resolve_parimutuel_v1;

pub(crate) use self::create_v1::*;
pub(crate) use self::resolve_legacy_amm_v1::*;
pub(crate) use self::resolve_p2p_v1::*;
pub(crate) use self::resolve_parimutuel_lulo_v1::*;
pub(crate) use self::resolve_parimutuel_v1::*;

pub fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &'a [u8],
) -> ProgramResult {
    use crate::instruction::ResolverInstruction as I;

    let instruction = I::try_from_slice(instruction_data)?;

    log!("Instruction: {}", instruction.variant_name());

    match instruction {
        I::CreateV1(args) => create_v1(program_id, accounts, args),
        I::ResolveP2pV1 => resolve_p2p_v1(program_id, accounts),
        I::ResolveLegacyAmmV1 => resolve_legacy_amm_v1(program_id, accounts),
        I::ResolveParimutuelV1 => resolve_parimutuel_v1(program_id, accounts),
        I::ResolveParimutuelLuloV1 => resolve_parimutuel_lulo_v1(program_id, accounts),
    }
}
