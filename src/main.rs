mod ckks;
mod keygen;
mod polynomial;
mod utils;

// use crate::polynomial::Polynomial;
use ckks::{CKKSEncryptor, CKKSDecryptor, CkksParameters};
use keygen::KeyGenerator;

fn main() {

    // Set parameters: degree of polynomial (e.g., N = 2048) and prime modulus (q)
    let params = CkksParameters::new(2048, 1000000000000007);

    // Key generation
    let keygen = KeyGenerator::new();
    // let (public_key, secret_key, _) = keygen.generate_keys();
    let (public_key, secret_key) = keygen.generate_keys();

    // Initialize CKKS encryptor and decryptor with parameters
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Plaintext to be encrypted
    let plaintext1 = vec![1.34, 2.56, 3.78]; 
    let plaintext2 = vec![4.32, 5.26, 3.78]; 

    println!("\n=== Encrypting plaintext 1 ===");
    let ciphertext1 = encryptor.encrypt(&plaintext1);
    
    println!("\n=== Encrypting plaintext 2 ===");
    let ciphertext2 = encryptor.encrypt(&plaintext2);

    println!("\n=== Decrypting ciphertext 1 ===");
    let decrypted_plaintext1 = decryptor.decrypt(&ciphertext1, false);
    println!("Decrypted plaintext 1: {:?}", decrypted_plaintext1);

    println!("\n=== Decrypting ciphertext 2 ===");
    let decrypted_plaintext2 = decryptor.decrypt(&ciphertext2, false);
    println!("Decrypted plaintext 2: {:?}", decrypted_plaintext2);

    // Example integer plaintext to be encrypted
    let plaintext_integers = vec![2, 4, 8]; 
    let plaintext_integers2 = vec![4,2,8];
    let neg = vec![-1,2,-3];

    println!("\n=== Encrypting integers ===");
    let ciphertext_integers = encryptor.encrypt_integers(&plaintext_integers);
    let ciphertext_integers2 = encryptor.encrypt_integers(&plaintext_integers2);
    let neg_cipher = encryptor.encrypt_integers(&neg);


    println!("\n=== Decrypting integers ===");
    let decrypted_integers = decryptor.decrypt_integers(&ciphertext_integers);

    println!("Decrypted integers: {:?}", decrypted_integers);


    println!("\n=== Homomorphic Addition ===");
    let encrypted_sum = encryptor.homomorphic_add(&ciphertext1, &ciphertext2);

    let encrypted_int_sum = encryptor.homomorphic_add(&ciphertext_integers,&ciphertext_integers2);


    let encrypted_int_mult = encryptor.homomorphic_multiply(&ciphertext_integers,&ciphertext_integers2);
    let neg_encrypted = encryptor.homomorphic_negate(&neg_cipher);


    println!("\n=== Homomorphic Subtraction ===");
    let encrypted_diff = encryptor.homomorphic_sub(&ciphertext1, &ciphertext2);
    let encrypted_diff_int = encryptor.homomorphic_sub(&ciphertext_integers,&ciphertext_integers2);


    println!("\n=== Decrypting Sum ===");
    let decrypted_sum = decryptor.decrypt(&encrypted_sum, false);
    let decrypted_sum_int = decryptor.decrypt_integers(&encrypted_int_sum);

    println!("\n=== Decrypting Difference ===");
    let decrypted_diff = decryptor.decrypt(&encrypted_diff, false);
    let decrypted_diff_int = decryptor.decrypt(&encrypted_diff_int, false);

    let decrypted_mult = decryptor.decrypt(&encrypted_int_mult,false);
    let neg_decrypt = decryptor.decrypt(&neg_encrypted,false);

    println!("\nDecrypted sum: {:?}", decrypted_sum);
    println!("Decrypted difference: {:?}", decrypted_diff);
    println!("\nDecrypted sum: {:?}", decrypted_sum_int);
    println!("\nDecrypted difference: {:?}", decrypted_diff_int);

    println!("\nDecrypted multiply: {:?}", decrypted_mult);
    println!("\nNegated values: {:?}",neg_decrypt);

}
