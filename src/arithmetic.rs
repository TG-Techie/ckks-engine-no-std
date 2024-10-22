use crate::ckks::CKKSEncryptor;
use crate::polynomial::Polynomial;
use crate::utils::{mod_reduce};
use log::{info};
impl CKKSEncryptor {

    // Function to perform homomorphic addition on two encrypted polynomials (ciphertexts)
    pub fn homomorphic_add(&self, cipher1: &Polynomial, cipher2: &Polynomial) -> Polynomial {

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
        
        // Negate the coefficients of the polynomial (ciphertext)
        let negated_poly = cipher1.negation();
        info!("Negated Polynomial before mod reduction: {:?}", negated_poly);
        
        // Perform modular reduction to ensure the negated result fits within the modulus
        let reduced_result = mod_reduce(&negated_poly, self.params.modulus);
        info!("Negated Polynomial after mod reduction: {:?}", reduced_result);
        
        // Return the reduced result as the final homomorphic negation result
        reduced_result
    }

    // Function to perform homomorphic ceil on encrypted polynomials (ciphertexts)
    pub fn homomorphic_ceil(&self, cipher: &Polynomial) -> Polynomial {
        // This function will operate on encrypted coefficients
        let ceil_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // scale down
            let ceil_value = scaled_value.ceil() as i64; // apply ceil
            (ceil_value as i64) * (1e7 as i64) // scale up back after ceil
        }).collect();

        // Return the new polynomial with ceil applied on encrypted data
        let ceil_polynomial = Polynomial::new(ceil_poly);
        info!("Polynomial after homomorphic ceil: {:?}", ceil_polynomial);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&ceil_polynomial, self.params.modulus);
        info!("Result after mod reduction (ceil): {:?}", reduced_result);

        // Return the reduced result
        reduced_result
    }

    // Function to perform homomorphic floor on encrypted polynomials (ciphertexts)
    pub fn homomorphic_floor(&self, cipher: &Polynomial) -> Polynomial {
        // This function will operate on encrypted coefficients
        let floor_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // scale down
            let floor_value = scaled_value.floor() as i64; // apply floor
            (floor_value as i64) * (1e7 as i64) // scale up back after floor
        }).collect();

        // Return the new polynomial with floor applied on encrypted data
        let floor_polynomial = Polynomial::new(floor_poly);
        info!("Polynomial after homomorphic floor: {:?}", floor_polynomial);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&floor_polynomial, self.params.modulus);
        info!("Result after mod reduction (floor): {:?}", reduced_result);

        // Return the reduced result
        reduced_result
    }


    // Function to perform homomorphic round on encrypted polynomials (ciphertexts)
    pub fn homomorphic_round(&self, cipher: &Polynomial) -> Polynomial {
        // Operate on encrypted coefficients
        let round_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // Scale down
            let rounded_value = scaled_value.round() as i64; // Apply round
            (rounded_value as i64) * (1e7 as i64) // Scale up back after rounding
        }).collect();

        // Create a new polynomial with rounded coefficients
        let rounded_polynomial = Polynomial::new(round_poly);
        info!("Polynomial after homomorphic round: {:?}", rounded_polynomial);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&rounded_polynomial, self.params.modulus);
        info!("Result after mod reduction (round): {:?}", reduced_result);

        // Return the reduced result
        reduced_result
    }

    // Function to perform homomorphic truncate on encrypted polynomials (ciphertexts)
    pub fn homomorphic_truncate(&self, cipher: &Polynomial) -> Polynomial {
        // Operate on encrypted coefficients
        let truncate_poly: Vec<i64> = cipher.coeffs.iter().map(|&c| {
            let scaled_value = (c as f64) / 1e7; // Scale down
            let truncated_value = scaled_value.trunc() as i64; // Apply truncate (floor)
            (truncated_value as i64) * (1e7 as i64) // Scale up back after truncation
        }).collect();

        // Create a new polynomial with truncated coefficients
        let truncated_polynomial = Polynomial::new(truncate_poly);
        info!("Polynomial after homomorphic truncate: {:?}", truncated_polynomial);

        // Perform modular reduction to ensure the result fits within the modulus
        let reduced_result = mod_reduce(&truncated_polynomial, self.params.modulus);
        info!("Result after mod reduction (truncate): {:?}", reduced_result);

        // Return the reduced result
        reduced_result
    }



}
