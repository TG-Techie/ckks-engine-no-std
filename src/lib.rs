// lib.rs

// Declare modules
pub mod ckks;
pub mod polynomial;
pub mod keygen;
pub mod utils;
pub mod arithmetic;
pub mod stringfn;

use std::env;
use log::info;
// Re-export key structs/functions from modules for easy access
pub use ckks::{CKKSEncryptor, CKKSDecryptor, CkksParameters};
pub use polynomial::Polynomial;
pub use keygen::{PublicKey, SecretKey, KeyGenerator};






