use ark_ff::PrimeField;

pub fn scalar_mul<F: PrimeField>(scalar: F, poly: Vec<F>) -> Vec<F> {
    poly.iter().map(|coeff| *coeff * scalar).collect()
}

pub fn multiply_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let mut product = vec![F::zero(); left.len() + right.len() - 1];

    for (i, &left_coeff) in left.iter().enumerate() {
        for (j, &right_coeff) in right.iter().enumerate() {
            product[i + j] += left_coeff * right_coeff;
        }
    }

    product
}

pub fn add_polynomials<F: PrimeField>(left: Vec<F>, right: Vec<F>) -> Vec<F> {
    let (larger, smaller) = if left.len() > right.len() {
        (left, right)
    } else {
        (right, left)
    };
    larger
        .iter()
        .enumerate()
        .map(|(i, &coeff)| coeff + *smaller.get(i).unwrap_or(&F::zero()))
        .collect()
}
