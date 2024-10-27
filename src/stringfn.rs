use crate::polynomial::Polynomial;
use crate::ckks::CKKSEncryptor;

impl CKKSEncryptor {
    /// Function to calculate the homomorphic length of an encrypted string.
    /// This is the number of non-zero coefficients in the encrypted polynomial.
    pub fn homomorphic_length(&self, encrypted_poly: &Polynomial) -> usize {
        // Filter out zero coefficients and count the remaining non-zero ones
        let non_zero_coeffs = encrypted_poly.coeffs.iter().filter(|&&coeff| coeff != 0).count();
        
        // Return the number of non-zero coefficients
        non_zero_coeffs
    }


    /// Function to concatenate two encrypted strings by combining their encrypted polynomials.
    /// This will create a new polynomial containing the coefficients of both encrypted strings.
    
    pub fn concatenate_encrypted_strings(&self, encrypted_poly1: &Polynomial, encrypted_poly2: &Polynomial) -> Polynomial {
        // Combine the coefficients of both encrypted polynomials
        let mut combined_coeffs = encrypted_poly1.coeffs.clone();
        combined_coeffs.extend_from_slice(&encrypted_poly2.coeffs);
        
        // Create and return the new concatenated polynomial
        Polynomial::new(combined_coeffs)
    }
    
}
