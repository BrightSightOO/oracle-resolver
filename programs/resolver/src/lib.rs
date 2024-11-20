#![deny(clippy::disallowed_macros, clippy::disallowed_methods, clippy::disallowed_types)]

#[macro_use]
mod macros;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod pda;
pub mod processor;
pub mod state;
pub mod utils;

// Export sdk types for downstream users with a different sdk version.
pub use solana_program;

#[cfg(not(feature = "no-entrypoint"))]
security_txt::security_txt! {
    // Required fields.
    name: "Oracle Resolver",
    project_url: "https://github.com/BrightSightOO",
    contacts: "email:james@hedgehog.markets",
    policy: "https://github.com/BrightSightOO/oracle-resolver/security",

    // Optional fields.
    source_code: "https://github.com/BrightSightOO/oracle-resolver",
}

#[cfg(not(feature = "no-entrypoint"))]
include_idl::include_idl!(concat!(env!("OUT_DIR"), "/solana.idl.zip"));

solana_program::declare_id!("RESwds5X9Yj1kzXkjuA5ncR8TqhHeqj7qcrUz9QM29f");
