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
    pub fn sub(&self, other: &Polynomial) -> Polynomial {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut result = vec![0; len];

        for i in 0..len {
            let a = if i < self.coeffs.len() { self.coeffs[i] } else { 0 };
            let b = if i < other.coeffs.len() { other.coeffs[i] } else { 0 };
            result[i] = a - b;
        }

        Polynomial::new(result)
    }

}
