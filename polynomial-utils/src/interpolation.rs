use crate::operations::{add_polynomials, multiply_polynomials, scalar_mul};
use crate::polynomial::DenseUnivariatePoly;
use ark_ff::PrimeField;

pub fn lagrange_interpolate<F: PrimeField>(
    x_values: &[F],
    y_values: &[F],
) -> DenseUnivariatePoly<F> {
    let mut final_poly = vec![F::zero()];

    for (i, &x_val) in x_values.iter().enumerate() {
        let current_poly = lagrange_basis(y_values[i], x_val, x_values);
        final_poly = add_polynomials(final_poly, current_poly);
    }

    DenseUnivariatePoly {
        coefficients: final_poly,
    }
}

fn lagrange_basis<F: PrimeField>(y: F, x0: F, x_values: &[F]) -> Vec<F> {
    let mut numerator = vec![F::one()];

    for &x in x_values.iter() {
        if x != x0 {
            numerator = multiply_polynomials(numerator, vec![-x, F::one()]);
        }
    }

    let denominator = DenseUnivariatePoly::new(numerator.clone()).evaluate(x0);
    scalar_mul(y / denominator, numerator)
}
