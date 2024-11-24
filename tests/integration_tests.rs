use ckks_engine::*;
use approx::AbsDiffEq;
use crate::utils::{encode, decode, mod_reduce,decode_string,mod_reduce_string};

fn run_homomorphic_arithmetic_tests(poly1: &[f64], poly2: Option<&[f64]>, epsilon: f64, large_epsilon: f64) {
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();
    let params = CkksParameters::new(2048, 1000000000000007);
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());

    // Encrypt inputs
    let encrypted_poly1 = encryptor.encrypt_collection(poly1);
    let encrypted_poly2 = poly2.map(|p| encryptor.encrypt_collection(p));

    // Test addition
    if let Some(encrypted_poly2) = &encrypted_poly2 {
        let encrypted_add = encryptor.homomorphic_add(&encrypted_poly1, encrypted_poly2);
        let decrypted_add = decryptor.decrypt(&encrypted_add);
        let expected_add: Vec<f64> = poly1
            .iter()
            .zip(poly2.unwrap().iter())
            .map(|(&a, &b)| ((a + b) * 100.0).round() / 100.0)
            .collect();

        assert!(
            expected_add
                .iter()
                .zip(decrypted_add.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, epsilon)
                }),
            "Addition failed. Expected: {:?}, Decrypted: {:?}",
            expected_add,
            decrypted_add
        );
    }

    // Test subtraction
    if let Some(encrypted_poly2) = &encrypted_poly2 {
        let encrypted_sub = encryptor.homomorphic_subtract(&encrypted_poly1, encrypted_poly2);
        let decrypted_sub = decryptor.decrypt(&encrypted_sub);
        let expected_sub: Vec<f64> = poly1
            .iter()
            .zip(poly2.unwrap().iter())
            .map(|(&a, &b)| ((a - b) * 100.0).round() / 100.0)
            .collect();

        assert!(
            expected_sub
                .iter()
                .zip(decrypted_sub.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, epsilon)
                }),
            "Subtraction failed. Expected: {:?}, Decrypted: {:?}",
            expected_sub,
            decrypted_sub
        );
    }

    // Test multiplication
    if let Some(encrypted_poly2) = &encrypted_poly2 {
        let encrypted_mul = encryptor.homomorphic_multiply(&encrypted_poly1, encrypted_poly2);
        let decrypted_mul = decryptor.decrypt(&encrypted_mul);
        let expected_mul: Vec<f64> = poly1
            .iter()
            .zip(poly2.unwrap().iter())
            .map(|(&a, &b)| ((a * b) * 100.0).round() / 100.0)
            .collect();

        assert!(
            expected_mul
                .iter()
                .zip(decrypted_mul.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, large_epsilon)
                }),
            "Multiplication failed. Expected: {:?}, Decrypted: {:?}",
            expected_mul,
            decrypted_mul
        );
    }

    // Test negation
    if let Some(_) = Some(&encrypted_poly1) {
        let encrypted_neg = encryptor.homomorphic_negation(&encrypted_poly1);
        let decrypted_neg = decryptor.decrypt(&encrypted_neg);
        let expected_neg: Vec<f64> = poly1.iter().map(|&a| ((-a) * 100.0).round() / 100.0).collect();

        assert!(
            expected_neg
                .iter()
                .zip(decrypted_neg.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, epsilon)
                }),
            "Negation failed. Expected: {:?}, Decrypted: {:?}",
            expected_neg,
            decrypted_neg
        );
    }

    // Test exponentiation
    if let Some(_) = Some(&encrypted_poly1) {
        let exponent = 2;
        let encrypted_exp = encryptor.homomorphic_exponentiation(&encrypted_poly1, exponent);
        let decrypted_exp = decryptor.decrypt(&encrypted_exp);
        let expected_exp: Vec<f64> = poly1
            .iter()
            .map(|&a| ((a.powi(exponent as i32)) * 100.0).round() / 100.0)
            .collect();

        assert!(
            expected_exp
                .iter()
                .zip(decrypted_exp.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, large_epsilon)
                }),
            "Exponentiation failed. Expected: {:?}, Decrypted: {:?}",
            expected_exp,
            decrypted_exp
        );
    }

    // Test division
    if let Some(encrypted_poly2) = &encrypted_poly2 {
        let encrypted_div = encryptor.homomorphic_divide(&encrypted_poly1, encrypted_poly2);
        let decrypted_div = decryptor.decrypt(&encrypted_div);
        let expected_div: Vec<f64> = poly1
            .iter()
            .zip(poly2.unwrap().iter())
            .map(|(&a, &b)| ((a / b) * 100.0).round() / 100.0)
            .collect();

        assert!(
            expected_div
                .iter()
                .zip(decrypted_div.iter())
                .all(|(expected_val, decrypted_val)| {
                    decrypted_val.abs_diff_eq(expected_val, large_epsilon)
                }),
            "Division failed. Expected: {:?}, Decrypted: {:?}",
            expected_div,
            decrypted_div
        );
    }

}

#[test]
fn test_homomorphic_arithmetic_with_integers() {
    let poly1 = [10.0, 20.0, 30.0];
    let poly2 = [5.0, 15.0, 21.0];
    run_homomorphic_arithmetic_tests(&poly1, Some(&poly2), 0.9, 10.0);
}

#[test]
fn test_homomorphic_arithmetic_with_floats() {
    let poly1 = [1.1, 2.2, 3.3];
    let poly2 = [4.4, 5.5, 6.6];
    run_homomorphic_arithmetic_tests(&poly1, Some(&poly2), 0.9, 10.0);
}

#[test]
fn test_homomorphic_arithmetic_with_negatives() {
    let poly1 = [-10.0, -20.0, -30.0];
    let poly2 = [5.0, 15.0, 25.0];
    run_homomorphic_arithmetic_tests(&poly1, Some(&poly2), 0.9, 10.0);
}

#[test]
fn test_homomorphic_arithmetic_with_single_element_arrays() {
    let poly1 = [42.0];
    let poly2 = [58.0];
    run_homomorphic_arithmetic_tests(&poly1, Some(&poly2), 0.9, 10.0);
}

#[test]
fn test_floating_point_operations() {
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();
    let params = CkksParameters::new(2048, 1000000000000007);
    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());


    let float_array: [f64; 3] = [3.6, 5.26, 3.78];
    let encrypted_float_array = encryptor.encrypt_collection(&float_array);

    // Ceil
    let encrypted_ceil = encryptor.homomorphic_ceil(&encrypted_float_array);
    let decrypted_ceil = decryptor.decrypt(&encrypted_ceil);
    let expected_ceil: Vec<f64> = float_array.iter().map(|&a| a.ceil()).collect();
    assert_eq!(decrypted_ceil, expected_ceil, "Ceil operation failed.");

    // Floor
    let encrypted_floor = encryptor.homomorphic_floor(&encrypted_float_array);
    let decrypted_floor = decryptor.decrypt(&encrypted_floor);
    let expected_floor: Vec<f64> = float_array.iter().map(|&a| a.floor()).collect();
    assert_eq!(decrypted_floor, expected_floor, "Floor operation failed.");

    // Round
    let encrypted_round = encryptor.homomorphic_round(&encrypted_float_array);
    let decrypted_round = decryptor.decrypt(&encrypted_round);
    let expected_round: Vec<f64> = float_array.iter().map(|&a| a.round()).collect();
    assert_eq!(decrypted_round, expected_round, "Round operation failed.");

    // Truncate
    let encrypted_truncate = encryptor.homomorphic_truncate(&encrypted_float_array);
    let decrypted_truncate = decryptor.decrypt_as_int(&encrypted_truncate);
    let expected_truncate: Vec<i64> = float_array.iter().map(|&a| a as i64).collect();
    assert_eq!(
        decrypted_truncate, expected_truncate,
        "Truncate operation failed."
    );
}


fn initialize_ckks() -> (CKKSEncryptor, CKKSDecryptor) { //TO-DO remove redundant declarations in arithmetic tests and use this function
    let keygen = KeyGenerator::new();
    let (public_key, secret_key) = keygen.generate_keys();
    // let params = CkksParameters::new(2048, 1000000000000007);
    let params = CkksParameters::new(4096, 1000000000000007); // Larger modulus

    let encryptor = CKKSEncryptor::new(public_key.clone(), params.clone());
    let decryptor = CKKSDecryptor::new(secret_key.clone(), params.clone());
    (encryptor, decryptor)
}



fn encode_string(input: &str) -> Vec<f64> {
    input.chars().map(|c| c as u32 as f64).collect()
}

fn map_to_nearest_ascii(val: f64) -> char {
    let rounded_val = val.round() as u8;
    if rounded_val.is_ascii() {
        rounded_val as char
    } else {
        '?' // Fallback for non-ASCII values
    }
}

fn map_to_nearest_unicode(val: f64) -> char {
    let rounded_val = val.round() as u32;
    std::char::from_u32(rounded_val).unwrap_or('?') // Use '?' for invalid Unicode values


    // if let Some(character) = std::char::from_u32(rounded_val) {
    //     character
    // } else {
    //     '?' // Fallback for non-valid Unicode values
    // }

}


#[test]
fn test_homomorphic_length_calculation() {

    let (encryptor, _) = initialize_ckks();
    let original_string = "TestLength";
    let encrypted_string1 = encryptor.encrypt_string(original_string);
    let length = encryptor.homomorphic_length(&encrypted_string1);

    let expected_length = original_string.len();
    // println!("")
    assert_eq!(length, expected_length, "Homomorphic length calculation failed");
}

#[test]
fn test_homomorphic_concatenation() {
    let (encryptor, decryptor) = initialize_ckks();

    let string1 = "Hello Team!";
    let string2 = Some("World");
    let encrypted_data1 = encryptor.encrypt_string(string1);

    if let Some(inner_string2) = string2 {

        let encrypted_data2 = encryptor.encrypt_string(inner_string2);

        let concatenated_encrypted = encryptor.concatenate_encrypted_strings(&encrypted_data1, &encrypted_data2);
        let decrypted_data = decryptor.decrypt_string(&concatenated_encrypted);
        let expected_string = format!("{}{}", string1, inner_string2);
        assert_eq!(decrypted_data, expected_string, "Homomorphic concatenation failed");
    }
}



#[test]
fn test_homomorphic_substring_extraction() {
    let (encryptor, decryptor) = initialize_ckks();
    let original_string = "HelloWorld";
    let encrypted_string = encryptor.encrypt_string(&original_string);

    let substring_start = 5;
    let substring_end = 10;
    let extracted_encrypted = encryptor.extract_encrypted_substring(&encrypted_string, substring_start..substring_end);

    let decrypted_data = decryptor.decrypt_string(&extracted_encrypted);

    let expected_string = "World";
    // println!("decrypted data is : {}",decrypted_data);
    assert_eq!(decrypted_data, expected_string, "Substring extraction failed");
}

// #[test]
// fn test_noise_resilience() {
    // let (encryptor, decryptor) = initialize_ckks();
    // let original_string = "NoiseTest😀";
    // let encoded = encode_string(&original_string);
    //
    // // Introduce noise
    // let mut noisy_encoded: Vec<f64> = encoded.iter().map(|&x| x + 0.001).collect();
    // let encrypted_data = encryptor.encrypt_collection(&noisy_encoded);
    // let decrypted_data = decryptor.decrypt(&encrypted_data);
    //
    // let decrypted_string: String = decrypted_data
    //     .iter()
    //     .map(|&val| map_to_nearest_unicode(val))
    //     .collect();
    //
    // assert_eq!(decrypted_string, original_string, "Noise resilience test failed");
// }

fn validate_string_size(size: usize) -> bool {
    const MAX_ALLOWED_SIZE: usize = 10_000_000; // Define a reasonable maximum size
    if size > MAX_ALLOWED_SIZE {
        println!("Error: String size exceeds the allowed limit of {} bytes.", MAX_ALLOWED_SIZE);
        return false;
    }
    true
}

#[test]
fn test_large_string_handling() {
    let (encryptor, decryptor) = initialize_ckks(); // Replace with actual initialization logic

    // Use a test string size
    let desired_size = 100000; // Intentionally exceeding the limit for testing

    // Step 2: Validate the string size
    if !validate_string_size(desired_size) {
        println!("Test aborted due to size validation failure.");
        return; // Exit the function safely
    }

    // Step 3: Create the original string
    let original_string = "A".repeat(desired_size);

    // Encode and encrypt in chunks
    let chunk_size = 1000;
    let chunks: Vec<&[u8]> = original_string.as_bytes().chunks(chunk_size).collect();

    let mut encrypted_chunks = vec![];
    for chunk in chunks {
        // Convert &[u8] to &str
        let encoded_chunk = encode_string(std::str::from_utf8(chunk).unwrap()); // Replace with actual encoding logic
        let encrypted_chunk = encryptor.encrypt_collection(&encoded_chunk); // Encrypt
        encrypted_chunks.push(encrypted_chunk);
    }


    // Decrypt and reconstruct the string
    let mut decrypted_chunks = vec![];
    for encrypted_chunk in encrypted_chunks {
        let decrypted_chunk = decryptor.decrypt(&encrypted_chunk); // Decrypt
        let chunk_string: String = decrypted_chunk
            .iter()
            .map(|&val| map_to_nearest_unicode(val)) // Replace with actual mapping logic
            .collect();
        decrypted_chunks.push(chunk_string);
    }

    let reconstructed_string = decrypted_chunks.concat();

    // Assert the reconstructed string matches the original
    assert_eq!(
        reconstructed_string, original_string,
        "Large string handling failed"
    );
}