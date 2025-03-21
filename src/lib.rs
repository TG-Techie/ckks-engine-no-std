// lib.rs
#![no_std]

#[macro_use]
extern crate alloc;

// Declare modules
pub mod arithmetic;
pub mod ckks;
pub mod keygen;
pub mod polynomial;
pub mod stringfn;
pub mod utils;

// CHANGE: added float_polyfill to support no_std
pub(crate) mod float_polyfill;

// Re-export key structs/functions from modules for easy access
pub use ckks::{CKKSDecryptor, CKKSEncryptor, CkksParameters};
pub use keygen::{KeyGenerator, PublicKey, SecretKey};
pub use polynomial::Polynomial;
