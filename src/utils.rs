use crate::polynomial::Polynomial;
// CHANGED: removed use of the log crate
// use log::info;

// CHANGE: include support for abs, ceil, floor, round, powi for f64

// CHANGED(MUST_CHANGE): switched to using mock rng to avoid std dependency
use rand::{self, Rng}; // Importing random number generator traits

// CHANGED: added Vec for no_std
use alloc::vec::Vec;

// CHANGED: added String for no_std
use alloc::string::String;

// CHANGED: added FloatPolyfill for no_std
use crate::float_polyfill::FloatPolyfill;

// Rounds a given value to a specified number of decimal places
fn round_to(value: f64, decimal_places: usize) -> f64 {
    let factor = 10f64.powi(decimal_places as i32); // Calculate the rounding factor
    (value * factor).round() / factor // Round the value and return
}

// Encode real numbers into polynomial form with scaling
pub fn encode(plaintext: &[f64], scaling_factor: f64) -> Polynomial {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive"); // Ensure the scaling factor is positive
    }
    // Print the input plaintext and scaling factor
    // info!(
    //     "Encoding real numbers {:?} with scaling factor {}",
    //     plaintext, scaling_factor
    // );

    // Scale the real numbers and convert them to integer coefficients
    let coeffs: Vec<i64> = plaintext
        .iter()
        .map(|&x| (x * scaling_factor).round() as i64) // Scale the real numbers
        .collect();

    // Print the resulting polynomial coefficients
    // info!("Encoded polynomial coefficients: {:?}", coeffs);

    Polynomial::new(coeffs) // Return a new polynomial with the coefficients
}

// Decode polynomial back to real numbers
pub fn decode(ciphertext: &Polynomial, scaling_factor: f64) -> Vec<f64> {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive"); // Ensure the scaling factor is positive
    }
    let threshold = 1e-10; // Define a small threshold for considering values as zero
    let decimal_places = 2; // Number of decimal places for rounding

    // Print the input ciphertext and scaling factor
    // info!(
    //     "Decoding polynomial coefficients {:?} with scaling factor {}",
    //     ciphertext.coeffs, scaling_factor
    // );

    // Perform decoding (reverse scaling) and apply thresholding and rounding
    let decoded_values: Vec<f64> = ciphertext
        .coeffs
        .iter()
        .map(|&c| {
            let value = (c as f64) / scaling_factor; // Reverse scaling
            let rounded_value = round_to(value, decimal_places); // Round the value to 2 decimal places
                                                                 // Apply thresholding to treat small values as zero
            if rounded_value.abs() < threshold {
                0.0 // Treat small values as zero
            } else {
                rounded_value // Return the rounded value
            }
        })
        .collect();

    // Print the decoded real numbers
    // info!(
    //     "Decoded real numbers (with thresholding and rounding): {:?}",
    //     decoded_values
    // );

    decoded_values // Return the decoded values
}

// Add noise to a polynomial
pub fn add_noise(poly: &Polynomial, _pub_key: &impl core::fmt::Debug) -> Polynomial {
    let mut rng = rand::rngs::mock::StepRng::new(0, 1); // Create a new random number generator using the mock_rng crate
                                                        // Generate noise for each coefficient of the polynomial
    let noise: Vec<i64> = poly
        .coeffs
        .iter()
        .map(|&coeff| coeff + rng.gen_range(-10..10))
        .collect();
    // info!(
    //     "Adding noise to polynomial {:?}. Result after noise addition: {:?}",
    //     poly.coeffs, noise
    // );
    Polynomial::new(noise) // Return a new polynomial with added noise
}

// Modular reduction using the prime modulus q
pub fn mod_reduce(poly: &Polynomial, modulus: i64) -> Polynomial {
    // Reduce each coefficient of the polynomial modulo the given modulus
    let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff % modulus).collect();
    // info!(
    //     "Performing modular reduction on polynomial {:?}. Result after mod reduction: {:?}",
    //     poly.coeffs, reduced
    // );
    Polynomial::new(reduced) // Return a new polynomial with reduced coefficients
}

// Modular reduction using the prime modulus q
pub fn mod_reduce_string(poly: &Polynomial, modulus: i64) -> Polynomial {
    // Reduce each coefficient of the polynomial modulo the given modulus
    let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff % modulus).collect();

    // Log the before and after state of the coefficients
    // info!(
    //     "Performing modular reduction on polynomial {:?}.",
    //     poly.coeffs
    // );
    // info!("Result after mod reduction: {:?}", reduced);

    // Filter out zero coefficients if necessary (optional)
    let filtered: Vec<i64> = reduced.into_iter().filter(|&coeff| coeff != 0).collect();
    // info!("Filtered coefficients (non-zero): {:?}", filtered);

    Polynomial::new(filtered) // Return a new polynomial with reduced coefficients
}

// pub fn encode_string(plaintext: &str, scaling_factor: f64) -> Polynomial {
//     if scaling_factor <= 0.0 {
//         panic!("Scaling factor must be positive");
//     }
//
//     // Convert each character to its ASCII/Unicode value and scale
//     let coeffs: Vec<i64> = plaintext.chars()
//         .map(|c| (c as i64 * scaling_factor as i64))  // Scale and collect coefficients
//         .collect();
//
//     info!("Encoded string '{}' as polynomial coefficients: {:?}", plaintext, coeffs);
//
//     Polynomial::new(coeffs)  // Return the polynomial with the encoded coefficients
// }
//
//
// // Decode polynomial back to a string
// pub fn decode_string(ciphertext: &Polynomial, scaling_factor: f64) -> String {
//     if scaling_factor <= 0.0 {
//         panic!("Scaling factor must be positive");
//     }
//
//     // Reverse the scaling factor and convert each coefficient back to its character representation
//     let decoded_chars: String = ciphertext.coeffs.iter()
//         .map(|&c| {
//             let value = c as f64 / scaling_factor;
//             // Convert the value back to a character
//             value.round() as u8 as char
//         })
//         .collect();
//
//     info!("Decoded polynomial {:?} back to string: '{}'", ciphertext.coeffs, decoded_chars);
//
//     decoded_chars  // Return the decoded string
// }

pub fn encode_string(plaintext: &str, scaling_factor: f64) -> Polynomial {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");
    }

    // Convert each character to its Unicode code point and scale it
    let coeffs: Vec<i64> = plaintext
        .chars()
        .map(|c| {
            let unicode_val = c as u32; // Get Unicode code point
            let scaled_val = (unicode_val as f64 * scaling_factor).round(); // Scale and round
            scaled_val as i64 // Convert to i64 for polynomial storage
        })
        .collect();

    // info!(
    //     "Encoded string '{}' as polynomial coefficients: {:?}",
    //     plaintext, coeffs
    // );

    Polynomial::new(coeffs) // Return the polynomial with encoded coefficients
}

// Decode polynomial back to a string
pub fn decode_string(ciphertext: &Polynomial, scaling_factor: f64) -> String {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");
    }

    // Reverse scaling and convert coefficients back to Unicode characters
    let decoded_chars: String = ciphertext
        .coeffs
        .iter()
        .map(|&c| {
            let scaled_val = c as f64 / scaling_factor; // Reverse scaling
            let unicode_val = scaled_val.round() as u32; // Convert back to Unicode value
            core::char::from_u32(unicode_val).unwrap_or('?') // Map to character or '?' if invalid
        })
        .collect();

    // info!(
    //     "Decoded polynomial {:?} back to string: '{}'",
    //     ciphertext.coeffs, decoded_chars
    // );

    decoded_chars // Return the decoded string
}
