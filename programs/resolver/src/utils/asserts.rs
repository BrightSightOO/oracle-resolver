use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::system_program;
use solana_utils::log;

macro_rules! programs {
    ($(
        $desc:literal: $fn_name:ident($id:expr $(, $extra_id:expr)* $(,)?)
    );* $(;)?) => {
        $(
            pub fn $fn_name(pubkey: &Pubkey) -> Result<(), ProgramError> {
                if !solana_utils::pubkeys_eq(pubkey, &$id) $(&& !solana_utils::pubkeys_eq(pubkey, &$extra_id))* {
                    solana_utils::log!(concat!("Error: Incorrect address for ", $desc, " program"));
                    return Err(ProgramError::IncorrectProgramId);
                }
                Ok(())
            }
        )*
    };
}

programs! {
    "P2P": assert_p2p_program(cpi::hpl::p2p::ID);

    "outcome tokens": assert_outcome_tokens_program(cpi::legacy::amm::OUTCOME_TOKENS_ID);

    "token": assert_token_program(cpi::spl::TOKEN_ID, cpi::spl::TOKEN_2022_ID);
    "system": assert_system_program(system_program::ID);
}

pub fn assert_signer(account_info: &AccountInfo) -> Result<(), ProgramError> {
    if !account_info.is_signer {
        log!("Error: Account {} is expected to be a signer", account_info.key);
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}
