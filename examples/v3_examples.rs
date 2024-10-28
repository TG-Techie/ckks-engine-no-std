// v3_examples.rs

use ckks_engine::*;
use log::info;
use env_logger;

fn main(){
    // Initialize the logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    // Run the homomorphic divide test
    test_homomorphic_divide();
}

fn test_homomorphic_divide() {
    // Initialize parameters with a higher polynomial degree
    let degree = 2048;
    let params = CkksParameters::new(degree, 1000000000000007);

    // Initialize key generator
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize encryptor and decryptor
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Encrypt numerator and denominator
    let numerator = 500;
    let denominator = 10;
    let encrypted_numerator = encryptor.encrypt_value(numerator);
    let encrypted_denominator = encryptor.encrypt_value(denominator);

    // Perform division with 5 iterations for reciprocal approximation
    let encrypted_division = encryptor.homomorphic_divide(&encrypted_numerator, &encrypted_denominator, 10);

    // Decrypt the result as integers
    let decrypted_division = decryptor.decrypt_as_int(&encrypted_division);
    info!("Decrypted division result: {:?}", decrypted_division);
}
