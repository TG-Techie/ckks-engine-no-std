use ckks_engine::*;
use log::info;

fn test_homomorphic_divide() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
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
    let numerator = [10,20,30];
    let denominator = 2;
    let encrypted_numerator = encryptor.encrypt_collection(&numerator);
    let encrypted_denominator = encryptor.encrypt_value(denominator);

    // Perform division with 5 iterations for reciprocal approximation
    let encrypted_division = encryptor.homomorphic_divide(&encrypted_numerator, &encrypted_denominator);

    // Decrypt the result as integers
    let decrypted_division = decryptor.decrypt(&encrypted_division);
    info!("Decrypted division result: {:?}", decrypted_division);
}

fn main(){
    test_homomorphic_divide()
}