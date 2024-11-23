use ckks_engine::*;
use approx::AbsDiffEq;

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
    

