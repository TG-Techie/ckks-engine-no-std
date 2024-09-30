use crate::ckks::CKKSEncryptor;
use crate::polynomial::Polynomial;
use crate::utils::{mod_reduce};
use log::{info};
impl CKKSEncryptor {

    // Function to perform homomorphic addition on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_add(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        // Print ciphertexts before addition for debugging
        info!("Ciphertext 1 before addition: {:?}", cipher1);
        info!("Ciphertext 2 before addition: {:?}", cipher2);

        // Add the two polynomials (ciphertexts). Assuming the ciphertexts have the same scaling factor
        let result = cipher1.add(cipher2);
        info!("Result after homomorphic addition: {:?}", result);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        info!("Result after mod reduction: {:?}", reduced_result);

        // Return the reduced result as the final homomorphic addition result
        reduced_result
    }

    // Function to perform homomorphic subtraction on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_subtract(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        // Print ciphertexts before subtraction for debugging
        info!("Ciphertext 1 before subtraction: {:?}", cipher1);
        info!("Ciphertext 2 before subtraction: {:?}", cipher2);

        // Subtract the second polynomial (cipher2) from the first (cipher1)
        let result = cipher1.subtract(cipher2);
        info!("Result after homomorphic subtraction: {:?}", result);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        info!("Result after mod reduction: {:?}", reduced_result);

        // Return the reduced result as the final homomorphic subtraction result
        reduced_result
    }

    // Function to perform homomorphic multiplication on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_multiply(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        // Print ciphertexts before multiplication for debugging
        info!("Ciphertext 1 before multiplication: {:?}", cipher1);
        info!("Ciphertext 2 before multiplication: {:?}", cipher2);
    
        // Multiply the two polynomials (ciphertexts). The result size is determined by the degree of the polynomials
        let result = cipher1.multiply(cipher2);
        info!("Result after polynomial multiplication: {:?}", result);
    
        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        info!("Result after mod reduction: {:?}", reduced_result);
    
        // Return the reduced result as the final homomorphic multiplication result
        reduced_result
    }    

    // Function to perform homomorphic negation on an encrypted polynomial (ciphertext)
    pub fn homomorphic_negation(&self, cipher1: &Polynomial) -> Polynomial {
        // Print the ciphertext before negation for debugging
        info!("Ciphertext before negation: {:?}", cipher1);
        
        // Negate the coefficients of the polynomial (ciphertext)
        let negated_poly = cipher1.negation();
        info!("Negated Polynomial before mod reduction: {:?}", negated_poly);
        
        // Perform modular reduction to ensure the negated result fits within the modulus
        let reduced_result = mod_reduce(&negated_poly, self.params.modulus);
        info!("Negated Polynomial after mod reduction: {:?}", reduced_result);
        
        // Return the reduced result as the final homomorphic negation result
        reduced_result
    }
}
