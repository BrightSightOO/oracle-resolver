//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum OracleResolverError {
    /// 0 - Failed to deserialize account
    #[error("Failed to deserialize account")]
    DeserializationError = 0x0,
    /// 1 - Failed to serialize account
    #[error("Failed to serialize account")]
    SerializationError = 0x1,
    /// 2 - Invalid request kind
    #[error("Invalid request kind")]
    InvalidRequestKind = 0x2,
}

impl solana_program::program_error::PrintProgramError for OracleResolverError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
