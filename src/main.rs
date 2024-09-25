mod ckks;
mod keygen;
mod polynomial;
mod utils;

// use crate::polynomial::Polynomial;
use ckks::{CKKSEncryptor, CKKSDecryptor, CkksParameters};
use keygen::KeyGenerator;

fn main() {

    // Set parameters: degree of polynomial (e.g., N = 2048) and prime modulus (q)
    let params = CkksParameters::new(2048, 1_000_000_007);

    // Key generation
    let keygen = KeyGenerator::new();
    // let (public_key, secret_key, _) = keygen.generate_keys();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor with parameters
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Plaintext to be encrypted
    let plaintext1 = vec![1.23, 4.56, 7.89]; 
    let plaintext2 = vec![0.98, 2.34, 5.67]; 

    println!("\n=== Encrypting plaintext 1 ===");
    let ciphertext1 = encryptor.encrypt(&plaintext1);
    
    println!("\n=== Encrypting plaintext 2 ===");
    let ciphertext2 = encryptor.encrypt(&plaintext2);

    println!("\n=== Homomorphic Addition ===");
    let encrypted_sum = encryptor.homomorphic_add(&ciphertext1, &ciphertext2);

    println!("\n=== Homomorphic Subtraction ===");
    let encrypted_diff = encryptor.homomorphic_sub(&ciphertext1, &ciphertext2);

    println!("\n=== Decrypting Sum ===");
    let decrypted_sum = decryptor.decrypt(&encrypted_sum, false);

    println!("\n=== Decrypting Difference ===");
    let decrypted_diff = decryptor.decrypt(&encrypted_diff, false);

    println!("\nDecrypted sum: {:?}", decrypted_sum);
    println!("Decrypted difference: {:?}", decrypted_diff);

}
