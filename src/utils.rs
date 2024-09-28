use crate::polynomial::Polynomial;
use rand::Rng;

// Define a constant scaling factor for encoding and decoding
pub const SCALE: f64 = 1e5;  // Use a fixed scaling factor

// Rounds a given value to a specified number of decimal places
fn round_to(value: f64, decimal_places: usize) -> f64 {
    let factor = 10f64.powi(decimal_places as i32); // Calculate the rounding factor
    (value * factor).round() / factor  // Round the value and return
}

// Encode real numbers into polynomial form with scaling
pub fn encode(plaintext: &[f64], scaling_factor: f64) -> Polynomial {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");  // Ensure the scaling factor is positive
    }
    // Print the input plaintext and scaling factor
    println!("Encoding real numbers {:?} with scaling factor {}", plaintext, scaling_factor);
    
    // Scale the real numbers and convert them to integer coefficients
    let coeffs: Vec<i64> = plaintext.iter()
        .map(|&x| (x * scaling_factor).round() as i64)  // Scale the real numbers
        .collect();
    
    // Print the resulting polynomial coefficients
    println!("Encoded polynomial coefficients: {:?}", coeffs);
    
    Polynomial::new(coeffs)  // Return a new polynomial with the coefficients
}

// Decode polynomial back to real numbers
pub fn decode(ciphertext: &Polynomial, scaling_factor: f64) -> Vec<f64> {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");  // Ensure the scaling factor is positive
    }
    let threshold = 1e-10; // Define a small threshold for considering values as zero
    let decimal_places = 2; // Number of decimal places for rounding

    // Print the input ciphertext and scaling factor
    println!("Decoding polynomial coefficients {:?} with scaling factor {}", ciphertext.coeffs, scaling_factor);

    // Perform decoding (reverse scaling) and apply thresholding and rounding
    let decoded_values: Vec<f64> = ciphertext.coeffs.iter()
        .map(|&c| {
            let value = (c as f64) / scaling_factor;  // Reverse scaling
            let rounded_value = round_to(value, decimal_places); // Round the value to 2 decimal places
            // Apply thresholding to treat small values as zero
            if rounded_value.abs() < threshold {
                0.0  // Treat small values as zero
            } else {
                rounded_value  // Return the rounded value
            }
        })
        .collect();
    
    // Print the decoded real numbers
    println!("Decoded real numbers (with thresholding and rounding): {:?}", decoded_values);

    decoded_values  // Return the decoded values
}

// Add noise to a polynomial
pub fn add_noise(poly: &Polynomial, _pub_key: &impl std::fmt::Debug) -> Polynomial {
    let mut rng = rand::thread_rng();  // Create a random number generator
    // Generate noise for each coefficient of the polynomial
    let noise: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff + rng.gen_range(-10..10)).collect();
    println!("Adding noise to polynomial {:?}. Result after noise addition: {:?}", poly.coeffs, noise);
    Polynomial::new(noise)  // Return a new polynomial with added noise
}

// Modular reduction using the prime modulus q
pub fn mod_reduce(poly: &Polynomial, modulus: i64) -> Polynomial {
    // Reduce each coefficient of the polynomial modulo the given modulus
    let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff % modulus).collect();
    println!("Performing modular reduction on polynomial {:?}. Result after mod reduction: {:?}", poly.coeffs, reduced);
    Polynomial::new(reduced)  // Return a new polynomial with reduced coefficients
}
