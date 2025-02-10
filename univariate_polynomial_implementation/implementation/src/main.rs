use std::collections::HashMap;
use std::io;
use std::ops::{Add, Mul};

// Sparse Representation: Only stores the non-zero terms
struct SparsePolynomial<T> {
    terms: HashMap<usize, T>, // degree -> coefficient
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Default + From<u8>> SparsePolynomial<T> {
    fn evaluate(&self, x: T) -> T {
        let mut result = T::default();

        for (degree, coefficient) in &self.terms {
            result = result + *coefficient * pow(x, *degree);
        }

        result
    }
}

fn pow<T: Copy + Mul<Output = T> + From<u8>>(base: T, exp: usize) -> T {
    let mut result = T::from(1);
    for _ in 0..exp {
        result = result * base;
    }
    result
}

// // Dense Representation: Stores all coefficients, including zeros
struct DensePolynomial<T> {
    coefficients: Vec<T>,
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Default + From<u8>> DensePolynomial<T> {
    fn evaluate(&self, x: T) -> T {
        self.coefficients
            .iter()
            .enumerate()
            .fold(T::default(), |acc, (i, &coeff)| {
                acc + (coeff * pow(x, i)) // Correct order
            })
    }
}

fn main() {
    // Example: 3x^2 + 2x + 5 in both representations
    let sparse_poly = SparsePolynomial {
        terms: [(2, 3), (1, 2), (0, 5)].iter().cloned().collect(),
    };

    let dense_poly = DensePolynomial {
        coefficients: vec![5, 2, 3],
    };

    println!("Please input the value of x");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let x: usize = input.trim().parse().unwrap();

    println!("Sparse polynomial evaluates to:{}", sparse_poly.evaluate(x));
    println!("Dense polynomial evaluates to:{}", dense_poly.evaluate(x));
}
