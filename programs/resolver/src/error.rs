use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use solana_utils::log;
use thiserror::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Error, FromPrimitive)]
pub enum ResolverError {
    #[error("Failed to deserialize account")]
    DeserializationError,

    #[error("Failed to serialize account")]
    SerializationError,

    #[error("Invalid request kind")]
    InvalidRequestKind,

    #[error("Request is not resolved")]
    RequestNotResolved,
}

impl PrintProgramError for ResolverError {
    fn print<E>(&self) {
        log!("Error: {self}");
    }
}

impl From<ResolverError> for ProgramError {
    fn from(e: ResolverError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ResolverError {
    fn type_of() -> &'static str {
        "ResolverError"
    }
}
