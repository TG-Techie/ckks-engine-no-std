use crate::ckks::CKKSEncryptor;
use crate::polynomial::Polynomial;
use crate::utils::{mod_reduce};

impl CKKSEncryptor{

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

    pub fn homomorphic_subtract(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before subtraction: {:?}", cipher1);
        println!("Ciphertext 2 before subtraction: {:?}", cipher2);

        // Subtract ciphertexts
        let result = cipher1.subtract(cipher2);
        println!("Result after homomorphic subtraction: {:?}", result);

        // Modular reduction using the prime modulus
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);

        reduced_result
    }

    pub fn homomorphic_multiply(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {
        println!("Ciphertext 1 before multiplication: {:?}", cipher1);
        println!("Ciphertext 2 before multiplication: {:?}", cipher2);
    
        // Result size is determined by the maximum degree of the polynomials
        
        // Create the resulting polynomial
        let result = cipher1.multiply(cipher2);
        println!("Result after polynomial multiplication: {:?}", result);
    
        // Modular reduction is still applied to the result
        let reduced_result = mod_reduce(&result, self.params.modulus);
        println!("Result after mod reduction: {:?}", reduced_result);
    
        reduced_result
    }    

    pub fn homomorphic_negation(&self, cipher1: &Polynomial) -> Polynomial {
        println!("Ciphertext before negation: {:?}", cipher1);
        
        // Negate the coefficients of the polynomial
        
        // Create a new Polynomial with negated coefficients
        let negated_poly = cipher1.negation();
        println!("Negated Polynomial before mod reduction: {:?}", negated_poly);
        
        // Apply modular reduction
        let reduced_result = mod_reduce(&negated_poly, self.params.modulus);
        println!("Negated Polynomial after mod reduction: {:?}", reduced_result);
        
        reduced_result
    }
    
}