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

    pub fn evaluate(&self, value: F) -> F {
        let mut result = F::zero();
        for (degree, coeff) in &self.coefficients {
            result += value.pow(&[*degree as u64]) * *coeff;
        }
        result
    }

    pub fn basis(i: usize, points: &[(F, F)]) -> Self {
        let mut basis_poly = Self::new();
        basis_poly.insert_term(0, F::one());
        let x_i = points[i].0;

        for (j, (x_j, _)) in points.iter().enumerate() {
            if i == j {
                continue;
            }

            // create (x - x_j) term

            let mut term = Self::new();
            term.insert_term(1, F::one()); // x term
            term.insert_term(0, -(*x_j)); // (x - x_j)

            let denominator = (x_i - x_j).inverse().expect("points should be distinct");

            basis_poly = basis_poly.mul_scalar(denominator);

            basis_poly = basis_poly.mul(&term);
        }

        basis_poly
    }

    pub fn mul_scalar(&self, scalar: F) -> Self {
        let mut result = self.clone();
        for coeff in result.coefficients.values_mut() {
            *coeff *= scalar;
        }
        result
    }

    /// ✅ **Multiplication of Two Polynomials**
    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::new();
        for (&d1, &c1) in &self.coefficients {
            for (&d2, &c2) in &other.coefficients {
                result.insert_term(d1 + d2, c1 * c2);
            }
        }
        result
    }

    /// ✅ **Addition of Two Polynomials**
    pub fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (&d, &c) in &other.coefficients {
            let new_coeff = result.coefficients.entry(d).or_insert(F::zero());
            *new_coeff += c;
        }
        result
    }

    /// ✅ **Interpolate Using Lagrange Basis**
    pub fn interpolate(points: &[(F, F)]) -> Self {
        let mut poly = Self::new();
        for (i, (_, y_i)) in points.iter().enumerate() {
            let basis_poly = Self::basis(i, points);
            poly = poly.add(&basis_poly.mul_scalar(*y_i));
        }
        poly
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::AdditiveGroup; // Example finite field (BN254 curve)

    #[test]
    fn test_new() {
        let poly: SparseUnivariatePoly<Fr> = SparseUnivariatePoly::new();
        assert!(poly.coefficients.is_empty());
    }

    #[test]
    fn test_insert_term() {
        let mut poly = SparseUnivariatePoly::new();
        let coeff = Fr::from(3u32);

        poly.insert_term(2, coeff);
        assert_eq!(poly.coefficients.get(&2), Some(&coeff));

        // Inserting zero should remove the term
        poly.insert_term(2, Fr::ZERO);
        assert!(!poly.coefficients.contains_key(&2));
    }

    #[test]
    fn test_evaluate() {
        let mut poly = SparseUnivariatePoly::new();
        poly.insert_term(0, Fr::from(5u32)); // f(x) = 5
        poly.insert_term(1, Fr::from(3u32)); // f(x) = 3x + 5

        assert_eq!(poly.evaluate(Fr::ZERO), Fr::from(5u32)); // f(0) = 5
        assert_eq!(poly.evaluate(Fr::from(2u32)), Fr::from(11u32)); // f(2) = 3(2) + 5 = 11
    }

    #[test]
    fn test_scalar_multiplication() {
        let mut poly = SparseUnivariatePoly::new();
        poly.insert_term(1, Fr::from(2u32));

        let result = poly.mul_scalar(Fr::from(3u32));
        assert_eq!(result.coefficients.get(&1), Some(&Fr::from(6u32)));
    }

    #[test]
    fn test_polynomial_multiplication() {
        let mut p1 = SparseUnivariatePoly::new();
        let mut p2 = SparseUnivariatePoly::new();

        p1.insert_term(1, Fr::from(2u32)); // 2x
        p2.insert_term(0, Fr::from(3u32)); // 3

        let result = p1.mul(&p2); // Expected: (2x) * 3 = 6x
        assert_eq!(result.coefficients.get(&1), Some(&Fr::from(6u32)));
    }

    #[test]
    fn test_polynomial_addition() {
        let mut p1 = SparseUnivariatePoly::new();
        let mut p2 = SparseUnivariatePoly::new();

        p1.insert_term(1, Fr::from(2u32)); // 2x
        p2.insert_term(1, Fr::from(3u32)); // 3x

        let result = p1.add(&p2); // Expected: (2x + 3x) = 5x
        assert_eq!(result.coefficients.get(&1), Some(&Fr::from(5u32)));
    }

    #[test]
    fn test_lagrange_basis() {
        let points = vec![
            (Fr::from(1u32), Fr::from(2u32)),
            (Fr::from(2u32), Fr::from(3u32)),
        ];
        let basis_poly = SparseUnivariatePoly::basis(0, &points);
        // Verify L_0(1) = 1 and L_0(2) = 0
        assert_eq!(basis_poly.evaluate(Fr::from(1u32)), Fr::from(1u32));
        assert_eq!(basis_poly.evaluate(Fr::from(2u32)), Fr::ZERO);
    }

    #[test]
    fn test_interpolation() {
        let points = vec![
            (Fr::from(1u32), Fr::from(2u32)),
            (Fr::from(2u32), Fr::from(3u32)),
        ];
        let poly = SparseUnivariatePoly::interpolate(&points);

        // Should match given points
        assert_eq!(poly.evaluate(Fr::from(1u32)), Fr::from(2u32));
        assert_eq!(poly.evaluate(Fr::from(2u32)), Fr::from(3u32));
    }
}
