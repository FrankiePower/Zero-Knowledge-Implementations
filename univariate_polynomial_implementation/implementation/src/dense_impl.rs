use ark_ff::PrimeField;

#[derive(Debug)]
pub struct DenseUnivariatePoly<F: PrimeField> {
    coefficients: Vec<F>,
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

    pub fn lagrange_interpolate(x_values: &Vec<F>, y_values: &Vec<F>) -> DenseUnivariatePoly<F> {
        let mut final_interpolated_polynomial = vec![F::zero()];

        for (index, x_value) in x_values.iter().enumerate() {
            let current_polynomial = lagrange_basis(y_values[index], *x_value, x_values.clone());

            final_interpolated_polynomial =
                add_polynomials(final_interpolated_polynomial, current_polynomial)
        }

        DenseUnivariatePoly {
            coefficients: final_interpolated_polynomial,
        }
    }
}

fn lagrange_basis<F: PrimeField>(
    y_point: F,
    focus_x_point: F,
    interpolating_set: Vec<F>,
) -> Vec<F> {
    let mut numerator = vec![F::one()];

    for x in interpolating_set.iter() {
        if *x != focus_x_point {
            numerator = multiply_polynomials(numerator, vec![-*x, F::one()]);
        }
    }

    let univariate_poly: DenseUnivariatePoly<F> = DenseUnivariatePoly::new(numerator.clone());

    let denominator = univariate_poly.evaluate(focus_x_point);

    let interpolated_polynomial = scalar_mul(y_point / denominator, numerator);

    interpolated_polynomial
}

fn scalar_mul<F: PrimeField>(scalar: F, polynomial: Vec<F>) -> Vec<F> {
    let mut result_polynomial: Vec<F> = Vec::new();

    for coeff in polynomial.iter() {
        result_polynomial.push(scalar * coeff);
    }

    result_polynomial
}

pub fn multiply_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let mut polynomial_product = vec![F::zero(); (left.len() + right.len()) - 1];

    for (left_index, left_coeff) in left.iter().enumerate() {
        for (right_index, right_coeff) in right.iter().enumerate() {
            polynomial_product[left_index + right_index] += *left_coeff * *right_coeff;
        }
    }

    polynomial_product
}

pub fn add_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let mut summed_polynomial: Vec<F> = Vec::new();

    let (larger_polynomial, smaller_polynomial) = if left.len() > right.len() {
        (left, right)
    } else {
        (right, left)
    };

    for (exp, coeff) in larger_polynomial.iter().enumerate() {
        if exp < smaller_polynomial.len() {
            summed_polynomial.push(*coeff + smaller_polynomial[exp]);
        } else {
            summed_polynomial.push(*coeff);
        }
    }

    summed_polynomial
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    fn test_setup() -> DenseUnivariatePoly<Fq> {
        let set_of_points = vec![Fq::from(7), Fq::from(5), Fq::from(2), Fq::from(1)];
        let polynomial = DenseUnivariatePoly::new(set_of_points.clone());

        polynomial
    }

    #[test]
    fn test_degree() {
        let polynomial = test_setup();
        assert_eq!(polynomial.degree(), 3);
    }

    #[test]
    fn test_evaluation() {
        let polynomial = test_setup();
        let evaluation_value = Fq::from(3);

        assert_eq!(polynomial.evaluate(evaluation_value), Fq::from(67));
    }

    #[test]
    fn test_add_polynomials() {
        let p1 = vec![Fq::from(5), Fq::from(2), Fq::from(5)];
        let p2 = vec![Fq::from(2), Fq::from(1), Fq::from(8), Fq::from(10)];

        assert_eq!(
            add_polynomials(p1, p2),
            vec![Fq::from(7), Fq::from(3), Fq::from(13), Fq::from(10)]
        );
    }

    #[test]
    fn test_multiply_polynomials() {
        let p1 = vec![Fq::from(5), Fq::from(0), Fq::from(2)];
        let p2 = vec![Fq::from(6), Fq::from(2)];

        assert_eq!(
            multiply_polynomials(p1, p2),
            vec![Fq::from(30), Fq::from(10), Fq::from(12), Fq::from(4)]
        );
    }

    #[test]
    fn test_lagrange_interpolate() {
        let x_values = vec![Fq::from(1), Fq::from(2), Fq::from(3)];
        let y_values = vec![Fq::from(6), Fq::from(11), Fq::from(18)];

        assert_eq!(
            DenseUnivariatePoly::lagrange_interpolate(&x_values, &y_values).coefficients,
            vec![Fq::from(3), Fq::from(2), Fq::from(1)]
        );
    }
}
