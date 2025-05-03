// CHANGED(MUST_CHANGE): switched to using mock rng to avoid std dependency
use rand::{self, Rng}; // Importing random number generator traits

use serde::{Deserialize, Serialize};

// CHANGED: added Vec for no_std
use alloc::vec::Vec;

pub const KEY_LENGTH: usize = 2048; // Length of public & private key polynomials

// Struct to represent the public key containing two polynomials
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "postcard_max_size",
    derive(postcard::experimental::max_size::MaxSize)
)]
pub struct PublicKey {
    #[serde(with = "serde_arrays")]
    pub pk_0: [i64; KEY_LENGTH], // First polynomial of the public key
    #[serde(with = "serde_arrays")]
    pub pk_1: [i64; KEY_LENGTH], // Second polynomial of the public key
}

// Struct to represent the secret key containing a polynomial
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(
    feature = "postcard_max_size",
    derive(postcard::experimental::max_size::MaxSize)
)]
pub struct SecretKey {
    #[serde(with = "serde_arrays")]
    pub poly: [i64; KEY_LENGTH], // Polynomial of the secret key
}

// Struct for key generation
pub struct KeyGenerator;

impl KeyGenerator {
    // Constructor to create a new KeyGenerator
    pub fn new() -> Self {
        KeyGenerator
    }

    /// Generates a pair of public and secret keys using the specified random number generator (thing that implements [`Rng`]).
    pub fn generate_keys<R: Rng>(&self, rng: &mut R) -> (PublicKey, SecretKey) {
        // let mut rng = rand::rngs::mock::StepRng::new(0, 1); // Create a new random number generator using the mock_rng crate

        // // Generate secret key (random polynomial of size 2048)
        // let sec_key_poly: [i64; KEY_LENGTH] = (0..KEY_LENGTH)
        //     .map(|_| rng.gen_range(1..100))
        //     // .collect::<Vec<i64>>()
        //     .try_into()
        //     .unwrap();
        let mut sec_key_poly: [i64; KEY_LENGTH] = [0; KEY_LENGTH];
        for i in 0..KEY_LENGTH {
            sec_key_poly[i] = rng.gen_range(1..100);
        }

        let sec_key = SecretKey { poly: sec_key_poly }; // Create secret key using the generated polynomial

        // Generate a random polynomial for public key generation
        let random_poly: [i64; KEY_LENGTH] = (0..2048)
            .map(|_| rng.gen_range(1..100))
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();

        // Create public key polynomials
        let pk_0: [i64; KEY_LENGTH] = sec_key_poly
            .iter()
            .zip(&random_poly)
            .map(|(&sk, &r)| -sk * r + rng.gen_range(-10..10)) // Compute pk_0 as -sk * random + noise
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        let pk_1: [i64; KEY_LENGTH] = random_poly; // Set pk_1 to the random polynomial

        // Create public key with the generated polynomials
        let pub_key = PublicKey { pk_0, pk_1 };

        (pub_key, sec_key) // Return the generated public and secret keys
    }
}
