use crate::polynomial::Polynomial;
use crate::keygen::{PublicKey, SecretKey};
use crate::utils::{encode, decode, mod_reduce,add_noise};

#[derive(Debug, Clone)]  
pub struct CkksParameters {
    pub degree: usize,   // Polynomial degree N
    pub modulus: i64,    // Prime modulus q
}

impl CkksParameters {
    pub fn new(degree: usize, modulus: i64) -> Self {
        CkksParameters { degree, modulus }
    }
}

pub struct CKKSEncryptor {
    pub pub_key: PublicKey,
    pub params: CkksParameters,  
}

impl CKKSEncryptor {
    pub fn new(pub_key: PublicKey, params: CkksParameters) -> Self {
        Self { pub_key, params }
    }

    pub fn encrypt_collection<T>(&self, plaintext: &[T]) -> Polynomial
    where
    T: Into<f64> + Copy,  // T can be converted into f64 and must implement Copy
{
    // Step 1: Encode the plaintext into a polynomial
    let scaling_factor = 10000000000000.0;  // Set a scaling factor for encoding
    let plaintext_f64: Vec<f64> = plaintext.iter().map(|&x| x.into()).collect();  // Convert to f64
    let encoded = encode(&plaintext_f64, scaling_factor);
    println!("Encoded plaintext: {:?}", encoded);

    // Step 2: Use public key for encryption
    let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
        .zip(&self.pub_key.pk_0)
        .zip(&self.pub_key.pk_1)
        .map(|((&e, &pk0), &pk1)| e + pk0 * pk1)  // Encrypt: encoded + pk_0 * pk_1
        .collect();
    let encrypted_polynomial = Polynomial::new(encrypted_poly);
    println!("Encrypted polynomial: {:?}", encrypted_polynomial);

    // Step 3: Perform modular reduction using the prime modulus
    let ciphertext = mod_reduce(&encrypted_polynomial, self.params.modulus);
    println!("Ciphertext (after mod reduction): {:?}", ciphertext);

    ciphertext
}


    pub fn encrypt_value<T>(&self, plaintext: T) -> Polynomial
    where
    T: Into<f64> + Copy, // Accepts a type that can be converted into f64
    {
    // Step 1: Convert the input value into a vector of f64
    let plaintext_vec: Vec<f64> = vec![plaintext.into()];

    // Step 2: Encode the plaintext into a polynomial
    let scaling_factor = 10000000000000.0; // Set a scaling factor for encoding
    let encoded = encode(&plaintext_vec, scaling_factor);
    println!("Encoded plaintext: {:?}", encoded);

    // Step 3: Use public key for encryption
    let encrypted_poly: Vec<i64> = encoded.coeffs.iter()
        .zip(&self.pub_key.pk_0)
        .zip(&self.pub_key.pk_1)
        .map(|((&e, &pk0), &pk1)| e + pk0 * pk1) // Encrypt: encoded + pk_0 * pk_1
        .collect();
    let encrypted_polynomial = Polynomial::new(encrypted_poly);
    println!("Encrypted polynomial: {:?}", encrypted_polynomial);

    // Step 4: Perform modular reduction using the prime modulus
    let ciphertext = mod_reduce(&encrypted_polynomial, self.params.modulus);
    println!("Ciphertext (after mod reduction): {:?}", ciphertext);

    ciphertext
    }


}

pub struct CKKSDecryptor {
    sec_key: SecretKey,
    params: CkksParameters,  
}

impl CKKSDecryptor {
    pub fn new(sec_key: SecretKey, params: CkksParameters) -> Self {
        Self { sec_key, params }
    }

    pub fn decrypt(&self, ciphertext: &Polynomial) -> Vec<f64> {
        println!("Ciphertext before decryption: {:?}", ciphertext);

        // Step 1: Perform modular reduction to keep coefficients manageable
        let reduced_poly = mod_reduce(ciphertext, self.params.modulus);
        println!("Decrypted polynomial (after mod reduction): {:?}", reduced_poly);

        // Step 2: Apply the secret key to reverse the public key's effect
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk)  // Subtract secret key influence
            .collect();
        let decrypted_polynomial = Polynomial::new(decrypted_poly);
        println!("Decrypted polynomial (after applying secret key): {:?}", decrypted_polynomial);

        // Step 3: Decode the result and scale it back to retrieve the original value
        let scaling_factor = 10000000000000.0;  // Use the same scaling factor as in encryption
        let decoded = decode(&decrypted_polynomial, scaling_factor);
        println!("Decoded plaintext (after decryption): {:?}", decoded);

        decoded
    }
}    