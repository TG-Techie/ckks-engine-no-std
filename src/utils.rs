use crate::polynomial::Polynomial;
use rand::Rng;

pub const SCALE: f64 = 1e5;  // Use a fixed scaling factor

// Encode real numbers into polynomial form with scaling
pub fn encode_float(plaintext: &[f64], scaling_factor: f64) -> Polynomial {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");
    }
    // Print the input plaintext and scaling factor
    println!("Encoding real numbers {:?} with scaling factor {}", plaintext, scaling_factor);
    
    let coeffs: Vec<i64> = plaintext.iter()
        .map(|&x| (x * scaling_factor).round() as i64)  // Scale the real numbers
        .collect();
    
    // Print the resulting polynomial coefficients
    println!("Encoded polynomial coefficients: {:?}", coeffs);
    
    Polynomial::new(coeffs)
}

pub fn encode_integers(plaintext: &[i64], scaling_factor: f64) -> Polynomial {
    // Print the input plaintext and scaling factor
    if plaintext.is_empty() {
        panic!("Input plaintext cannot be empty");
    }
    println!("Encoding integers {:?} with scaling factor {}", plaintext, scaling_factor);
    
    let coeffs: Vec<i64> = plaintext.iter()
        .map(|&x| (x as f64 * scaling_factor).round() as i64)  // Scale the integers
        .collect();
    
    // Print the resulting polynomial coefficients
    println!("Encoded polynomial coefficients: {:?}", coeffs);
    
    Polynomial::new(coeffs)
}

// Decode polynomial back into real numbers by removing the scaling factor
pub fn decode_float(ciphertext: &Polynomial, scaling_factor: f64, _is_multiplication: bool) -> Vec<f64> {
    // Print the input ciphertext and scaling factor
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");
    }
    println!("Decoding polynomial coefficients {:?} with scaling factor {}", ciphertext.coeffs, scaling_factor);

    // Perform decoding (reverse scaling)
    let decoded_values: Vec<f64> = ciphertext.coeffs.iter()
        .map(|&c| (c as f64) / scaling_factor)  // Reverse the scaling
        .collect();
    
    // Print the decoded real numbers
    println!("Decoded real numbers: {:?}", decoded_values);

    decoded_values
}

// Decode polynomial back into integers by removing the scaling factor
pub fn decode_integers(ciphertext: &Polynomial, scaling_factor: f64) -> Vec<i64> {
    if scaling_factor <= 0.0 {
        panic!("Scaling factor must be positive");
    }
    // Print the input ciphertext and scaling factor
    println!("Decoding polynomial coefficients {:?} with scaling factor {}", ciphertext.coeffs, scaling_factor);

    const TOLERANCE: f64 = 1e-10;  // Define a tolerance level for near-zero values

    // Perform decoding (reverse scaling) and convert to i64
    let decoded_values: Vec<i64> = ciphertext.coeffs.iter()
        .map(|&c| (c as f64) / scaling_factor)  // Reverse the scaling
        .map(|x| if x.abs() < TOLERANCE { 0.0 } else { x.round() })  // Set small values to zero and round
        .map(|x| x as i64)  // Convert to i64
        .collect();
    // Print the decoded integers
    println!("Decoded integers: {:?}", decoded_values);

    decoded_values
}


// Add noise to a polynomial
pub fn add_noise(poly: &Polynomial, _pub_key: &impl std::fmt::Debug) -> Polynomial {
    let mut rng = rand::thread_rng();
    let noise: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff + rng.gen_range(-10..10)).collect();
    println!("Adding noise to polynomial {:?}. Result after noise addition: {:?}", poly.coeffs, noise);
    Polynomial::new(noise)
}

// Modular reduction using the prime modulus q
pub fn mod_reduce(poly: &Polynomial, modulus: i64) -> Polynomial {
    let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff % modulus).collect();
    println!("Performing modular reduction on polynomial {:?}. Result after mod reduction: {:?}", poly.coeffs, reduced);
    Polynomial::new(reduced)
}

// pub fn mod_reduce_int(poly: &Polynomial, modulus: i64) -> Polynomial {
//     if modulus <= 0 {
//         panic!("Modulus must be a positive integer");
//     }

//     let reduced: Vec<i64> = poly.coeffs.iter().map(|&coeff| coeff.rem_euclid(modulus)).collect();
//     println!("Performing modular reduction on polynomial {:?}. Result after mod reduction: {:?}", poly.coeffs, reduced);
//     Polynomial::new(reduced)
// }
