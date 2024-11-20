use std::cell::RefMut;
use std::convert::Infallible;
use std::io::Write;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use borsh::{BorshDeserialize, BorshSerialize};
use borsh_size::{BorshSize, BorshSizeProperties};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use solana_program::account_info::AccountInfo;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_utils::VariantName;

use crate::error::ResolverError;

#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    FromPrimitive,
    BorshDeserialize,
    BorshSerialize,
    BorshSize,
    VariantName,
)]
#[repr(u8)]
pub enum AccountType {
    /// Uninitialized account, which has all bytes set to zero by default.
    #[default]
    Uninitialized,
    /// Account a market resolver.
    ResolverV1,
}

pub(crate) trait Account: BorshDeserialize + BorshSerialize {
    const TYPE: AccountType;

    #[inline]
    fn name() -> &'static str {
        Self::TYPE.variant_name()
    }

    fn check_account_owner(owner: &Pubkey) -> Result<(), ProgramError> {
        if !solana_utils::pubkeys_eq(owner, &crate::ID) {
            log!("Error: {} account is owned by the wrong program", Self::name());
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }

    fn safe_deserialize(data: &[u8]) -> Result<Self, ProgramError> {
        let account_type = get_account_type(data, Self::name())?;
        if account_type != Self::TYPE {
            log!(
                "Error: Incorrect account type: {}, expected {}",
                account_type.variant_name(),
                Self::name(),
            );
            return Err(ResolverError::DeserializationError.into());
        }

        deserialize_account::<Self>(data)
    }

    #[track_caller]
    fn from_account_info(info: &AccountInfo) -> Result<Self, ProgramError> {
        let data = info.try_borrow_data()?;
        let account = Self::safe_deserialize(*data)?;

        Self::check_account_owner(info.owner)?;

        Ok(account)
    }
}

pub(crate) trait AccountSized: Account + BorshSize {
    #[track_caller]
    fn from_account_info_mut<'a, 'info>(
        info: &'a AccountInfo<'info>,
    ) -> Result<AccountSizedMut<'a, 'info, Self>, ProgramError> {
        let data = info.try_borrow_mut_data()?;

        let account = Self::safe_deserialize(&data)?;

        Self::check_account_owner(info.owner)?;

        Ok(AccountSizedMut { info: PhantomData, data, account })
    }
}

impl<T: Account + BorshSize> AccountSized for T {}

#[must_use = "Must call `.save()` to save account"]
pub(crate) struct AccountSizedMut<'a, 'info, T> {
    info: PhantomData<&'a AccountInfo<'info>>,
    data: RefMut<'a, &'info mut [u8]>,
    account: T,
}

impl<T: AccountSized> AccountSizedMut<'_, '_, T> {
    pub fn save(mut self) -> Result<T, ProgramError> {
        if !T::IS_FIXED_SIZE {
            let size = self.borsh_size();

            if size > self.data.len() {
                log!("Error: {} account overflows allocation", T::name());
                return Err(ResolverError::SerializationError.into());
            }
        }

        serialize_account(&mut *self.data, &self.account)?;

        Ok(self.account)
    }
}

impl<T> Deref for AccountSizedMut<'_, '_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl<T> DerefMut for AccountSizedMut<'_, '_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account
    }
}

pub(crate) trait InitAccount<Params, Error>: Account
where
    Params: TryInto<(Self, usize), Error = Error>,
{
    /// Returns the account and the space required for initialization.
    fn try_init(params: Params) -> Result<AccountInitializer<Self>, Error> {
        let (account, space) = params.try_into()?;
        Ok(AccountInitializer { account, space })
    }

    fn init(params: Params) -> AccountInitializer<Self>
    where
        Error: Into<Infallible>,
    {
        match Self::try_init(params).map_err(|err| err.into()) {
            Ok(account_init) => account_init,
            #[allow(unreachable_patterns)]
            Err(err) => match err {},
        }
    }
}

impl<T, Params, Error> InitAccount<Params, Error> for T
where
    T: Account,
    Params: TryInto<(Self, usize), Error = Error>,
{
}

#[must_use = "Must call `.save()` to initialize account"]
pub(crate) struct AccountInitializer<T> {
    account: T,
    space: usize,
}

impl<T: Account> AccountInitializer<T> {
    pub fn save(self, context: InitContext) -> Result<T, ProgramError> {
        let InitContext { account: account_info, payer, system_program, program_id, signers_seeds } =
            context;

        solana_utils::create_or_allocate_account(
            account_info,
            payer,
            system_program,
            self.space,
            program_id,
            signers_seeds,
        )?;

        serialize_account(account_info.try_borrow_mut_data()?.deref_mut(), &self.account)?;

        Ok(self.account)
    }
}

impl<T> Deref for AccountInitializer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.account
    }
}

impl<T> DerefMut for AccountInitializer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.account
    }
}

pub(crate) struct InitContext<'a, 'b, 'c, 'info> {
    pub account: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub program_id: &'a Pubkey,
    pub signers_seeds: &'a [&'b [&'c [u8]]],
}

pub(crate) fn get_account_type(data: &[u8], expected: &str) -> Result<AccountType, ResolverError> {
    let key = match data {
        [] | [0, ..] => {
            log!("Error: {expected} account is uninitialized");
            return Err(ResolverError::DeserializationError);
        }
        &[key, ..] => key,
    };

    AccountType::from_u8(key).ok_or_else(|| {
        log!("Error: Unknown account type: {key:#x}, expected {expected}");
        ResolverError::DeserializationError
    })
}

pub(crate) fn deserialize_account<T: Account>(data: &[u8]) -> Result<T, ProgramError> {
    T::deserialize(&mut &data[..]).map_err(|err| {
        log!("Error: {} account deserialization failed: {err}", T::name());
        ResolverError::DeserializationError.into()
    })
}

pub(crate) fn serialize_account<W: Write, T: Account>(
    writer: W,
    account: &T,
) -> Result<(), ProgramError> {
    borsh::to_writer(writer, account).map_err(|err| {
        log!("Error: {} serialization failed {err}", T::name());
        ResolverError::SerializationError.into()
    })
}
