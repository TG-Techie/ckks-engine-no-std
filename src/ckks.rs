// ckks.rs

use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8, ClientKey};
use concrete::prelude::*;

pub fn encrypt_message(plaintext: u8, client_key: &ClientKey) -> FheUint8 {
    FheUint8::encrypt(plaintext, client_key)
}

pub fn decrypt_message(ciphertext: &FheUint8, client_key: &ClientKey) -> u8 {
    ciphertext.decrypt(client_key)
}
