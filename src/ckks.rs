use crate::polynomial::Polynomial;
use crate::keygen::{PublicKey, SecretKey};
use crate::utils::{encode_float, decode_float, mod_reduce,encode_integers,decode_integers,add_noise};

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
    pub_key: PublicKey,
    params: CkksParameters,  
}

impl CKKSEncryptor {
    pub fn new(pub_key: PublicKey, params: CkksParameters) -> Self {
        Self { pub_key, params }
    }

    pub fn encrypt(&self, plaintext: &[f64]) -> Polynomial {
        // Step 1: Encode the plaintext into a polynomial
        let scaling_factor = 10000000000000.0;  // Set a scaling factor for encoding
        let encoded = encode_float(plaintext, scaling_factor);
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

    pub fn encrypt_integers(&self, plaintext: &[i64]) -> Polynomial {
        // Step 1: Encode the plaintext integers into a polynomial
        let scaling_factor = 10000000000000.0;  // Set a scaling factor for encoding
        let encoded = encode_integers(plaintext, scaling_factor);
        println!("Encoded plaintext: {:?}", encoded);

        // Step 2: Use public key for encryption (same logic as before)
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

    pub fn homomorphic_add(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before addition: {:?}", cipher1);
        println!("Ciphertext 2 before addition: {:?}", cipher2);

        // Add ciphertexts (assuming they have the same scaling factor)
        let result = cipher1.add(cipher2);
        println!("Result after homomorphic addition: {:?}", result);

        // Perform modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_sub(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before subtraction: {:?}", cipher1);
        println!("Ciphertext 2 before subtraction: {:?}", cipher2);

        // Subtract ciphertexts
        let result = cipher1.sub(cipher2);
        println!("Result after homomorphic subtraction: {:?}", result);

        // Modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_multiply(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before multiplication: {:?}", cipher1);
        println!("Ciphertext 2 before multiplication: {:?}", cipher2);
    
        // Result size based on cipher1's length
        let result_size = cipher1.coeffs.len() + cipher2.coeffs.len() - 1; // Adjust size for polynomial degree
        let mut result_coeffs = vec![0.0; result_size]; // Use f64 for coefficients
    
        // Multiply coefficients
        for (i, &c1) in cipher1.coeffs.iter().enumerate() {
            for (j, &c2) in cipher2.coeffs.iter().enumerate() {
                // Ensure we stay within the bounds of result size
                if i + j < result_size {
                    result_coeffs[i + j] += (c1 as f64 * c2 as f64) / 10000000000000.0; // Scale and accumulate
                }
            }
        }
    
        // Create the resulting polynomial
        let result = Polynomial::new(result_coeffs.iter().map(|&x| x.round() as i64).collect());
        println!("Result after polynomial multiplication: {:?}", result);
    
        // Modular reduction is still applied to the result
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);
    
        reduced_result
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

    pub fn decrypt(&self, ciphertext: &Polynomial, is_multiplication: bool) -> Vec<f64> {
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
        let decoded = decode_float(&decrypted_polynomial, scaling_factor, is_multiplication);
        println!("Decoded plaintext (after decryption): {:?}", decoded);

        decoded
    }

    pub fn decrypt_integers(&self, ciphertext: &Polynomial) -> Vec<i64> {
        println!("Ciphertext before decryption: {:?}", ciphertext);

        // Step 1: Perform modular reduction
        let reduced_poly = mod_reduce(ciphertext, self.params.modulus);
        println!("Decrypted polynomial (after mod reduction): {:?}", reduced_poly);

        // Step 2: Apply the secret key to reverse the public key's effect
        let decrypted_poly: Vec<i64> = reduced_poly.coeffs.iter()
            .zip(&self.sec_key.poly)
            .map(|(&c, &sk)| c - sk)  // Subtract secret key influence
            .collect();
        let decrypted_polynomial = Polynomial::new(decrypted_poly);
        println!("Decrypted polynomial (after applying secret key): {:?}", decrypted_polynomial);

        // Step 3: Decode the result and scale it back to retrieve the original integer value
        let scaling_factor = 10000000000000.0;  // Use the same scaling factor as in encryption
        let decoded: Vec<i64> = decode_integers(&decrypted_polynomial, scaling_factor);
        
        // Print the decoded integers
        println!("Decoded integers: {:?}", decoded);
        
        decoded
    }
}    
