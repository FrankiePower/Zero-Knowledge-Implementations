use ark_ff::PrimeField;
use std::collections::HashMap;

// Sparse Representation: Only stores the non-zero terms
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SparseUnivariatePoly<F: PrimeField> {
    coefficients: HashMap<usize, F>, // degree -> coefficient
}

impl<F: PrimeField> SparseUnivariatePoly<F> {
    pub fn new() -> Self {
        Self {
            coefficients: HashMap::new(),
        }
    }

    pub fn degree(&self) -> Option<usize> {
        self.coefficients.keys().max().copied()
    }

    pub fn insert_term(&mut self, degree: usize, coeff: F) {
        if coeff.is_zero() {
            self.coefficients.remove(&degree);
        } else {
            self.coefficients.insert(degree, coeff);
        }
    }
}
