//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ResolveParimutuelLuloV1 {
    /// Resolver
    pub resolver: solana_program::pubkey::Pubkey,
    /// Market
    pub market: solana_program::pubkey::Pubkey,
    /// Oracle request
    pub request: solana_program::pubkey::Pubkey,
    /// Deposit token mint
    pub mint: solana_program::pubkey::Pubkey,
    /// Deposit token account
    pub deposit: solana_program::pubkey::Pubkey,
    /// LULO pool data
    pub lulo_pool: solana_program::pubkey::Pubkey,
    /// LULO user account
    pub lulo_user: solana_program::pubkey::Pubkey,
    /// LULO deposit token account
    pub lulo_deposit: solana_program::pubkey::Pubkey,
    /// LULO promotion reserve
    pub lulo_promotion_reserve: solana_program::pubkey::Pubkey,
    /// Payer
    pub payer: solana_program::pubkey::Pubkey,
    /// LULO program
    pub lulo_program: solana_program::pubkey::Pubkey,
    /// SPL associated token program
    pub ata_program: solana_program::pubkey::Pubkey,
    /// SPL token program
    pub token_program: solana_program::pubkey::Pubkey,
    /// System program
    pub system_program: solana_program::pubkey::Pubkey,
    /// HPL parimutuel (LULO) program
    pub parimutuel_lulo_program: solana_program::pubkey::Pubkey,
}

impl ResolveParimutuelLuloV1 {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.resolver, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.market, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.request, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(self.mint, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.deposit, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.lulo_pool, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.lulo_user, false));
        accounts.push(solana_program::instruction::AccountMeta::new(self.lulo_deposit, false));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lulo_promotion_reserve,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(self.payer, true));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.lulo_program, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(self.ata_program, false));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.parimutuel_lulo_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ResolveParimutuelLuloV1InstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::ORACLE_RESOLVER_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ResolveParimutuelLuloV1InstructionData {
    discriminator: u8,
}

impl ResolveParimutuelLuloV1InstructionData {
    pub fn new() -> Self {
        Self { discriminator: 4 }
    }
}

impl Default for ResolveParimutuelLuloV1InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ResolveParimutuelLuloV1`.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[]` mint
///   4. `[writable]` deposit
///   5. `[writable]` lulo_pool
///   6. `[writable]` lulo_user
///   7. `[writable]` lulo_deposit
///   8. `[writable]` lulo_promotion_reserve
///   9. `[writable, signer]` payer
///   10. `[]` lulo_program
///   11. `[optional]` ata_program (default to `ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL`)
///   12. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
///   13. `[optional]` system_program (default to `11111111111111111111111111111111`)
///   14. `[]` parimutuel_lulo_program
#[derive(Clone, Debug, Default)]
pub struct ResolveParimutuelLuloV1Builder {
    resolver: Option<solana_program::pubkey::Pubkey>,
    market: Option<solana_program::pubkey::Pubkey>,
    request: Option<solana_program::pubkey::Pubkey>,
    mint: Option<solana_program::pubkey::Pubkey>,
    deposit: Option<solana_program::pubkey::Pubkey>,
    lulo_pool: Option<solana_program::pubkey::Pubkey>,
    lulo_user: Option<solana_program::pubkey::Pubkey>,
    lulo_deposit: Option<solana_program::pubkey::Pubkey>,
    lulo_promotion_reserve: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    lulo_program: Option<solana_program::pubkey::Pubkey>,
    ata_program: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    parimutuel_lulo_program: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ResolveParimutuelLuloV1Builder {
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
    /// Deposit token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.mint = Some(mint);
        self
    }
    /// Deposit token account
    #[inline(always)]
    pub fn deposit(&mut self, deposit: solana_program::pubkey::Pubkey) -> &mut Self {
        self.deposit = Some(deposit);
        self
    }
    /// LULO pool data
    #[inline(always)]
    pub fn lulo_pool(&mut self, lulo_pool: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lulo_pool = Some(lulo_pool);
        self
    }
    /// LULO user account
    #[inline(always)]
    pub fn lulo_user(&mut self, lulo_user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lulo_user = Some(lulo_user);
        self
    }
    /// LULO deposit token account
    #[inline(always)]
    pub fn lulo_deposit(&mut self, lulo_deposit: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lulo_deposit = Some(lulo_deposit);
        self
    }
    /// LULO promotion reserve
    #[inline(always)]
    pub fn lulo_promotion_reserve(
        &mut self,
        lulo_promotion_reserve: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.lulo_promotion_reserve = Some(lulo_promotion_reserve);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// LULO program
    #[inline(always)]
    pub fn lulo_program(&mut self, lulo_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lulo_program = Some(lulo_program);
        self
    }
    /// `[optional account, default to 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL']`
    /// SPL associated token program
    #[inline(always)]
    pub fn ata_program(&mut self, ata_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.ata_program = Some(ata_program);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    /// SPL token program
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    /// System program
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    /// HPL parimutuel (LULO) program
    #[inline(always)]
    pub fn parimutuel_lulo_program(
        &mut self,
        parimutuel_lulo_program: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.parimutuel_lulo_program = Some(parimutuel_lulo_program);
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
        let accounts = ResolveParimutuelLuloV1 {
            resolver: self.resolver.expect("resolver is not set"),
            market: self.market.expect("market is not set"),
            request: self.request.expect("request is not set"),
            mint: self.mint.expect("mint is not set"),
            deposit: self.deposit.expect("deposit is not set"),
            lulo_pool: self.lulo_pool.expect("lulo_pool is not set"),
            lulo_user: self.lulo_user.expect("lulo_user is not set"),
            lulo_deposit: self.lulo_deposit.expect("lulo_deposit is not set"),
            lulo_promotion_reserve: self
                .lulo_promotion_reserve
                .expect("lulo_promotion_reserve is not set"),
            payer: self.payer.expect("payer is not set"),
            lulo_program: self.lulo_program.expect("lulo_program is not set"),
            ata_program: self
                .ata_program
                .unwrap_or(solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")),
            token_program: self
                .token_program
                .unwrap_or(solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
            parimutuel_lulo_program: self
                .parimutuel_lulo_program
                .expect("parimutuel_lulo_program is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `resolve_parimutuel_lulo_v1` CPI accounts.
pub struct ResolveParimutuelLuloV1CpiAccounts<'a, 'b> {
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token account
    pub deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO pool data
    pub lulo_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO user account
    pub lulo_user: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO deposit token account
    pub lulo_deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO promotion reserve
    pub lulo_promotion_reserve: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO program
    pub lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// HPL parimutuel (LULO) program
    pub parimutuel_lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `resolve_parimutuel_lulo_v1` CPI instruction.
pub struct ResolveParimutuelLuloV1Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Resolver
    pub resolver: &'b solana_program::account_info::AccountInfo<'a>,
    /// Market
    pub market: &'b solana_program::account_info::AccountInfo<'a>,
    /// Oracle request
    pub request: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token mint
    pub mint: &'b solana_program::account_info::AccountInfo<'a>,
    /// Deposit token account
    pub deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO pool data
    pub lulo_pool: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO user account
    pub lulo_user: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO deposit token account
    pub lulo_deposit: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO promotion reserve
    pub lulo_promotion_reserve: &'b solana_program::account_info::AccountInfo<'a>,
    /// Payer
    pub payer: &'b solana_program::account_info::AccountInfo<'a>,
    /// LULO program
    pub lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL associated token program
    pub ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// SPL token program
    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// System program
    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// HPL parimutuel (LULO) program
    pub parimutuel_lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ResolveParimutuelLuloV1Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ResolveParimutuelLuloV1CpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            resolver: accounts.resolver,
            market: accounts.market,
            request: accounts.request,
            mint: accounts.mint,
            deposit: accounts.deposit,
            lulo_pool: accounts.lulo_pool,
            lulo_user: accounts.lulo_user,
            lulo_deposit: accounts.lulo_deposit,
            lulo_promotion_reserve: accounts.lulo_promotion_reserve,
            payer: accounts.payer,
            lulo_program: accounts.lulo_program,
            ata_program: accounts.ata_program,
            token_program: accounts.token_program,
            system_program: accounts.system_program,
            parimutuel_lulo_program: accounts.parimutuel_lulo_program,
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
        let mut accounts = Vec::with_capacity(15 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.resolver.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.market.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.request.key, false));
        accounts
            .push(solana_program::instruction::AccountMeta::new_readonly(*self.mint.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.deposit.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.lulo_pool.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.lulo_user.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.lulo_deposit.key, false));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lulo_promotion_reserve.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(*self.payer.key, true));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.lulo_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.ata_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.parimutuel_lulo_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ResolveParimutuelLuloV1InstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ORACLE_RESOLVER_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(16 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.resolver.clone());
        account_infos.push(self.market.clone());
        account_infos.push(self.request.clone());
        account_infos.push(self.mint.clone());
        account_infos.push(self.deposit.clone());
        account_infos.push(self.lulo_pool.clone());
        account_infos.push(self.lulo_user.clone());
        account_infos.push(self.lulo_deposit.clone());
        account_infos.push(self.lulo_promotion_reserve.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.lulo_program.clone());
        account_infos.push(self.ata_program.clone());
        account_infos.push(self.token_program.clone());
        account_infos.push(self.system_program.clone());
        account_infos.push(self.parimutuel_lulo_program.clone());
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

/// Instruction builder for `ResolveParimutuelLuloV1` via CPI.
///
/// ### Accounts:
///
///   0. `[]` resolver
///   1. `[writable]` market
///   2. `[]` request
///   3. `[]` mint
///   4. `[writable]` deposit
///   5. `[writable]` lulo_pool
///   6. `[writable]` lulo_user
///   7. `[writable]` lulo_deposit
///   8. `[writable]` lulo_promotion_reserve
///   9. `[writable, signer]` payer
///   10. `[]` lulo_program
///   11. `[]` ata_program
///   12. `[]` token_program
///   13. `[]` system_program
///   14. `[]` parimutuel_lulo_program
#[derive(Clone, Debug)]
pub struct ResolveParimutuelLuloV1CpiBuilder<'a, 'b> {
    instruction: Box<ResolveParimutuelLuloV1CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ResolveParimutuelLuloV1CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ResolveParimutuelLuloV1CpiBuilderInstruction {
            __program: program,
            resolver: None,
            market: None,
            request: None,
            mint: None,
            deposit: None,
            lulo_pool: None,
            lulo_user: None,
            lulo_deposit: None,
            lulo_promotion_reserve: None,
            payer: None,
            lulo_program: None,
            ata_program: None,
            token_program: None,
            system_program: None,
            parimutuel_lulo_program: None,
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
    /// Deposit token mint
    #[inline(always)]
    pub fn mint(&mut self, mint: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.mint = Some(mint);
        self
    }
    /// Deposit token account
    #[inline(always)]
    pub fn deposit(
        &mut self,
        deposit: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.deposit = Some(deposit);
        self
    }
    /// LULO pool data
    #[inline(always)]
    pub fn lulo_pool(
        &mut self,
        lulo_pool: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lulo_pool = Some(lulo_pool);
        self
    }
    /// LULO user account
    #[inline(always)]
    pub fn lulo_user(
        &mut self,
        lulo_user: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lulo_user = Some(lulo_user);
        self
    }
    /// LULO deposit token account
    #[inline(always)]
    pub fn lulo_deposit(
        &mut self,
        lulo_deposit: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lulo_deposit = Some(lulo_deposit);
        self
    }
    /// LULO promotion reserve
    #[inline(always)]
    pub fn lulo_promotion_reserve(
        &mut self,
        lulo_promotion_reserve: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lulo_promotion_reserve = Some(lulo_promotion_reserve);
        self
    }
    /// Payer
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    /// LULO program
    #[inline(always)]
    pub fn lulo_program(
        &mut self,
        lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lulo_program = Some(lulo_program);
        self
    }
    /// SPL associated token program
    #[inline(always)]
    pub fn ata_program(
        &mut self,
        ata_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.ata_program = Some(ata_program);
        self
    }
    /// SPL token program
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    /// System program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    /// HPL parimutuel (LULO) program
    #[inline(always)]
    pub fn parimutuel_lulo_program(
        &mut self,
        parimutuel_lulo_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.parimutuel_lulo_program = Some(parimutuel_lulo_program);
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
        let instruction = ResolveParimutuelLuloV1Cpi {
            __program: self.instruction.__program,

            resolver: self.instruction.resolver.expect("resolver is not set"),

            market: self.instruction.market.expect("market is not set"),

            request: self.instruction.request.expect("request is not set"),

            mint: self.instruction.mint.expect("mint is not set"),

            deposit: self.instruction.deposit.expect("deposit is not set"),

            lulo_pool: self.instruction.lulo_pool.expect("lulo_pool is not set"),

            lulo_user: self.instruction.lulo_user.expect("lulo_user is not set"),

            lulo_deposit: self.instruction.lulo_deposit.expect("lulo_deposit is not set"),

            lulo_promotion_reserve: self
                .instruction
                .lulo_promotion_reserve
                .expect("lulo_promotion_reserve is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            lulo_program: self.instruction.lulo_program.expect("lulo_program is not set"),

            ata_program: self.instruction.ata_program.expect("ata_program is not set"),

            token_program: self.instruction.token_program.expect("token_program is not set"),

            system_program: self.instruction.system_program.expect("system_program is not set"),

            parimutuel_lulo_program: self
                .instruction
                .parimutuel_lulo_program
                .expect("parimutuel_lulo_program is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ResolveParimutuelLuloV1CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    resolver: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    market: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    request: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    deposit: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lulo_pool: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lulo_user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lulo_deposit: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lulo_promotion_reserve: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lulo_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ata_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    parimutuel_lulo_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(&'b solana_program::account_info::AccountInfo<'a>, bool, bool)>,
}
