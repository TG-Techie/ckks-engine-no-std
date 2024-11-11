#[derive(Debug, Clone)]
pub struct Polynomial {
    pub coeffs: Vec<i64>,  // Coefficients for the polynomial
}

impl Polynomial {
    // Constructor to create a new Polynomial with given coefficients
    pub fn new(coeffs: Vec<i64>) -> Self {
        Polynomial { coeffs }
    }

    //wrote this only for homomorphic truncate - we haven't used this anywhere apart from it
    pub fn decode(&self) -> Vec<i64> {
        self.coeffs.iter().map(|&c| {
            let real = (c as f64) / 10_000_000.0;
            real.round() as i64 // Round to the nearest integer
        }).collect()
    }

    // Polynomial addition
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        // Determine the maximum length of the two polynomials
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];  // Initialize result vector with zeros

        // Add coefficients of both polynomials
        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };  // Get coefficient from self or 0 if out of bounds
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 }; // Get coefficient from other or 0 if out of bounds
            result[i] = a + b;  // Add coefficients
        }

        Polynomial::new(result)  // Return new polynomial as the result
    }

    // Polynomial subtraction
    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        // Determine the maximum length of the two polynomials
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];  // Initialize result vector with zeros

        // Subtract coefficients of the second polynomial from the first
        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };  // Get coefficient from self or 0 if out of bounds
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 }; // Get coefficient from other or 0 if out of bounds
            result[i] = a - b;  // Subtract coefficients
        }

        Polynomial::new(result)  // Return new polynomial as the result
    }

    // Polynomial multiplication
    pub fn multiply(&self, other: &Polynomial) -> Polynomial {
        // Determine size for the resulting polynomial
        let result_size = self.coeffs.len().max(other.coeffs.len());
        let mut result_coeffs = vec![0.0; result_size]; // Initialize result coefficients with f64 for scaling

        // Multiply matching coefficients of both polynomials
        let min_len = self.coeffs.len().min(other.coeffs.len());
        for i in 0..min_len {
            result_coeffs[i] = (self.coeffs[i] as f64 * other.coeffs[i] as f64) / 1e7; // Scale and store the product
        }
        // Create a new polynomial with rounded coefficients
        Polynomial::new(result_coeffs.iter().map(|&x| x.round() as i64).collect())
    }
    // pub fn multiply_v2(&self, other: &Polynomial) -> Polynomial {
    //     // Determine size for the resulting polynomial
    //     let result_size = self.coeffs.len() + other.coeffs.len() - 1;
    //     let mut result_coeffs = vec![0.0; result_size]; // Initialize result coefficients with f64 for scaling
    //
    //     // Perform convolution (polynomial multiplication)
    //     for i in 0..self.coeffs.len() {
    //         for j in 0..other.coeffs.len() {
    //             result_coeffs[i + j] += (self.coeffs[i] as f64 * other.coeffs[j] as f64) / 1e7; // Scale and add
    //         }
    //     }
    //
    //     // Round the coefficients to the nearest integer
    //     let rounded_coeffs: Vec<i64> = result_coeffs.iter().map(|&x| x.round() as i64).collect();
    //
    //     Polynomial::new(rounded_coeffs)  // Return new polynomial as the result
    // }

    pub fn multiply_v2(&self, other: &Polynomial) -> Polynomial {
        let result_size = self.coeffs.len() + other.coeffs.len() - 1;
        let mut result_coeffs = vec![0; result_size];

        // Perform convolution
        for i in 0..self.coeffs.len() {
            for j in 0..other.coeffs.len() {
                result_coeffs[i + j] += self.coeffs[i] * other.coeffs[j];
            }
        }

        // Rescale by dividing by the scaling factor (s = 1e7)
        let s = 10_000_000;
        let scaled_result: Vec<i64> = result_coeffs.iter().map(|&c| (c + s / 2) / s).collect(); // Rounded division

        Polynomial::new(scaled_result)
    }




    // Polynomial negation
    pub fn negation(&self) -> Polynomial {
        // Negate each coefficient of the polynomial
        let negated_coeffs: Vec<f64> = self.coeffs.iter().map(|&c| -(c as f64)).collect();
        // Create a new polynomial with rounded negated coefficients
        Polynomial::new(negated_coeffs.iter().map(|&x| x.round() as i64).collect())
    }

    pub fn divide(&self, divisor: &Polynomial, scaling_factor: f64) -> Polynomial {
        let mut result_coeffs = Vec::with_capacity(self.coeffs.len());

        for (a, b) in self.coeffs.iter().zip(divisor.coeffs.iter()) {
            // Check for zero in the divisor to avoid division by zero
            if *b == 0 {
                panic!("Division by zero encountered in polynomial division");
            }

            // Perform the division and scaling
            let scaled_result = (*a as f64 / *b as f64) * scaling_factor;

            // Convert the scaled result to an integer and push it into the result coefficients
            result_coeffs.push(scaled_result.round() as i64);
        }

        // Return a new polynomial with the resulting coefficients after division
        Polynomial::new(result_coeffs)
    }
}
