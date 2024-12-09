//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ResolveLegacyAmmV1 {
    /// Resolver
    pub resolver: solana_program::pubkey::Pubkey,
    /// Market
    pub market: solana_program::pubkey::Pubkey,
    /// Oracle request
    pub request: solana_program::pubkey::Pubkey,
    /// Legacy outcome tokens program
    pub outcome_tokens_program: solana_program::pubkey::Pubkey,
}

impl ResolveLegacyAmmV1 {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.resolver, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.market, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.request, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.outcome_tokens_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ResolveLegacyAmmV1InstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::ORACLE_RESOLVER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ResolveLegacyAmmV1InstructionData {
    discriminator: u8,
}

impl ResolveLegacyAmmV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 2 }
    }
}

impl Default for ResolveLegacyAmmV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ResolveLegacyAmmV1`.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[]` outcome_tokens_program
#[derive(Clone, Debug, Default)]
pub struct ResolveLegacyAmmV1Builder {
    resolver: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    outcome_tokens_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ResolveLegacyAmmV1Builder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Resolver
    #[inline(always)]
    pub fn resolver(&mut self, resolver: solana_program::pubkey::Pubkey) -> &mut Self {
        self.resolver = Some(resolver);
        self
    }
    /// Market
    #[inline(always)]
    pub fn market(&mut self, market: solana_program::pubkey::Pubkey) -> &mut Self {
        self.market = Some(market);
        self
    }
    /// Oracle request
    #[inline(always)]
    pub fn request(&mut self, request: solana_program::pubkey::Pubkey) -> &mut Self {
        self.request = Some(request);
        self
    }
    /// Legacy outcome tokens program
    #[inline(always)]
    pub fn outcome_tokens_program(
        &mut self,
        outcome_tokens_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.outcome_tokens_program = Some(outcome_tokens_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ResolveLegacyAmmV1 {
            resolver: self.resolver.expect("resolver is not set"),
            market: self.market.expect("market is not set"),
            request: self.request.expect("request is not set"),
            outcome_tokens_program: self
                .outcome_tokens_program
                .expect("outcome_tokens_program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `resolve_legacy_amm_v1` CPI accounts.
pub struct ResolveLegacyAmmV1CpiAccounts<'a, 'b> {
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Legacy outcome tokens program
    pub outcome_tokens_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `resolve_legacy_amm_v1` CPI instruction.
pub struct ResolveLegacyAmmV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Legacy outcome tokens program
    pub outcome_tokens_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ResolveLegacyAmmV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ResolveLegacyAmmV1CpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            resolver: accounts.resolver,
            market: accounts.market,
            request: accounts.request,
            outcome_tokens_program: accounts.outcome_tokens_program,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(4 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.resolver.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.market.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.request.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.outcome_tokens_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ResolveLegacyAmmV1InstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ORACLE_RESOLVER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.resolver.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.outcome_tokens_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ResolveLegacyAmmV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[]` outcome_tokens_program
#[derive(Clone, Debug)]
pub struct ResolveLegacyAmmV1CpiBuilder<'a, 'b> {
    instruction: Box<ResolveLegacyAmmV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ResolveLegacyAmmV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ResolveLegacyAmmV1CpiBuilderInstruction {
            __program: program,
            resolver: None,
            market: None,
            request: None,
            outcome_tokens_program: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Resolver
    #[inline(always)]
    pub fn resolver(
        &mut self,
        resolver: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.resolver = Some(resolver);
        self
    }
    /// Market
    #[inline(always)]
    pub fn market(
        &mut self,
        market: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.market = Some(market);
        self
    }
    /// Oracle request
    #[inline(always)]
    pub fn request(
        &mut self,
        request: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.request = Some(request);
        self
    }
    /// Legacy outcome tokens program
    #[inline(always)]
    pub fn outcome_tokens_program(
        &mut self,
        outcome_tokens_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.outcome_tokens_program = Some(outcome_tokens_program);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction.__remaining_accounts.push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)],
    ) -> &mut Self {
        self.instruction.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = ResolveLegacyAmmV1Cpi {
            __program: self.instruction.__program,

            resolver: self.instruction.resolver.expect("resolver is not set"),

            market: self.instruction.market.expect("market is not set"),

            request: self.instruction.request.expect("request is not set"),

            outcome_tokens_program: self
                .instruction
                .outcome_tokens_program
                .expect("outcome_tokens_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ResolveLegacyAmmV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    resolver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    outcome_tokens_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}