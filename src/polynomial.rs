#[derive(Debug)]
pub struct Polynomial {
    pub coeffs: Vec<i64>,  // Coefficients for the polynomial
}

impl Polynomial {
    pub fn new(coeffs: Vec<i64>) -> Self {
        Polynomial { coeffs }
    }

    // Polynomial addition
    pub fn add(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];

        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 };
            result[i] = a + b;
        }

        Polynomial::new(result)
    }

    // Polynomial subtraction
    pub fn subtract(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];

        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 };
            result[i] = a - b;
        }

        Polynomial::new(result)
    }

    pub fn multiply(&self,other: &Polynomial) -> Polynomial {
        let result_size = self.coeffs.len().max(other.coeffs.len());
        let mut result_coeffs = vec![0.0; result_size]; // Use f64 for coefficients
    
        // Multiply matching coefficients
        let min_len = self.coeffs.len().min(other.coeffs.len());
        for i in 0..min_len {
            result_coeffs[i] = (self.coeffs[i] as f64 * other.coeffs[i] as f64) / 10000000000000.0; // Scale and store
        }
        Polynomial::new(result_coeffs.iter().map(|&x| x.round() as i64).collect())
    }

    pub fn negation(&self) -> Polynomial {
        let negated_coeffs: Vec<f64> = self.coeffs.iter().map(|&c| -(c as f64)).collect();
        Polynomial::new(negated_coeffs.iter().map(|&x| x.round() as i64).collect())
    }
}
