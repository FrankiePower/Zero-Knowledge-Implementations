use ark_ff::PrimeField;

#[derive(Debug, Clone)]
pub struct DenseUnivariatePoly<F: PrimeField> {
    pub coefficients: Vec<F>,
}

impl<F: PrimeField> DenseUnivariatePoly<F> {
    pub fn new(coeffs: Vec<F>) -> Self {
        Self {
            coefficients: coeffs,
        }
    }

    pub fn degree(&self) -> u32 {
        self.coefficients.len() as u32 - 1
    }

    pub fn evaluate(&self, value: F) -> F {
        let mut result = F::zero();
        for (index, coeff) in self.coefficients.iter().enumerate() {
            result += *coeff * value.pow([index as u64]);
        }
        result
    }
}
