// main.rs

use ckks_engine::ckks;
use concrete::{ConfigBuilder, generate_keys, set_server_key, FheUint8};

fn main() {
    let config = ConfigBuilder::all_disabled()
        .enable_default_uint8()
        .build();

    let (client_key, server_key) = generate_keys(config);
    set_server_key(server_key);

    let clear_a = 27u8;
    let clear_b = 128u8;

    // Encrypt the messages
    let a = ckks::encrypt_message(clear_a, &client_key);
    let b = ckks::encrypt_message(clear_b, &client_key);

    // Perform the addition
    let result = a + b;

    // Decrypt the result
    let decrypted_result = ckks::decrypt_message(&result, &client_key);

    let clear_result = clear_a + clear_b;

    assert_eq!(decrypted_result, clear_result);
    
    println!("Decrypted result: {}", decrypted_result);
}
